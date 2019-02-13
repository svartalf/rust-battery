use std::iter;

use crate::Battery;
pub use self::device::IoKitDevice;

mod iokit;
mod device;

#[derive(Debug)]
pub struct IoKitDiscovery(Option<device::IoKitDevice>);

impl IoKitDiscovery {
    pub fn new() -> IoKitDiscovery {
        let inner = match device::IoKitDevice::new() {
            Ok(device) => Some(device),
            Err(_) => None,
        };

        IoKitDiscovery(inner)
    }
}

impl iter::Iterator for IoKitDiscovery {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(device) => Some(Battery::from(device)),
            None => None,
        }
    }
}
