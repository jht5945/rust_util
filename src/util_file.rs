use std::fs::{self, File};
use std::io::{Lines, BufReader};
use std::path::{Path, PathBuf};
use crate::{util_os, util_io, util_msg, new_box_ioerror, XResult};

pub struct JoinFilesReader {
    files: Vec<String>,
    file_ptr: usize,
    file_lines: Option<Box<Lines<BufReader<File>>>>,
}

fn open_file_as_lines(f: &str) -> XResult<Lines<BufReader<File>>> {
    let f = File::open(f)?;
    let br = BufReader::new(f);
    use std::io::BufRead;
    Ok(br.lines())
}

impl JoinFilesReader {
    pub fn new(fns: &[&str]) -> XResult<Self> {
        let mut files: Vec<String> = vec![];
        for f in fns {
            files.push(f.to_string());
        }
        let file_ptr = 0;
        let mut file_lines = None;
        if !files.is_empty() {
            file_lines = Some(Box::new(open_file_as_lines(&files[0])?));
        }
        Ok(Self { files, file_ptr, file_lines })
    }
}

impl Iterator for JoinFilesReader {
    type Item = XResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.file_lines {
                Some(ref mut ln) => match ln.next() {
                    Some(r) => return Some(r.map_err(|e| e.into())),
                    None => {
                        self.file_ptr += 1;
                        self.file_lines = None;
                        if self.file_ptr >= self.files.len() {
                            return None;
                        } else {
                            // if open file failed, will not continue read files
                            self.file_lines = Some(Box::new(match open_file_as_lines(&self.files[self.file_ptr]) {
                                Ok(ln) => ln,
                                Err(e) => return Some(Err(e)),
                            }));
                        }
                    }
                },
                None => return None,
            }
            if self.file_ptr >= self.files.len() {
                return None;
            }
        }
    }
}

pub fn join_path(path1: &str, path2: &str) -> String {
    let sp = if util_os::is_macos_or_linux() { "/" } else { "\\" };
    if path1.ends_with(sp) {
        path1.to_string() + path2
    } else {
        path1.to_string() + sp + path2
    }
}

pub fn find_parents_exists_file(file: &str) -> Option<PathBuf> {
    let mut loop_count = 0_usize;
    match PathBuf::from(".").canonicalize() {
        Err(_) => None,
        Ok(mut path) => loop {
            loop_count += 1;
            if loop_count > 1000 {
                util_msg::print_error("Loop count more than 1000!");
                return None;
            }
            if path.join(file).is_file() {
                return Some(path);
            }
            if !path.pop() {
                return None;
            }
        }
    }
}

pub fn find_parents_exists_dir(dir: &str) -> Option<PathBuf> {
    let mut loop_count = 0_usize;
    match PathBuf::from(".").canonicalize() {
        Err(_) => None,
        Ok(mut path) => loop {
            loop_count += 1;
            if loop_count > 1000 {
                util_msg::print_error("Loop count more than 1000!");
                return None;
            }
            if path.join(dir).is_dir() {
                return Some(path);
            }
            if !path.pop() {
                return None;
            }
        }
    }
}

pub fn read_config(config: Option<String>, files: &[String]) -> Option<PathBuf> {
    match config {
        Some(config_str) => Some(PathBuf::from(config_str)),
        None => locate_file(files),
    }
}

pub fn locate_file(files: &[String]) -> Option<PathBuf> {
    for f in files {
        match PathBuf::from(&resolve_file_path(f)) {
            pb if pb.is_file() => return Some(pb),
            _ => (),
        }
    }
    None
}

#[deprecated]
pub fn get_home_str() -> Option<String> {
    util_os::get_user_home()
}

pub fn resolve_file_path(path: &str) -> String {
    let home_path = match util_os::get_user_home() {
        Some(p) => p,
        None => return path.to_owned(),
    };
    match path {
        "~" => home_path,
        p if p.starts_with("~/") => home_path + &path.chars().skip(1).collect::<String>(),
        p => p.to_owned(),
    }
}

pub fn get_home_path() -> Option<PathBuf> {
    Some(PathBuf::from(util_os::get_user_home()?))
}

pub fn get_absolute_path(path: &str) -> Option<PathBuf> {
    match path {
        "~" => Some(PathBuf::from(util_os::get_user_home()?)),
        path if path.starts_with("~/") => Some(PathBuf::from(&format!("{}/{}", util_os::get_user_home()?, &path[2..]))),
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
    where FError: Fn(&Path, Box<dyn std::error::Error>),
          FProcess: Fn(&Path),
          FFilter: Fn(&Path) -> bool {
    walk_dir_with_depth_check(&mut 0u32, dir, func_walk_error, func_process_file, func_filter_dir)
}

fn walk_dir_with_depth_check<FError, FProcess, FFilter>(depth: &mut u32, dir: &Path,
                                                        func_walk_error: &FError,
                                                        func_process_file: &FProcess,
                                                        func_filter_dir: &FFilter) -> XResult<()>
    where FError: Fn(&Path, Box<dyn std::error::Error>),
          FProcess: Fn(&Path),
          FFilter: Fn(&Path) -> bool {
    if *depth > 100u32 {
        return Err(new_box_ioerror(&format!("Depth exceed, depth: {}, path: {:?}", *depth, dir)));
    }
    let read_dir = match dir.read_dir() {
        Ok(rd) => rd,
        Err(err) => {
            func_walk_error(dir, Box::new(err));
            return Ok(());
        }
    };
    for dir_entry_item in read_dir {
        let dir_entry = match dir_entry_item {
            Ok(item) => item,
            Err(err) => {
                func_walk_error(dir, Box::new(err));
                continue; // Ok?
            }
        };

        let path_buf = dir_entry.path();
        let sub_dir = path_buf.as_path();
        if sub_dir.is_file() {
            func_process_file(sub_dir);
        } else if sub_dir.is_dir() && func_filter_dir(sub_dir) {
            *depth += 1;
            if let Err(err) = walk_dir_with_depth_check(depth, sub_dir, func_walk_error, func_process_file, func_filter_dir) {
                func_walk_error(sub_dir, err);
            }
            *depth -= 1;
        } // should process else ? not file, dir
    }
    Ok(())
}
