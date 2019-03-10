use std::fmt;
use std::ops::Deref;
use std::os::unix::io::AsRawFd;

use crate::{Result, Error};
use crate::platform::traits::{BatteryManager, BatteryIterator};
use super::{acpi, IoCtlIterator};

pub struct IoCtlManager(acpi::AcpiDevice);

impl BatteryManager for IoCtlManager {
    type Iterator = IoCtlIterator;

    fn new() -> Result<Self> {
        Ok(Self(acpi::AcpiDevice::new()?))
    }

    fn refresh(&self, device: &mut <Self::Iterator as BatteryIterator>::Device) -> Result<()> {
        let bif = self.0.bif(device.unit())?;
        let bst = self.0.bst(device.unit())?;

        match (bif, bst) {
            (Some(bif), Some(bst)) => device.refresh(bif, bst),
            (None, _) => Err(Error::invalid_data("Returned bif struct is invalid")),
            (_, None) => Err(Error::invalid_data("Returned bst struct is invalid")),
        }
    }
}

impl Deref for IoCtlManager {
    type Target = acpi::AcpiDevice;

    fn deref(&self) -> &acpi::AcpiDevice {
        &self.0
    }
}

impl fmt::Debug for IoCtlManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FreeBSD")
            .field("fd", &self.0.as_raw_fd())
            .finish()
    }
}
