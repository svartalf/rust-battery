use std::io;
use std::fs;
use std::path::Path;


pub fn get_string<T: AsRef<Path>>(path: T) -> io::Result<String> {
    match fs::read_to_string(path) {
        Err(e) => Err(e),
        Ok(ref content) => {
            let trimmed = content.trim();
            if trimmed.starts_with('\0') {
                Err(io::Error::from(io::ErrorKind::InvalidData))
            } else {
                Ok(trimmed.to_string())
            }
        }
    }
}

// TODO: Generic somehow?

pub fn get_f32<T: AsRef<Path>>(path: T) -> io::Result<f32> {
    get_string(path).and_then(|value| {
        value.parse::<f32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })
}

pub fn get_u32<T: AsRef<Path>>(path: T) -> io::Result<u32> {
    get_string(path).and_then(|value| {
        value.parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })
}
