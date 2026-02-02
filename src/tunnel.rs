use anyhow::{Context, Result};
use ssh2::{Session, Channel};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

/// Хэндл для управления жизненным циклом туннеля
pub struct TunnelHandle {
    pub stop_tx: Sender<()>,
    pub join_handle: thread::JoinHandle<()>,
}

/// Конфигурация SSH-туннеля
#[derive(Debug, Clone)]
pub struct SshTunnel {
    pub server_ip: String,
    pub username: String,
    pub key_path: String,
    pub password: Option<String>,

    pub local_port: u16,
    pub remote_port: u16,
}

impl SshTunnel {
    /// Запуск SSH Local Port Forwarding:
    /// localhost:local_port -> server:remote_port
    pub fn start(&self) -> Result<TunnelHandle> {
        let (stop_tx, stop_rx) = mpsc::channel();
        let cfg = self.clone();

        let join_handle = thread::spawn(move || {
            if let Err(e) = run_tunnel(cfg, stop_rx) {
                eprintln!("SSH tunnel stopped with error: {e:?}");
            }
        });

        Ok(TunnelHandle {
            stop_tx,
            join_handle,
        })
    }
}

fn run_tunnel(cfg: SshTunnel, stop_rx: mpsc::Receiver<()>) -> Result<()> {
    // 1️⃣ TCP listener на локальном порту
    let listener = TcpListener::bind(("127.0.0.1", cfg.local_port))
        .with_context(|| format!("Bind local port {}", cfg.local_port))?;

    listener
        .set_nonblocking(true)
        .context("Set nonblocking listener")?;

    // 2️⃣ SSH TCP соединение
    let tcp = TcpStream::connect(format!("{}:22", cfg.server_ip))
        .with_context(|| "Connect to SSH server")?;

    let mut session = Session::new().context("Create SSH session")?;
    session.set_tcp_stream(tcp);
    session.handshake().context("SSH handshake")?;

    // 3️⃣ Аутентификация
    authenticate(&session, &cfg)?;

    if !session.authenticated() {
        anyhow::bail!("SSH authentication failed");
    }

    // 4️⃣ Основной loop
    loop {
        // graceful stop
        if stop_rx.try_recv().is_ok() {
            break;
        }

        match listener.accept() {
            Ok((local_stream, _)) => {
                let mut channel = session
                    .channel_direct_tcpip(
                        "127.0.0.1",
                        cfg.remote_port,
                        None,
                    )
                    .context("Open remote channel")?;

                pipe(local_stream, &mut channel)?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(50));
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

fn authenticate(session: &Session, cfg: &SshTunnel) -> Result<()> {
    let key_path = expand_tilde(&cfg.key_path);

    if let Some(pass) = &cfg.password {
        session
            .userauth_pubkey_file(
                &cfg.username,
                None,
                Path::new(&key_path),
                Some(pass),
            )
            .context("Auth with key + password")?;
    } else {
        session
            .userauth_pubkey_file(
                &cfg.username,
                None,
                Path::new(&key_path),
                None,
            )
            .context("Auth with key")?;
    }

    Ok(())
}

/// Проксирование данных local <-> remote
fn pipe(mut local: TcpStream, remote: &mut Channel) -> Result<()> {
    local.set_nonblocking(true)?;
    // remote.set_blocking(false);

    let mut buf = [0u8; 8192];

    loop {
        match local.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                remote.write_all(&buf[..n])?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e.into()),
        }

        match remote.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                local.write_all(&buf[..n])?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e.into()),
        }

        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}

/// Поддержка "~/.ssh/id_rsa"
fn expand_tilde(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped).to_string_lossy().into();
        }
    }
    path.to_string()
}
