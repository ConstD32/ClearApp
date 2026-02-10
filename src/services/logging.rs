use slint::{SharedString, VecModel};
use std::rc::Rc;
use std::cell::RefCell;
use chrono::Local;

thread_local! {
    static LOG_MODEL: RefCell<Option<Rc<VecModel<SharedString>>>> = RefCell::new(None);
}

pub fn init_ui_logger(model: Rc<VecModel<SharedString>>) {
    LOG_MODEL.with(|m| {
        *m.borrow_mut() = Some(model);
    });
}

pub fn log_message(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("[{}] {}", timestamp, msg);

    LOG_MODEL.with(|m| {
        if let Some(model) = &*m.borrow() {
            model.push(line.into());
        }
    });
}

pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("PANIC: {:?}", info);

        slint::invoke_from_event_loop(move || {
            log_message(&msg);
        }).ok();
    }));
}