use std::io;
use std::iter;
use std::default;

use crate::Battery;
use crate::platform::traits::{BatteryManager, BatteryIterator};
pub use self::device::IoKitDevice;

mod iokit;
mod device;
mod traits;

#[derive(Debug)]
pub struct IoKitManager(iokit::IoMasterPort);

impl IoKitManager {

    pub fn iter(&self) -> IoKitIterator {
        let inner = self.0.get_services()
            .expect("Unable to get I/OKit matching services");

        IoKitIterator(inner)
    }
}

impl default::Default for IoKitManager {
    fn default() -> IoKitManager {
        // For now all internal errors are panicking,
        // but they will be propagated later in future
        let master_port = iokit::IoMasterPort::new()
            .expect("Unable to create I/OKit master port");

        IoKitManager(master_port)
    }
}

impl BatteryManager for IoKitManager {
    fn refresh(&mut self, battery: &mut Battery) -> io::Result<()> {
        battery.get_mut_ref().get_mut_ref().refresh()?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct IoKitIterator(iokit::IoIterator);

impl iter::Iterator for IoKitIterator {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(io_obj) => {
                // It is like a four levels of abstraction now.
                // Battery -> IoKitDevice -> PowerSource -> IoObject
                // What have I done!
                let ps = iokit::PowerSource::from(io_obj);
                let device = IoKitDevice::from(ps);
                Some(Battery::from(device))
            }
        }
    }
}

impl BatteryIterator for IoKitIterator {}

#[cfg(test)]
mod tests;
