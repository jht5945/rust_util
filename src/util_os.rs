use std::env;
use std::path::PathBuf;

use crate::iff;

pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

pub fn is_macos_or_linux() -> bool {
    is_macos() || is_linux()
}

pub fn get_user_home() -> Option<String> {
    iff!(is_macos_or_linux(), env::var("HOME").ok(), None)
}

pub fn get_full_work_dir() -> Option<String> {
    PathBuf::from(".").canonicalize().ok().and_then(|p| {
        p.to_str().map(ToString::to_string)
    })
}