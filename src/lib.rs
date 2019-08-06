extern crate term;

use std::{
    //cell::RefCell,
    env,
    fs,
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;
pub const SIZE_KB: i64 = 1024;
pub const SIZE_MB: i64 = SIZE_KB * SIZE_KB;
pub const SIZE_GB: i64 = SIZE_MB * SIZE_KB;
pub const SIZE_PB: i64 = SIZE_GB * SIZE_KB;
pub const SIZE_TB: i64 = SIZE_PB * SIZE_KB;

pub mod util_io;
pub mod util_os;
pub mod util_msg;

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_home_str() -> Option<String> {
    match util_os::is_macos_or_linux() {
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

pub fn new_box_ioerror(m: &str) -> Box<dyn std::error::Error> {
    Box::new(Error::new(ErrorKind::Other, m))
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
