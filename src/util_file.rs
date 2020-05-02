
use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

use super::{
    iff,
    util_os,
    util_io,
    new_box_ioerror,
    XResult,
};

pub fn get_home_str() -> Option<String> {
    iff!(util_os::is_macos_or_linux(), env::var("HOME").ok(), None)
}

pub fn resolve_file_path(path: &str) -> String {
    let home_path = match get_home_str() {
        Some(p) => p, None => return path.to_owned(),
    };
    match path {
        "~" => home_path,
        p if p.starts_with("~/") => home_path + &path.chars().skip(1).collect::<String>(),
        p => p.to_owned(),
    }
}

pub fn get_home_path() -> Option<PathBuf> {
    Some(PathBuf::from(get_home_str()?))
}

pub fn get_absolute_path(path: &str) -> Option<PathBuf> {
    match path {
        "~" => Some(PathBuf::from(get_home_str()?)),
        path if path.starts_with("~/")  => Some(PathBuf::from(&format!("{}/{}", get_home_str()?, &path[2..]))),
        path => fs::canonicalize(path).ok(),
    }
}

pub fn read_file_content(file: &str) -> XResult<String> {
    match get_absolute_path(file) {
        None => Err(new_box_ioerror(&format!("File not found: {}", file))),
        Some(p) => util_io::read_to_string(&mut fs::File::open(p)?),
    }
}

pub fn is_symlink(path: &Path) -> bool {
    path.symlink_metadata().map(|meta| meta.file_type().is_symlink()).unwrap_or(false)
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
        return Err(new_box_ioerror(&format!("Depth exceed, depth: {}, path: {:?}", *depth, dir)));
    }
    let read_dir = match dir.read_dir() {
        Ok(rd) => rd, Err(err) => {
            func_walk_error(&dir, Box::new(err));
            return Ok(());
        },
    };
    for dir_entry_item in read_dir {
        let dir_entry = match dir_entry_item {
            Ok(item) => item, Err(err) => {
                func_walk_error(&dir, Box::new(err));
                continue; // Ok?
            },
        };

        let path_buf = dir_entry.path();
        let sub_dir = path_buf.as_path();
        if sub_dir.is_file() {
            func_process_file(&sub_dir);
        } else if sub_dir.is_dir() && func_filter_dir(&sub_dir) {
            *depth += 1;
            if let Err(err) = walk_dir_with_depth_check(depth, &sub_dir, func_walk_error, func_process_file, func_filter_dir) {
                func_walk_error(&sub_dir, err);
            }
            *depth -= 1;
        } // should process else ? not file, dir
    }
    Ok(())
}
