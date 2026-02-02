// tray_manager.rs
use std::sync::{Arc, Mutex};
use tray_icon::{
    TrayIconBuilder,
    MenuBuilder,
    menu::{MenuEvent, MenuItemBuilder},
};

pub struct TrayManager {
    icon: tray_icon::TrayIcon,
    is_visible: Arc<Mutex<bool>>,
}

impl TrayManager {
    pub fn new(
        icon_bytes: &[u8],
        tooltip: &str,
        on_show: impl Fn() + Send + 'static,
        on_hide: impl Fn() + Send + 'static,
        on_quit: impl Fn() + Send + 'static,
    ) -> Self {
        // Создание меню трея
        let show_item = MenuItemBuilder::new()
            .text("Показать")
            .id(1)
            .enabled(true)
            .build();

        let hide_item = MenuItemBuilder::new()
            .text("Скрыть")
            .id(2)
            .enabled(true)
            .build();

        let quit_item = MenuItemBuilder::new()
            .text("Выход")
            .id(3)
            .enabled(true)
            .build();

        let tray_menu = MenuBuilder::new()
            .items(&[&show_item, &hide_item, &quit_item])
            .separator()
            .build()
            .unwrap();

        // Создание иконки
        let icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(tooltip)
            .with_icon_from_buffer(icon_bytes, None, None)
            .build()
            .unwrap();

        // Обработка событий меню
        std::thread::spawn(move || {
            loop {
                if let Ok(event) = MenuEvent::receiver().recv() {
                    match event.id.0 {
                        1 => on_show(),
                        2 => on_hide(),
                        3 => on_quit(),
                        _ => {}
                    }
                }
            }
        });

        Self {
            icon,
            is_visible: Arc::new(Mutex::new(true)),
        }
    }

    pub fn minimize_to_tray(&self) {
        *self.is_visible.lock().unwrap() = false;
    }

    pub fn restore_from_tray(&self) {
        *self.is_visible.lock().unwrap() = true;
    }
}