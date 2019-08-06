
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
