// build.rc

slint::include_modules!();

use slint::{ModelRc, SharedString, VecModel};
use std::path::PathBuf;
use std::rc::Rc;

use crate::config::app_config::AppConfig;

pub struct UiContext {
    pub ui: AppWin,
    pub logs_model: Rc<VecModel<SharedString>>,
}

pub fn build_ui(config: AppConfig) -> Result<UiContext, slint::PlatformError> {
    let logs_model = Rc::new(VecModel::<SharedString>::default());

    let ui = AppWin::new()?;
    ui.set_logs(ModelRc::from(logs_model.clone()));

    logs_model.push("Старт приложения".into());
    logs_model.push("Инициализация GUI".into());

    // folders -> model
    let folders: Vec<FolderItem> = config
        .folders
        .iter()
        .map(|t| FolderItem {
            name: t.name.clone().into(),
            path: t.path.clone().into(),
        })
        .collect();

    let model = Rc::new(VecModel::from(folders));
    ui.set_folders(ModelRc::from(model));

    // callbacks
    bind_callbacks(&ui, config.clone());

    // meta
    ui.set_app_name(env!("CARGO_PKG_NAME").into());
    ui.set_version(env!("CARGO_PKG_VERSION").into());

    Ok(UiContext { ui, logs_model })
}

fn bind_callbacks(ui: &AppWin, config: AppConfig) {
    ui.on_clear_folder(move |index| {
        let folder = match config.folders.get(index as usize) {
            Some(t) => t,
            None => return,
        };

        let path = PathBuf::from(&folder.path);

        if let Err(e) = clean_folder(&path) {
            eprintln!("Ошибка очистки: {}", e);
        }
    });

    ui.on_file_new(|| println!("New file"));
    ui.on_file_open(|| println!("Open file"));
    ui.on_quit(|| std::process::exit(0));

    ui.on_update(|| {
        update();
        println!("Обновление");
    });

    // ui.on_show_about({
    //     let weak = ui.as_weak();
    //     move || {
    //         if let Some(a) = weak.upgrade() {
    //             a.show().unwrap();
    //         }
    //     }
    // });
}
