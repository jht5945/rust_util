use std::sync::Mutex;
use crate::util_msg::MessageType;

lazy_static! {
    static ref EXIT_CALLBACK: Mutex<Vec<Box<dyn Fn() -> () + Send + 'static>>> = Mutex::new(vec![]);
}

pub fn register_callback<F>(f: F) where F: Fn() -> () + Send + 'static {
    let mut exit_callbacks = EXIT_CALLBACK.lock().unwrap();
    exit_callbacks.push(Box::new(f));
}

pub fn invoke_callbacks() {
    let mut exit_callbacks = EXIT_CALLBACK.lock().unwrap();
    let total = exit_callbacks.len();
    let mut index = 0;
    while exit_callbacks.len() > 0 {
        crate::util_msg::when(MessageType::DEBUG, || {
            crate::util_msg::print_debug(&format!("Running exit callbacks: {} of {}", index, total));
        });
        exit_callbacks.remove(0)();
        index += 1;
    }
}