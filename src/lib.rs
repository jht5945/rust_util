extern crate term;

use std::{
    //cell::RefCell,
    env,
    fs,
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

pub mod util_io;
pub mod util_os;
pub mod util_msg;
pub mod util_size;

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
