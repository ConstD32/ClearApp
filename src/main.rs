// main.rc

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod domain;
mod services;
mod ui;

slint::include_modules!();

use crate::config::app_config::AppConfig;
use services::logging::setup_panic_hook;

use crate::ui::build_ui::build_ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Загрузка конфигурации
    let config = AppConfig::load()?;

    let ctx = build_ui(config)?;
    setup_panic_hook();

    let _ = ctx.ui.run();
    Ok(())
}