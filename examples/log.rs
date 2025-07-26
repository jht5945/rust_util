#[macro_use] extern crate rust_util;

use std::sync::mpsc::channel;
use std::thread;
use rust_util::{XResult, SimpleError};
use rust_util::util_msg::{set_logger_sender, set_logger_std_out};

// cargo run --example log
fn main() -> XResult<()> {
    let (sender, receiver) = channel::<String>();
    set_logger_sender(sender);

    let _t = thread::spawn(move || {
        loop {
            let msg = receiver.recv();
            println!("[RECV]: {:?}", msg)
        }
    });

    std::env::set_var("LOGGER_LEVEL", "*");
    println!(r##"env LOGGER_LEVEL set to:
debug or *
info or ? -- default
ok or #
warn or !
error or ^"##);

    debugging!("Hello {}", "world!");
    information!("Hello {}", "world!");
    success!("Hello {}", "world!");
    warning!("Hello {}", "world!");
    failure!("Hello {}", "world!");

    println!("{:?}", test_opt_result());

    set_logger_std_out(false);
    information!("Std err!");
    warning!("Std err!");
    set_logger_std_out(true);

    simple_error!("helloworld {}", 1)
}

fn test_opt_result() -> XResult<()> {
    let a = Err(SimpleError::new("test".into()));
    opt_result!(a, "error: {}")
}
