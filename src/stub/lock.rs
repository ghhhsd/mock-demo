use std::sync::{Mutex, Once};

static mut LOCK: Option<Mutex<()>> = None;
static START: Once = Once::new();

pub fn get_lock() -> &'static Option<Mutex<()>> {
    unsafe {
        Once::call_once(&START, || {
            let lock = Mutex::new(());
            LOCK = Some(lock);
        });
        &LOCK
    }
}