use std::io;
use std::iter;
use std::process;

mod device;

use crate::Battery;
pub use self::device::IoRegDevice;

pub struct IoReg(Vec<IoRegDevice>);

impl IoReg {
    pub fn new() -> IoReg {
        match IoReg::get_plist() {
            Ok(devices) => IoReg(devices),
            Err(_) => IoReg(vec![]),
        }
    }

    fn get_plist() -> io::Result<Vec<IoRegDevice>> {
        let output = process::Command::new("ioreg")
            .args(&["-n", "AppleSmartBattery", "-r", "-a"])
            // `ioreg -n AppleSmartBattery -r -a`
            .output()?;
        let cursor = io::Cursor::new(output.stdout);

        plist::from_reader_xml(cursor)
            .map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, e)
            })
    }
}

impl iter::Iterator for IoReg {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop() {
            Some(device) => Some(Battery::from(device)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
