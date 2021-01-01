#[macro_use] extern crate rust_util;

use rust_util::XResult;

// cargo run --example log
fn main() -> XResult<()> {
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

    simple_error!("helloworld {}", 1)
}