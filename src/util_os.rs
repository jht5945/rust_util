
pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

pub fn is_macos_or_linux() -> bool {
    is_macos() || is_linux()
}
