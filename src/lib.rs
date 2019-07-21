extern crate term;

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

pub enum MessageType { INFO, OK, WARN, ERROR, }

pub fn print_message_ex(color: Option<term::color::Color>, h: &str, message: &str) {
    let mut t = term::stdout().unwrap();
    match color {
        Some(c) => t.fg(c).unwrap(),
        None => (),
    }
    write!(t, "{}", h).unwrap();
    t.reset().unwrap();
    println!(" {}", message);
}

pub fn print_message(mt: MessageType, message: &str) {
    match mt {
        MessageType::OK => print_message_ex(Some(term::color::GREEN), "[OK   ]", message),
        MessageType::WARN => print_message_ex(Some(term::color::YELLOW), "[WARN ]", message),
        MessageType::ERROR => print_message_ex(Some(term::color::RED), "[ERROR]", message),
        MessageType::INFO => print_message_ex(None, "[INFO]", message),
    }
}
