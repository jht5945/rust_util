extern crate term;

use std::{
    env,
    fs,
    io::{self, Write, Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
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

pub fn is_symlink(path: &Path) -> bool {
    match path.symlink_metadata() {
        Err(_) => false,
        Ok(meta) => meta.file_type().is_symlink(),
    }
}

pub fn walk_dir<FError, FProcess, FFilter>(dir: &Path,
        func_walk_error: &FError,
        func_process_file: &FProcess,
        func_filter_dir: &FFilter) -> XResult<()>
        where FError: Fn(&Path, Box<dyn std::error::Error>) -> (),
              FProcess: Fn(&Path) -> (),
              FFilter: Fn(&Path) -> bool {
    walk_dir_with_depth_check(&mut 0u32, dir, func_walk_error, func_process_file, func_filter_dir)
}

fn walk_dir_with_depth_check<FError, FProcess, FFilter>(depth: &mut u32, dir: &Path,
        func_walk_error: &FError,
        func_process_file: &FProcess,
        func_filter_dir: &FFilter) -> XResult<()>
        where FError: Fn(&Path, Box<dyn std::error::Error>) -> (),
              FProcess: Fn(&Path) -> (),
              FFilter: Fn(&Path) -> bool {
    if *depth > 100u32 {
        return Err(new_box_error(&format!("Depth exceed, depth: {}, path: {:?}", *depth, dir)));
    }
    let read_dir = match dir.read_dir() {
        Err(err) => {
            func_walk_error(&dir, Box::new(err));
            return Ok(());
        },
        Ok(rd) => rd,
    };
    for dir_entry_item in read_dir {
        let dir_entry = match dir_entry_item {
            Err(err) => {
                func_walk_error(&dir, Box::new(err));
                continue; // Ok?
            },
            Ok(item) => item,
        };

        let path_buf = dir_entry.path();
        let sub_dir = path_buf.as_path();
        if sub_dir.is_file() {
            func_process_file(&sub_dir);
        } else if sub_dir.is_dir() {
            if func_filter_dir(&sub_dir) {
                *depth += 1;
                match walk_dir_with_depth_check(depth, &sub_dir, func_walk_error, func_process_file, func_filter_dir) {
                    Err(err) => {
                        func_walk_error(&sub_dir, err);
                        ()
                    },
                    Ok(_) => (),
                }
                *depth -= 1;
            }
        } // should process else ? not file, dir
    }
    Ok(())
}

pub fn new_box_error(m: &str) -> Box<dyn std::error::Error> {
    Box::new(Error::new(ErrorKind::Other, m))
}

pub enum MessageType { INFO, OK, WARN, ERROR, }

pub fn print_color(color: Option<term::color::Color>, is_bold: bool, m: &str) {
    let mut t = term::stdout().unwrap();
    match color {
        Some(c) => t.fg(c).unwrap(),
        None => (),
    }
    if is_bold {
        t.attr(term::Attr::Bold).unwrap();
    }
    write!(t, "{}", m).unwrap();
    t.reset().unwrap();
}

pub fn print_color_and_flush(color: Option<term::color::Color>, is_bold: bool, m: &str) {
    print_color(color, is_bold, m);
    flush_stdout();
}

pub fn print_message_ex(color: Option<term::color::Color>, h: &str, message: &str) {
    print_color(color, true, h);
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

pub fn clear_lastline() {
    print_lastline("");
}

pub fn print_lastline(line: &str) {
    print!("\x1b[1000D{}\x1b[K", line);
    flush_stdout();
}

pub fn parse_size(size: &str) -> XResult<i64> {
    let lower_size = size.to_lowercase();
    let no_last_b_size = if lower_size.ends_with("b") {
        &lower_size[0..lower_size.len()-1]
    } else {
        &lower_size
    };
    if no_last_b_size.ends_with("k") {
        return Ok((SIZE_KB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("m") {
        return Ok((SIZE_MB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("g") {
        return Ok((SIZE_GB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("t") {
        return Ok((SIZE_TB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("p") {
        return Ok((SIZE_PB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    }

    Ok(no_last_b_size.parse::<i64>()?)
}

pub fn get_display_size(size: i64) -> String {
    if size < SIZE_KB {
        return size.to_string();
    } else if size < SIZE_MB {
        return format!("{:.*}KB", 2, (size as f64) / 1024.);
    } else if size < SIZE_GB {
        return format!("{:.*}MB", 2, (size as f64) / 1024. / 1024.);
    } else if size < SIZE_TB {
        return format!("{:.*}GB", 2, (size as f64) / 1024. / 1024. / 1024.);
    } else if size < SIZE_PB {
        return format!("{:.*}TB", 2, (size as f64) / 1024. / 1024. / 1024. / 1024.);
    } else {
        return format!("{:.*}PB", 2, (size as f64) / 1024. / 1024. / 1024. / 1024. / 1024.);
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

pub fn copy_io<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W, total: i64) -> io::Result<u64>
        where R: io::Read, W: io::Write {
    // let mut written = 0u64;
    // let written_cell = RefCell::new(written);
    let start = SystemTime::now();
    let written = copy_io_callback(reader, writer, total, &|total, written, _len| {
       // let mut written_borrowed = written_cell.borrow_mut();
        //written_cell.replace_with(|&mut old_written| old_written + len as u64);
        let cost = SystemTime::now().duration_since(start.clone()).unwrap().as_secs();
        let mut download_speed = "-".to_string();
        if cost > 0 {
            download_speed = format!("{}/s", get_display_size((written / cost) as i64));
        }
        if total > 0 {
            print_lastline(&format!("Downloading, Total: {}, Downloaded: {}, Speed: {}", 
                get_display_size(total),
                get_display_size(written as i64),
                download_speed));
        } else {
            print_lastline(&format!("Downloading, Downloaded: {}, Speed: {}", 
                get_display_size(written as i64),
                download_speed));
        }
    });
    println!();
    written
}

pub fn copy_io_callback<R: ?Sized, W: ?Sized, FCallback>(reader: &mut R, writer: &mut W, total: i64, callback: &FCallback) -> io::Result<u64>
        where R: io::Read,
              W: io::Write,
              FCallback: Fn(i64, u64, usize) -> () {
    let mut written = 0u64;
    let mut buf: [u8; DEFAULT_BUF_SIZE] = [0u8; DEFAULT_BUF_SIZE];
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
        callback(total, written, len);
    }
}
