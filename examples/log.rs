#[macro_use] extern crate rust_util;

// cargo run --example log
fn main() {
    information!("Hello {}", "world!");
}