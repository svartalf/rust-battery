use std::io;

use crate::Battery;
use crate::platform::traits::BatteryManager;
use super::IoCtlDevice;
use super::IoCtlIterator;

#[derive(Debug, Default)]
pub struct IoCtlManager;

impl IoCtlManager {
    pub fn iter(&self) -> IoCtlIterator {
        let inner = match IoCtlDevice::new() {
            Ok(device) => Some(device),
            Err(_) => None,
        };

        IoCtlIterator(inner)
    }
}

impl BatteryManager for IoCtlManager {
    fn refresh(&mut self, battery: &mut Battery) -> io::Result<()> {
        *battery.get_mut_ref() = IoCtlDevice::new()?;

        Ok(())
    }
}

