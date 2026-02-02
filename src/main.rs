slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
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
