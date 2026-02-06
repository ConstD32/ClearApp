// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    // slint_build::compile("ui/classic_window.slint").unwrap();
    // slint_build::compile("ui/learn.slint").unwrap();

    // Код ниже копирует файл конфигурации **********************************************
    // region
    // Получаем выходной каталог (target/debug или target/release)
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Определяем целевой каталог
    // #[rustfmt::skip]
    let target_dir = out_path
        .parent()
        .unwrap() // target/debug/build/<hash>/
        .parent()
        .unwrap() // target/debug/build/
        .parent()
        .unwrap(); // target/debug/

    // Список файлов для копирования
    let files_to_copy = ["opengl32.dll", "config.json"];

    for &src_file in &files_to_copy {
        let target_path = target_dir.join(src_file);

        // Копируем файл
        if Path::new(src_file).exists() {
            if let Err(e) = fs::copy(src_file, &target_path) {
                panic!("Не удалось скопировать {}: {}", src_file, e);
            }
            println!("cargo:warning=Скопирован {} в {}", src_file, target_path.display());
        } else {
            println!("cargo:warning=Файл {} не найден", src_file);
        }

        // Пересобираем при изменении файла
        println!("cargo:rerun-if-changed={}", src_file);
    }
    // ************************************************************************************
    // endregion
}
