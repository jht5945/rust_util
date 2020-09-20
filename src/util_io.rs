use std::{
    fs::File,
    io::{ self, ErrorKind, prelude::* },
    time::{ SystemTime, Duration },
};

use crate::{ XResult, new_box_ioerror };
use crate::util_size;
use crate::util_msg;
use crate::util_file;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub struct PrintStatusContext {
    pub print_interval_time: Duration,
    pub print_interval_bytes: i64,
    pub init_print_time: SystemTime,
    pub last_print_time: SystemTime,
    pub total_written_bytes: i64,
}

impl PrintStatusContext {
    pub fn new() -> Self {
        Self::new_with(Duration::from_millis(100), 512 * 1024)
    }

    pub fn new_with(print_interval_time: Duration, print_interval_bytes: i64) -> Self {
        Self {
            print_interval_time,
            print_interval_bytes,
            init_print_time: SystemTime::now(),
            last_print_time: SystemTime::now(),
            total_written_bytes: 0,
        }
    }

    pub fn check_print(&mut self, total: i64, written: i64) -> (bool, Duration) {
        let now = SystemTime::now();
        let total_cost = now.duration_since(self.init_print_time).unwrap_or_else(|_| Duration::from_millis(0));
        let last_print_cost = now.duration_since(self.last_print_time).unwrap_or_else(|_| Duration::from_millis(0));
        let should_update_status_line = || {
            if total > written && (total - written < self.print_interval_bytes) {
                return true;
            }
            if written > self.total_written_bytes && (written - self.total_written_bytes > self.print_interval_bytes) {
                return true;
            }
            match last_print_cost.as_millis() {
                m if m > self.print_interval_time.as_millis() => true,
                _ => false,
            }
        };
        if should_update_status_line() {
            self.last_print_time = now;
            self.total_written_bytes = written;
            (true, total_cost)
        } else {
            (false, total_cost)
        }
    }
}

impl Default for PrintStatusContext {
    fn default() -> Self {
        PrintStatusContext::new()
    }
}

pub fn get_read_stdin_or_file(file: &str) -> XResult<Box<dyn Read>> {
    if file.is_empty() {
        Ok(Box::new(io::stdin()))
    } else {
        match File::open(&util_file::resolve_file_path(file)) {
            Ok(f) => Ok(Box::new(f)),
            Err(err) => Err(new_box_ioerror(&format!("Open file {}, erorr: {}", file, err))),
        }
    }
}

pub fn read_to_string(read: &mut dyn Read) -> XResult<String> {
    let mut buffer = String::new();
    read.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn read_to_bytes(read: &mut dyn Read) -> XResult<Vec<u8>> {
    let mut buffer = vec![];
    read.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn copy_io_default<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W, total: i64) -> io::Result<u64>
        where R: io::Read, W: io::Write {
    copy_io_with_head(reader, writer, total, "Downloading", &mut PrintStatusContext::default())
}

pub fn copy_io<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W, total: i64, print_status_context: &mut PrintStatusContext)
        -> io::Result<u64>
        where R: io::Read, W: io::Write {
    copy_io_with_head(reader, writer, total, "Downloading", print_status_context)
}

pub fn copy_io_with_head<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W, total: i64, head: &str, print_status_context: &mut PrintStatusContext) -> io::Result<u64>
        where R: io::Read, W: io::Write {
    let written = copy_io_callback(reader, writer, total, print_status_context, &mut |total, written, _len, print_status_context| {
        print_status_last_line(head, total, written as i64, print_status_context);
    });
    println!();
    written
}

pub fn copy_io_callback<R: ?Sized, W: ?Sized, FCallback>(reader: &mut R, writer: &mut W, total: i64, print_status_context: &mut PrintStatusContext, callback: &mut FCallback) -> io::Result<u64>
        where R: io::Read,
              W: io::Write,
              FCallback: Fn(i64, u64, usize, &mut PrintStatusContext) {
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
        callback(total, written, len, print_status_context);
    }
}

pub fn print_status_last_line(head: &str, total: i64, written: i64, print_status_context: &mut PrintStatusContext) {
    let mut download_speed = "-".to_string();
    let (is_print, cost) = print_status_context.check_print(total, written);
    if !is_print {
        return;
    }
    let cost_as_secs = cost.as_secs();
    if cost_as_secs > 0 {
        download_speed = format!("{}/s", util_size::get_display_size((written / (cost_as_secs as i64)) as i64));
    }
    if total > 0 {
        util_msg::print_lastline(&format!("{}, Total: {}, Finished: {}, Speed: {}",
            head,
            util_size::get_display_size(total),
            util_size::get_display_size(written),
            download_speed));
    } else {
        util_msg::print_lastline(&format!("{}, Finished: {}, Speed: {}",
            head,
            util_size::get_display_size(written),
            download_speed));
    }
}
