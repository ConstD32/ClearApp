// main.rs

mod config;
mod logger;
mod model;
mod services;

slint::include_modules!();
use slint::{ModelRc, VecModel};

use crate::config::AppConfig;
use crate::logger::{log_message, setup_panic_hook};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use std::{fs, io};

fn main() -> Result<(), slint::PlatformError> {
    // Настройка обработки паник
    setup_panic_hook();
    log_message("Запуск Clear folders...");

    let ui = AppWin::new()?;
    // let ui_weak = ui.as_weak();

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
    // println!("Загруженная конфигурация:\n{:#?}", config);

    // Передача списка в окно
    let folders: Vec<FolderItem> = config
        .folders
        .iter()
        .map(|t| FolderItem { name: t.name.clone().into(), path: t.path.clone().into() })
        .collect();

    let model = Rc::new(VecModel::from(folders.clone()));
    ui.set_folders(ModelRc::from(model));

    ui.on_clear_folder({
        // let ui_weak = ui_weak.clone();
        let cfg = config.clone();
        // let manager = folders.clone();
        move |index| {
            // ⬇️ безопасно получаем UI
            // let ui = match ui_weak.upgrade() {
            //     Some(ui) => ui,
            //     None => return, // окно уже закрыто
            // };

            let folders = match cfg.folders.get(index as usize) {
                Some(t) => t,
                None => return,
            };

            // println!("{:#?}", folders.path.to_string());

            let path = PathBuf::from(&folders.path);

            if let Err(e) = clean_folder(&path) {
                eprintln!("Ошибка очистки: {}", e);
            }

            // let folder = ConfigFolders {
            //     name: cfg.folders.clone(),
            //     path: cfg.folders.clone(),
            // };

            // match folder.clear() {
            //     Ok(handle) => {
            //         manager.tunnels.lock().unwrap().insert(index as usize, handle);
            //         println!("Туннель {} запущен", tunnel_cfg.name);
            //     }
            //     Err(e) => {
            //         eprintln!("Ошибка запуска туннеля: {e}");
            //     }
            // }
        }
    });

    ui.on_file_new(|| {
        println!("New file");
    });

    ui.on_file_open(|| {
        println!("Open file");
    });

    ui.on_quit(|| {
        std::process::exit(0);
    });

    ui.run()
}

pub fn clean_folder(path: &Path) -> io::Result<()> {
    log_message(&format!("Cleaning folder: {}", path.display()));

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Folder not found: {}", path.display())));
    }

    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Not a directory: {}", path.display())));
    }

    if is_dangerous(path) {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Refusing to clean root directory"));
    }

    let mut files_removed = 0usize;
    let mut dirs_removed = 0usize;
    let mut errors = 0usize;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();

        let result = if p.is_dir() { fs::remove_dir_all(&p) } else { fs::remove_file(&p) };

        match result {
            Ok(_) => {
                if p.is_dir() {
                    dirs_removed += 1;
                } else {
                    files_removed += 1;
                }
            }
            Err(_) => {
                errors += 1;
            }
        }
    }

    log_message(&format!(
        "Cleaned: {} | files={} dirs={} errors={}",
        path.display(),
        files_removed,
        dirs_removed,
        errors
    ));

    Ok(())
}

fn is_dangerous(path: &Path) -> bool {
    path.parent().is_none() // C:\ или /
}
