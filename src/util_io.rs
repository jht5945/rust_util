
use std::{
    io::{self, ErrorKind},
    time::SystemTime,
};

use super::util_size::get_display_size;
use super::util_msg::print_lastline;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub fn copy_io<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W, total: i64) -> io::Result<u64>
        where R: io::Read, W: io::Write {
    //let written_cell = RefCell::new(0u64);
    let start = SystemTime::now();
    let written = copy_io_callback(reader, writer, total, &|total, written, _len| {
        //written_cell.replace_with(|&mut w| w + len as u64);
        //let written = *written_cell.borrow();
        let cost = SystemTime::now().duration_since(start.clone()).unwrap().as_secs();
        let mut download_speed = "-".to_string();
        if cost > 0 {
            download_speed = format!("{}/s", get_display_size((written / cost) as i64));
        }
        if total > 0 {
            print_lastline(&format!("Downloading, Total: {}, Downloaded: {}, Speed: {}", 
                get_display_size(total),
                get_display_size(written as i64),
                download_speed));
        } else {
            print_lastline(&format!("Downloading, Downloaded: {}, Speed: {}", 
                get_display_size(written as i64),
                download_speed));
        }
    });
    println!();
    written
}

pub fn copy_io_callback<R: ?Sized, W: ?Sized, FCallback>(reader: &mut R, writer: &mut W, total: i64, callback: &FCallback) -> io::Result<u64>
        where R: io::Read,
              W: io::Write,
              FCallback: Fn(i64, u64, usize) -> () {
    let mut written = 0u64;
    let mut buf: [u8; DEFAULT_BUF_SIZE] = [0u8; DEFAULT_BUF_SIZE];
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
        callback(total, written, len);
    }
}
