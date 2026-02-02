use crate::config::{AppConfig, TunnelConfig};
use crate::tunnel::{TunnelState, TunnelWorker};
use slint::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Включаем сгенерированный код из .slint файла
slint::include_modules!();

// Обертка для потокобезопасного обновления модели
#[derive(Clone)]
struct TunnelModelWrapper {
    // События для обновления модели из других потоков
    // update_sender: slint::ComponentHandle<MainWindow>,
}

impl TunnelModelWrapper {
    fn new(ui: &MainWindow) -> Self {
        Self {
            //   update_sender: ui.as_weak().into(),
        }
    }

    fn update_tunnel_status(
        &self,
        name: String,
        status: String,
        color: String,
        start_enabled: bool,
        stop_enabled: bool,
    ) {
        todo!("Добавить обработку данных");
        // let handle = self.update_sender.clone();
        // slint::invoke_from_event_loop(move || {
        //   if let Some(ui) = handle.upgrade() {
        //     // Получаем текущую модель
        //     let mut tunnels = ui.get_tunnels();
        //     let mut updated = false;
        //
        //     // Ищем и обновляем туннель
        //     for i in 0..tunnels.len() {
        //       if tunnels[i].name == name {
        //         tunnels[i] = TunnelInfo {
        //           name: name.into(),
        //           local_port: tunnels[i].local_port,
        //           remote_port: tunnels[i].remote_port,
        //           status: status.into(),
        //           status_color: color.into(),
        //           start_enabled,
        //           stop_enabled,
        //         };
        //         updated = true;
        //         break;
        //       }
        //     }
        //
        //     if updated {
        //       ui.set_tunnels(tunnels);
        //     }
        //   }
        // })
        // .unwrap_or(());
    }
}

pub struct AppUI {
    pub ui: MainWindow,
    pub workers: HashMap<String, TunnelWorker>,
    pub config: AppConfig,
    pub model_wrapper: TunnelModelWrapper,
}

impl AppUI {
    pub fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let ui = MainWindow::new()?;

        // Инициализация туннелей
        // let mut tunnels = Vec::new();
        // for tunnel_config in &config.tunnels {
        //     tunnels.push(TunnelInfo {
        //         name: tunnel_config.name.clone().into(),
        //         local_port: tunnel_config.local_port as i32,
        //         remote_port: tunnel_config.remote_port as i32,
        //         status: "🔴 Отключен".into(),
        //         status_color: "#ff0000".into(),
        //         start_enabled: true,
        //         stop_enabled: false,
        //     });
        // }

        // Установка туннелей в UI
        // ui.set_tunnels(tunnels.into());
        // ui.set_log_text("Лог приложения:\n".into());
        // ui.set_password("".into());

        // Создание workers
        // let mut workers = HashMap::new();
        // for tunnel in &config.tunnels {
        //     workers.insert(tunnel.name.clone(), TunnelWorker::new(tunnel.name.clone()));
        // }

        // let model_wrapper = TunnelModelWrapper::new(&ui);

        Ok(Self {
            ui,
            // workers,
            // config,
            // model_wrapper,
        })
    }

    // pub fn setup_callbacks(&mut self) {
    //     let ui_weak = self.ui.as_weak();
    //     let workers_clone = self.workers.clone();
    //     let config_clone = self.config.clone();
    //     let model_wrapper = self.model_wrapper.clone();
    //
    //     self.ui.on_start_tunnel({
    //         let ui_weak = ui_weak.clone();
    //         let workers_clone = workers_clone.clone();
    //         let config_clone = config_clone.clone();
    //         let model_wrapper = model_wrapper.clone();
    //
    //         move |name| {
    //             let name_str = name.to_string();
    //             if let Some(worker) = workers_clone.get(&name_str) {
    //                 if let Some(ui) = ui_weak.upgrade() {
    //                     let password = ui.get_password().to_string();
    //
    //                     if let Some(tunnel_conf) = config_clone.find_tunnel(&name_str) {
    //                         // Сразу обновляем статус в UI
    //                         model_wrapper.update_tunnel_status(
    //                             name_str.clone(),
    //                             "🟡 Переподключение...".to_string(),
    //                             "#ffa500".to_string(),
    //                             false,
    //                             true,
    //                         );
    //
    //                         worker.start(&config_clone, tunnel_conf, &password);
    //                     }
    //                 }
    //             }
    //         }
    //     });
    //
    //     let ui_weak = self.ui.as_weak();
    //     let workers_clone = self.workers.clone();
    //     let model_wrapper = self.model_wrapper.clone();
    //
    //     self.ui.on_stop_tunnel(move |name| {
    //         let name_str = name.to_string();
    //         if let Some(worker) = workers_clone.get(&name_str) {
    //             worker.stop();
    //
    //             // Обновляем статус в UI
    //             model_wrapper.update_tunnel_status(
    //                 name_str,
    //                 "🔴 Отключен".to_string(),
    //                 "#ff0000".to_string(),
    //                 true,
    //                 false,
    //             );
    //         }
    //     });
    //
    //     let ui_weak = self.ui.as_weak();
    //     let workers_clone = self.workers.clone();
    //
    //     self.ui.on_quit_app(move || {
    //         // Остановка всех туннелей
    //         for worker in workers_clone.values() {
    //             worker.stop();
    //         }
    //
    //         if let Some(ui) = ui_weak.upgrade() {
    //             ui.hide().unwrap();
    //             slint::quit_event_loop().unwrap();
    //         }
    //     });
    // }

    // pub fn start_status_updater(&self) {
    //     let workers_clone = self.workers.clone();
    //     let model_wrapper = self.model_wrapper.clone();
    //
    //     thread::spawn(move || {
    //         loop {
    //             thread::sleep(Duration::from_millis(500));
    //
    //             // Проверяем статусы всех туннелей
    //             for (name, worker) in &workers_clone {
    //                 match worker.get_state() {
    //                     TunnelState::Active => {
    //                         model_wrapper.update_tunnel_status(
    //                             name.clone(),
    //                             "🟢 Активен".to_string(),
    //                             "#00ff00".to_string(),
    //                             false,
    //                             true,
    //                         );
    //                     }
    //                     TunnelState::Reconnecting => {
    //                         model_wrapper.update_tunnel_status(
    //                             name.clone(),
    //                             "🟡 Переподключение...".to_string(),
    //                             "#ffa500".to_string(),
    //                             false,
    //                             true,
    //                         );
    //                     }
    //                     TunnelState::Error(err) => {
    //                         todo!("Добавить обработку данных");
    //                         // model_wrapper.update_tunnel_status(
    //                         //   name.clone(),
    //                         //   // format!("🔴 Ошибка: {}", err),
    //                         //   "#ff0000".to_string(),
    //                         //   true,
    //                         //   false,
    //                         // );
    //                     }
    //                     TunnelState::Stopped => {
    //                         // Только если еще не отображается как остановленный
    //                         // Можно добавить логику для проверки текущего статуса
    //                     }
    //                 }
    //             }
    //
    //             // Небольшая пауза
    //             thread::sleep(Duration::from_millis(100));
    //         }
    //     });
    // }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }
}
