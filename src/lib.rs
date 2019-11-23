#[macro_use]
extern crate lazy_static;
extern crate term;

use std::{
    io::{Error, ErrorKind},
};

pub mod util_io;
pub mod util_os;
pub mod util_env;
pub mod util_cmd;
pub mod util_msg;
pub mod util_size;
pub mod util_file;

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn new_box_error(m: &str) -> Box<dyn std::error::Error> {
    Box::new(Error::new(ErrorKind::Other, m))
}

pub fn new_box_ioerror(m: &str) -> Box<dyn std::error::Error> {
    Box::new(Error::new(ErrorKind::Other, m))
}
