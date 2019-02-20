use std::io;
use std::iter;
use std::default;

use crate::Battery;
use crate::platform::traits::{BatteryManager, BatteryIterator};
pub use self::device::IoKitDevice;

mod iokit;
mod device;

#[derive(Debug, Default)]
pub struct IoKitManager;

impl IoKitManager {
    pub fn iter(&self) -> IoKitIterator {
        let inner = match device::IoKitDevice::new() {
            Ok(device) => Some(device),
            Err(_) => None,
        };

        IoKitIterator(inner)
    }
}

impl BatteryManager for IoKitManager {
    fn refresh(&mut self, battery: &mut Battery) -> io::Result<()> {
        let inner = device::IoKitDevice::new()?;
        *battery.get_mut_ref() = inner;

        Ok(())
    }
}

#[derive(Debug)]
pub struct IoKitIterator(Option<device::IoKitDevice>);

impl iter::Iterator for IoKitIterator {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(device) => Some(Battery::from(device)),
            None => None,
        }
    }
}

impl BatteryIterator for IoKitIterator {}
