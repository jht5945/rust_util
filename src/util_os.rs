use std::path::PathBuf;

pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

pub fn is_macos_or_linux() -> bool {
    is_macos() || is_linux()
}

pub fn get_full_work_dir() -> Option<String> {
    PathBuf::from(".").canonicalize().ok().map(|p| {
        p.to_str().map(ToString::to_string)
    }).flatten()
}