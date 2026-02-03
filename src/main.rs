// main.rs

mod config;
mod logger;

use crate::config::AppConfig;
use crate::logger::{log_message, setup_panic_hook};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Настройка обработки паник
    setup_panic_hook();

    log_message("Запуск Clear folders...");

    // Загрузка конфигурации
    let config = AppConfig::load_from_file("config.json").unwrap_or_else(|e| {
        eprintln!("Ошибка загрузки конфигурации: {}", e);
        eprintln!("Создание конфигурации по умолчанию...");

        // Создание конфигурации по умолчанию
        let default_config = AppConfig {
            folders: vec![config::ConfigFolders { name: "Disk C:/Temp".to_string(), path: "C:/Temp".to_string() }],
        };

        // Сохранение конфигурации по умолчанию
        if let Err(e) = default_config.save_to_file("config.json") {
            eprintln!("Не удалось сохранить конфигурацию по умолчанию: {}", e);
        }
        default_config
    });
    println!("Загруженная конфигурация:\n{:#?}", config);

    let app = AppWin::new()?;

    app.on_file_new(|| {
        println!("New file");
    });

    app.on_file_open(|| {
        println!("Open file");
    });

    // app.on_about(|| {
    //     println!("About dialog");
    // });

    app.on_quit(|| {
        std::process::exit(0);
    });

    app.run()
}
