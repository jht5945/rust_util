#[macro_use]
extern crate lazy_static;
extern crate term;

use std::error::Error;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub mod util_io;
pub mod util_os;
pub mod util_env;
pub mod util_cmd;
pub mod util_msg;
pub mod util_str;
pub mod util_size;
pub mod util_file;
pub mod util_time;
pub mod util_net;
pub mod util_term;
pub mod util_git;
// #[feature(use_clap)]
pub mod util_clap;

/// iff!(condition, result_when_true, result_when_false)
#[macro_export] macro_rules! iff {
    ($c:expr, $t:expr, $f:expr) => ( if $c { $t } else { $f } )
}
#[macro_export] macro_rules! information {
    ($($arg:tt)+) => ( rust_util::util_msg::print_info(&format!($($arg)+)); )
}
#[macro_export] macro_rules! success {
    ($($arg:tt)+) => ( rust_util::util_msg::print_ok(&format!($($arg)+)); )
}
#[macro_export] macro_rules! warning {
    ($($arg:tt)+) => ( rust_util::util_msg::print_warn(&format!($($arg)+)); )
}
#[macro_export] macro_rules! failure {
    ($($arg:tt)+) => ( rust_util::util_msg::print_error(&format!($($arg)+)); )
}
#[macro_export] macro_rules! debugging {
    ($($arg:tt)+) => ( rust_util::util_msg::print_debug(&format!($($arg)+)); )
}
#[macro_export] macro_rules! opt_value {
    ($e: expr) => ( match $e { Some(o) => o, None => return, } )
}

pub type XResult<T> = Result<T, Box<dyn Error>>;

pub fn new_box_error(m: &str) -> Box<dyn Error> {
    Box::new(IoError::new(ErrorKind::Other, m))
}

pub fn new_box_ioerror(m: &str) -> Box<dyn Error> {
    Box::new(IoError::new(ErrorKind::Other, m))
}

#[macro_export] macro_rules! simple_error {
    ($($arg:tt)+) => ( Err(rust_util::SimpleError::new(format!($($arg)+)).into()) )
}

#[derive(Debug)]
pub struct SimpleError {
    pub message: String,
    pub source: Option<Box<dyn Error>>,
}

impl SimpleError {
    pub fn new(message: String) -> Self {
        Self { message, source: None }
    }

    pub fn new2(message: String, source: Box<dyn Error>) -> Self {
        Self { message, source: Some(source) }
    }
}

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.source {
            None => write!(f, "SimpleErorr, message: {}", self.message),
            Some(e) => write!(f, "SimpleErorr, message: {}, source erorr: {}", self.message, e),
        }
    }
}

impl Error for SimpleError {}

