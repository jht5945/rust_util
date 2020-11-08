use crate::XResult;

pub const SIZE_KB: i64 = 1024;
pub const SIZE_MB: i64 = SIZE_KB * SIZE_KB;
pub const SIZE_GB: i64 = SIZE_MB * SIZE_KB;
pub const SIZE_TB: i64 = SIZE_GB * SIZE_KB;
pub const SIZE_PB: i64 = SIZE_TB * SIZE_KB;


pub fn parse_size(size: &str) -> XResult<i64> {
    let lower_size = size.to_lowercase();
    let no_last_b_size = crate::iff!(
        lower_size.ends_with('b'), &lower_size[0..lower_size.len()-1], &lower_size
    );
    let parse_and_process_size = |mul: i64| -> XResult<i64> {
        Ok((mul as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64)
    };
    match no_last_b_size.chars().last() {
        Some('k') => parse_and_process_size(SIZE_KB),
        Some('m') => parse_and_process_size(SIZE_MB),
        Some('g') => parse_and_process_size(SIZE_GB),
        Some('t') => parse_and_process_size(SIZE_TB),
        Some('p') => parse_and_process_size(SIZE_PB),
        _ => Ok(no_last_b_size.parse::<i64>()?),
    }
}

pub fn get_display_size(size: i64) -> String {
    if size < SIZE_KB {
        size.to_string()
    } else if size < SIZE_MB {
        format!("{:.*}KB", 2, (size as f64) / SIZE_KB as f64)
    } else if size < SIZE_GB {
        format!("{:.*}MB", 2, (size as f64) / SIZE_MB as f64)
    } else if size < SIZE_TB {
        format!("{:.*}GB", 2, (size as f64) / SIZE_GB as f64)
    } else if size < SIZE_PB {
        format!("{:.*}TB", 2, (size as f64) / SIZE_TB as f64)
    } else {
        format!("{:.*}PB", 2, (size as f64) / SIZE_PB as f64)
    }
}


#[test]
fn test_parse_size() {
    assert_eq!(parse_size("1").unwrap(), 1);
    assert_eq!(parse_size("1k").unwrap(), 1024);
    assert_eq!(parse_size("1m").unwrap(), 1024 * 1024);
    assert_eq!(parse_size("1g").unwrap(), 1024 * 1024 * 1024);
    assert_eq!(parse_size("1t").unwrap(), 1024 * 1024 * 1024 * 1024);
    assert_eq!(parse_size("1p").unwrap(), 1024 * 1024 * 1024 * 1024 * 1024);
}

#[test]
fn test_get_display_size() {
    assert_eq!(get_display_size(0), "0");
    assert_eq!(get_display_size(111), "111");
    assert_eq!(get_display_size(1024), "1.00KB");
    assert_eq!(get_display_size(1024 * 1024), "1.00MB");
    assert_eq!(get_display_size(1024 * 1024 * 1024), "1.00GB");
    assert_eq!(get_display_size(1024 * 1024 * 1024 * 1024), "1.00TB");
    assert_eq!(get_display_size(1024 * 1024 * 1024 * 1024 * 1024), "1.00PB");
}