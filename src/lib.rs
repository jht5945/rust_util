extern crate term;

use std::{
    env,
    fs,
    io::{self, Write, Error, ErrorKind},
    path::PathBuf,
    process::Command,
};

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;
pub const SIZE_KB: i64 = 1024;
pub const SIZE_MB: i64 = SIZE_KB * SIZE_KB;
pub const SIZE_GB: i64 = SIZE_MB * SIZE_KB;
pub const SIZE_PB: i64 = SIZE_GB * SIZE_KB;
pub const SIZE_TB: i64 = SIZE_PB * SIZE_KB;

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn is_macos() -> bool {
    if cfg!(target_os = "macos") {
        true
    } else {
        false
    }
}

pub fn is_linux() -> bool {
    if cfg!(target_os = "linux") {
        true
    } else {
        false
    }
}

pub fn is_macos_or_linux() -> bool {
    is_macos() || is_linux()
}

pub fn get_home_str() -> Option<String> {
    match is_macos_or_linux() {
        true => env::var("HOME").ok(),
        false => None,
    }
}

pub fn get_home_path() -> Option<PathBuf> {
    Some(PathBuf::from(get_home_str()?))
}

pub fn get_absolute_path(path: &str) -> Option<PathBuf> {
    if path == "~" {
        return Some(PathBuf::from(get_home_str()?));
    } else if path.starts_with("~/") {
        return Some(PathBuf::from(&format!("{}/{}", get_home_str()?, &path[2..])));
    }
    fs::canonicalize(path).ok()
}

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

pub fn flush_stdout() {
    match io::stdout().flush() {
        Err(err) => print_message(MessageType::ERROR, &format!("Flush stdout failed: {}", err)),
        Ok(_) => (),
    }
}

pub fn print_lastline(line: &str) {
    print!("\x1b[100D{}\x1b[K", line);
    flush_stdout();
}

pub fn get_display_size(size: i64) -> String {
    if size < SIZE_KB {
        return size.to_string();
    } else if size < SIZE_MB {
        return format!("{:.*}KB", 2, (size as f64) / 1024.);
    } else if size < SIZE_GB {
        return format!("{:.*}MB", 2, (size as f64) / 1024. / 1024.);
    } else if size < SIZE_PB {
        return format!("{:.*}GB", 2, (size as f64) / 1024. / 1024. / 1024.);
    } else if size < SIZE_TB {
        return format!("{:.*}PB", 2, (size as f64) / 1024. / 1024. / 1024. / 1024.);
    } else {
        return format!("{:.*}TB", 2, (size as f64) / 1024. / 1024. / 1024. / 1024. / 1024.);
    }
}

pub fn run_command_and_wait(cmd: &mut Command) -> io::Result<()> {
    cmd.spawn()?.wait()?;
    Ok(())
}

pub fn extract_package_and_wait(dir: &str, file_name: &str) -> io::Result<()> {
    let mut cmd: Command;
    if file_name.ends_with(".zip") {
        cmd = Command::new("unzip");
    } else if file_name.ends_with(".tar.gz") {
        cmd = Command::new("tar");
        cmd.arg("-xzvf");
    } else {
        let m: &str = &format!("Unknown file type: {}", file_name);
        return Err(Error::new(ErrorKind::Other, m));
    }
    cmd.arg(file_name).current_dir(dir);
    run_command_and_wait(&mut cmd)
}

