extern crate web_sys;

pub enum LogLevel {
    Warning
}

pub fn log(msg: &str, level: LogLevel) {
    match level {
        LogLevel::Warning => web_sys::console::log_1(&msg.into())
    }
    println!("{}", msg);
}