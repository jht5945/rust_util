use super::XResult;

pub const SIZE_KB: i64 = 1024;
pub const SIZE_MB: i64 = SIZE_KB * SIZE_KB;
pub const SIZE_GB: i64 = SIZE_MB * SIZE_KB;
pub const SIZE_PB: i64 = SIZE_GB * SIZE_KB;
pub const SIZE_TB: i64 = SIZE_PB * SIZE_KB;


pub fn parse_size(size: &str) -> XResult<i64> {
    let lower_size = size.to_lowercase();
    let no_last_b_size = if lower_size.ends_with("b") {
        &lower_size[0..lower_size.len()-1]
    } else {
        &lower_size
    };
    if no_last_b_size.ends_with("k") {
        return Ok((SIZE_KB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("m") {
        return Ok((SIZE_MB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("g") {
        return Ok((SIZE_GB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("t") {
        return Ok((SIZE_TB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    } else if no_last_b_size.ends_with("p") {
        return Ok((SIZE_PB as f64 * no_last_b_size[0..no_last_b_size.len()-1].parse::<f64>()?) as i64);
    }

    Ok(no_last_b_size.parse::<i64>()?)
}

pub fn get_display_size(size: i64) -> String {
    if size < SIZE_KB {
        return size.to_string();
    } else if size < SIZE_MB {
        return format!("{:.*}KB", 2, (size as f64) / 1024.);
    } else if size < SIZE_GB {
        return format!("{:.*}MB", 2, (size as f64) / 1024. / 1024.);
    } else if size < SIZE_TB {
        return format!("{:.*}GB", 2, (size as f64) / 1024. / 1024. / 1024.);
    } else if size < SIZE_PB {
        return format!("{:.*}TB", 2, (size as f64) / 1024. / 1024. / 1024. / 1024.);
    } else {
        return format!("{:.*}PB", 2, (size as f64) / 1024. / 1024. / 1024. / 1024. / 1024.);
    }
}
