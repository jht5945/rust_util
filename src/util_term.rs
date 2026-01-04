use std::io::{self, Write};

use crate::util_msg;

pub const STD_BLACK: &str = "\x1B[30m";
pub const STD_RED: &str = "\x1B[31m";
pub const STD_GREEN: &str = "\x1B[32m";
pub const STD_YELLOW: &str = "\x1B[33m";
pub const STD_BLUE: &str = "\x1B[34m";
pub const STD_MAGENTA: &str = "\x1B[35m"; // 品红色/洋红
pub const STD_CYAN: &str = "\x1B[36m"; // 青色
pub const STD_WHITE: &str = "\x1B[37m";

pub const BLACK: &str = "\x1B[90m";
pub const RED: &str = "\x1B[91m";
pub const GREEN: &str = "\x1B[92m";
pub const YELLOW: &str = "\x1B[93m";
pub const BLUE: &str = "\x1B[94m";
pub const MAGENTA: &str = "\x1B[95m";
pub const CYAN: &str = "\x1B[96m";
pub const WHITE: &str = "\x1B[97m";

pub const BG_STD_BLACK: &str = "\x1B[40m";
pub const BG_STD_RED: &str = "\x1B[41m";
pub const BG_STD_GREEN: &str = "\x1B[42m";
pub const BG_STD_YELLOW: &str = "\x1B[43m";
pub const BG_STD_BLUE: &str = "\x1B[44m";
pub const BG_STD_MAGENTA: &str = "\x1B[45m";
pub const BG_STD_CYAN: &str = "\x1B[46m";
pub const BG_STD_WHITE: &str = "\x1B[47m";

pub const BG_BLACK: &str = "\x1B[100m";
pub const BG_RED: &str = "\x1B[101m";
pub const BG_GREEN: &str = "\x1B[102m";
pub const BG_YELLOW: &str = "\x1B[103m";
pub const BG_BLUE: &str = "\x1B[104m";
pub const BG_MAGENTA: &str = "\x1B[105m";
pub const BG_CYAN: &str = "\x1B[106m";
pub const BG_WHITE: &str = "\x1B[107m";

pub const BOLD: &str = "\x1B[1m";
pub const UNDER: &str = "\x1B[4m";
pub const END: &str = "\x1B[0m";

pub fn read_yes_no(hint: &str) -> bool {
    loop {
        util_msg::print_ex(&format!("{} (Yes/No): ", hint), false);
        io::stdout().flush().ok();
        let mut buff = String::new();
        let _ = io::stdin().read_line(&mut buff).expect("Read line from stdin");
        let buff = buff.trim().to_lowercase();
        if ["y", "yes"].contains(&buff.as_str()) {
            return true;
        }
        if ["n", "no"].contains(&buff.as_str()) {
            return false;
        }
    }
}
