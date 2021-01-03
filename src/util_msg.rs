use std::env;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref IS_ATTY: bool = is_atty();
    static ref LOGGER_LEVEL: MessageType = get_logger_level();
    static ref PRINT_MESSAGE_LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

#[derive(Clone, Copy)]
pub enum MessageType { DEBUG, INFO, OK, WARN, ERROR }

impl MessageType {
    pub fn get_u8_value(&self) -> u8 {
        match self {
            MessageType::DEBUG => 0,
            MessageType::INFO  => 1,
            MessageType::OK    => 2,
            MessageType::WARN  => 3,
            MessageType::ERROR => 4,
        }
    }
}

pub fn get_logger_level() -> MessageType {
    if let Some(logger_level) = env::var("LOGGER_LEVEL").ok()
                                                        .or_else(|| env::var("LOGGER").ok())
                                                        .or_else(|| env::var("LEVEL").ok()) {
        match logger_level.trim().to_lowercase().as_str() {
            "debug" | "*" => MessageType::DEBUG,
            "info"  | "?" => MessageType::INFO,
            "ok"    | "#" => MessageType::OK,
            "warn"  | "!" => MessageType::WARN,
            "error" | "^" => MessageType::ERROR,
            _ => {
                print_message_ex(Some(term::color::YELLOW), "[WARN ]", &format!("Unknown logger level: {}, set to default INFO", logger_level));
                MessageType::INFO
            },
        }
    } else {
        MessageType::INFO
    }
}

pub fn is_atty() -> bool {
    let stdout_fileno = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) };
    stdout_fileno != 0
}

pub fn print_color(color: Option<term::color::Color>, is_bold: bool, m: &str) {
    match term::stdout() {
        Some(mut t) => {
            if *IS_ATTY {
                if let Some(c) = color {
                    t.fg(c).ok();
                }
                if is_bold {
                    t.attr(term::Attr::Bold).ok();
                }
                write!(t, "{}", m).ok();
                t.reset().ok();
            } else {
                write!(t, "{}", m).ok();
            }
        },
        None => print!("{}", m),
    }
}

pub fn print_color_and_flush(color: Option<term::color::Color>, is_bold: bool, m: &str) {
    print_color(color, is_bold, m);
    flush_stdout();
}

pub fn print_message_ex(color: Option<term::color::Color>, h: &str, message: &str) {
    let mut lock = PRINT_MESSAGE_LOCK.lock().unwrap();
    print_color(color, true, h);
    println!(" {}", message);
    *lock = ();
}

pub fn print_ok   (message: &str) { print_message(MessageType::OK,    message); }
pub fn print_warn (message: &str) { print_message(MessageType::WARN,  message); }
pub fn print_error(message: &str) { print_message(MessageType::ERROR, message); }
pub fn print_info (message: &str) { print_message(MessageType::INFO,  message); }
pub fn print_debug(message: &str) { print_message(MessageType::DEBUG, message); }

#[inline]
pub fn is_logger_level_enabled(mt: MessageType) -> bool {
    let logger_level = *LOGGER_LEVEL;
    mt.get_u8_value() >= logger_level.get_u8_value()
}

pub fn print_message(mt: MessageType, message: &str) {
    if is_logger_level_enabled(mt) {
        match mt {
            MessageType::OK    => print_message_ex(Some(term::color::GREEN),   "[OK   ]", message),
            MessageType::WARN  => print_message_ex(Some(term::color::YELLOW),  "[WARN ]", message),
            MessageType::ERROR => print_message_ex(Some(term::color::RED),     "[ERROR]", message),
            MessageType::INFO  => print_message_ex(None,                       "[INFO ]", message),
            MessageType::DEBUG => print_message_ex(Some(term::color::MAGENTA), "[DEBUG]", message),
        }
    }
}

impl MessageType {
    pub fn print(&self, message: &str) {
        print_message(*self, message);
    }
}

pub fn flush_stdout() {
    io::stdout().flush().ok();
}

pub fn clear_lastline() {
    print_lastline("");
}

pub fn print_lastline(line: &str) {
    print!("\x1b[1000D{}\x1b[K", line);
    flush_stdout();
}

// thanks https://blog.csdn.net/star_xiong/article/details/89401149
pub fn find_char_boundary(s: &str, index: usize) -> usize {
    if s.len() <= index {
        return index;
    }
    let mut new_index = index;
    while !s.is_char_boundary(new_index) {
        new_index += 1;
    }
    new_index
}

pub fn get_term_width_message(message: &str, left: usize) -> String {
    match term_size::dimensions() {
        None => message.to_string(),
        Some((w, _h)) => {
            let len = message.len();
            if w > len {
               return message.to_string();
            }
            let mut s = String::new();
            s.push_str(&message[0..find_char_boundary(&message, w - 10 - 5 - left)]);
            s.push_str("[...]");
            s.push_str(&message[find_char_boundary(&message, len - 10)..]);
            s
        },
    }
}
