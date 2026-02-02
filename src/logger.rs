use chrono::Local;
use std::fs;
use std::io::Write;

pub fn log_message(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_msg = format!("[{}] {}", timestamp, msg);

    // Вывод в консоль
    println!("{}", log_msg);

    // Запись в файл
    if let Ok(mut file) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("ssh_tunnel_{}.log", Local::now().format("%Y-%m-%d")))
    {
        let _ = writeln!(file, "{}", log_msg);
    }
}

pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let msg = format!("[{}] PANIC: {:?}", timestamp, panic_info);

        println!("{}", msg);

        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("ssh_tunnel_error_{}.log", Local::now().format("%Y-%m-%d")))
        {
            let _ = writeln!(file, "{}", msg);
        }
    }));
}
