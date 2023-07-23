use std::fs::OpenOptions;
use std::io::Write;

/// Read the contents of a file that pertains to brightness control
fn _read_helper(file_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path)?.trim().parse::<u32>()?)
}

/// Dump the contents to a given file and return the number of bytes written
fn _write_helper(file_path: &str, contents: String) -> Result<(), std::io::Error> {
    let mut fd = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    Ok(fd.write_all(contents.as_bytes()).unwrap())
}

pub fn current_brightness_level(file_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    _read_helper(file_path)
}

pub fn max_brightness_level(file_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    _read_helper(file_path)
}

pub fn set_brightness(
    value: u32,
    brightness_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_brightness = max_brightness_level(&format!("{brightness_path}/max_brightness"))?;
    if value > max_brightness {
        return Ok(());
    }

    Ok(_write_helper(&format!("{brightness_path}/brightness"), format!("{value}"))?)
}
