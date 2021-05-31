#[macro_use] extern crate lazy_static;
extern crate term;

use std::error::Error;
use std::io::{Error as IoError, ErrorKind};
use std::fmt::{Display, Formatter, Result as FmtResult};

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
#[cfg(feature = "use_clap")]
pub mod util_clap;
pub mod util_tlv;
pub mod util_runtime;

/// iff!(condition, result_when_true, result_when_false)
#[macro_export] macro_rules! iff {
    ($c:expr, $t:expr, $f:expr) => ( if $c { $t } else { $f } )
}
#[macro_export] macro_rules! information {
    ($($arg:tt)+) => ( rust_util::util_msg::when(rust_util::util_msg::MessageType::INFO, || {
        rust_util::util_msg::print_info(&format!($($arg)+));
    }); )
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
    ($($arg:tt)+) => ( rust_util::util_msg::when(rust_util::util_msg::MessageType::DEBUG, || {
        rust_util::util_msg::print_debug(&format!($($arg)+))
    }); )
}
#[macro_export] macro_rules! failure_and_exit {
    ($($arg:tt)+) => ( {
        rust_util::util_msg::print_error(&format!($($arg)+));
        rust_util::util_runtime::invoke_callbacks();
        std::process::exit(-1);
    } )
}
#[macro_export] macro_rules! opt_value {
    ($ex: expr) => ( match $ex { Some(o) => o, None => return, } )
}

pub type XResult<T> = Result<T, Box<dyn Error>>;

pub fn new_box_error(m: &str) -> Box<dyn Error> {
    Box::new(IoError::new(ErrorKind::Other, m))
}

pub fn new_box_ioerror(m: &str) -> Box<dyn Error> {
    Box::new(IoError::new(ErrorKind::Other, m))
}

#[macro_export] macro_rules! opt_result {
    ($ex: expr, $($arg:tt)+) => (
        match $ex {
            Ok(o) => o,
            Err(e) => return Err(rust_util::SimpleError::new(
                format!("{}, file: {}, line: {}", format!($($arg)+, e), file!(), line!())
            ).into()),
        }
    )
}
#[macro_export] macro_rules! simple_error {
    ($($arg:tt)+) => ( Err(rust_util::SimpleError::new(
        format!("{}, file: {}, line: {}", format!($($arg)+), file!(), line!())
    ).into()) )
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

