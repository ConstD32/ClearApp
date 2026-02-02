slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = ClassicWindow::new()?;

    // Устанавливаем начальные значения
    ui.set_window_title("Мое приложение - Classic Window".into());
    ui.set_dark_mode(false);

    // Обработчики меню "Файл"
    ui.on_file_new(|| {
        println!("Файл -> Создать");
    });

    ui.on_file_open(|| {
        println!("Файл -> Открыть");
    });

    ui.on_file_save(|| {
        println!("Файл -> Сохранить");
    });

    ui.on_file_save_as(|| {
        println!("Файл -> Сохранить как");
    });

    ui.on_file_exit(|| {
        println!("Файл -> Выход");
        std::process::exit(0);
    });

    // Обработчики меню "Правка"
    ui.on_edit_undo(|| {
        println!("Правка -> Отменить");
    });

    ui.on_edit_redo(|| {
        println!("Правка -> Повторить");
    });

    ui.on_edit_cut(|| {
        println!("Правка -> Вырезать");
    });

    ui.on_edit_copy(|| {
        println!("Правка -> Копировать");
    });

    ui.on_edit_paste(|| {
        println!("Правка -> Вставить");
    });

    ui.on_edit_select_all(|| {
        println!("Правка -> Выделить все");
    });

    // Обработчики меню "Вид"
    ui.on_view_dark_mode(|dark| {
        println!("Вид -> Темный режим: {}", dark);
        ui.set_dark_mode(dark);
    });

    ui.on_view_toolbar(|visible| {
        println!("Вид -> Панель инструментов: {}", visible);
    });

    ui.on_view_statusbar(|visible| {
        println!("Вид -> Строка состояния: {}", visible);
    });

    // Обработчики меню "Справка"
    ui.on_help_about(|| {
        println!("Справка -> О программе");
        // Здесь можно показать диалоговое окно "О программе"
    });

    ui.on_help_documentation(|| {
        println!("Справка -> Документация");
    });

    // Запускаем приложение
    ui.run()?;

    Ok(())
}