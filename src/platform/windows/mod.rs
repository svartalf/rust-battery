// https://docs.microsoft.com/en-us/windows/desktop/power/power-management-portal

use std::iter;

use crate::{Battery};

mod ffi;
mod device;

pub use self::device::PowerDevice;

#[derive(Debug)]
pub struct PowerManager(Option<ffi::DeviceIterator>);

impl PowerManager {
    pub fn new() -> PowerManager {
        let inner = match ffi::DeviceIterator::new() {
            Ok(iter) => Some(iter),
            Err(_) => None
        };
        PowerManager(inner)
    }
}

impl iter::Iterator for PowerManager {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = match self.0 {
            None => return None,
            Some(ref mut inner) => inner,
        };

        loop {
            match inner.next() {
                None => return None,
                Some(handle) => {
                    match PowerDevice::try_from(handle) {
                        Ok(device) => return Some(Battery::from(device)),
                        Err(_) => continue,
                    }
                }
            }
        }
    }
}
