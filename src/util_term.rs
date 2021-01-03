use std::io::{self, Write};

pub const RED:    &str = "\x1B[91m";
pub const GREEN:  &str = "\x1B[92m";
pub const YELLOW: &str = "\x1B[93m";
pub const BOLD:   &str = "\x1B[1m";
pub const UNDER:  &str = "\x1B[4m";
pub const END:    &str = "\x1B[0m";

pub fn read_yes_no(hint: &str) -> bool {
    loop {
        print!("{} (Yes/No): ", hint);
        io::stdout().flush().ok();
        let mut buff = String::new();
        let _ = io::stdin().read_line(&mut buff).expect("Read line from stdin");
        let buff = buff.trim().to_lowercase();
        if vec!["y", "yes"].contains(&buff.as_str()) {
            return true;
        }
        if vec!["n", "no"].contains(&buff.as_str()) {
            return false;
        }
    }
}
