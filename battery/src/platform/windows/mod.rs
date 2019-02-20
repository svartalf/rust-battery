// https://docs.microsoft.com/en-us/windows/desktop/power/power-management-portal

use std::io;
use std::iter;

use crate::Battery;
use crate::platform::traits::{BatteryManager, BatteryIterator};

mod ffi;
mod device;

pub use self::device::PowerDevice;

#[derive(Debug, Default)]
pub struct PowerManager;

impl PowerManager {
    pub fn iter(&self) -> PowerIterator {
        let inner = match ffi::DeviceIterator::new() {
            Ok(iter) => Some(iter),
            Err(_) => None
        };

        PowerIterator(inner)
    }
}

impl BatteryManager for PowerManager {
    fn refresh(&mut self, battery: &mut Battery) -> io::Result<()> {
        let battery_tag = battery.get_ref().tag().clone();
        let di = ffi::DeviceIterator::new()?;
        let handle = di.prepare_handle()?;
        let device_handle = ffi::DeviceHandle {
            handle,
            tag: battery_tag,
        };
        let device = PowerDevice::try_from(device_handle)?;
        *battery.get_mut_ref() = device;

        Ok(())
    }

}

#[derive(Debug)]
pub struct PowerIterator(Option<ffi::DeviceIterator>);

impl iter::Iterator for PowerIterator {
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

impl BatteryIterator for PowerIterator {}
