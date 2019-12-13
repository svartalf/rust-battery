use std::fmt;
use std::ops::Deref;

use super::{iokit, IoKitIterator};
use crate::platform::traits::{BatteryIterator, BatteryManager};
use crate::Result;

pub struct IoKitManager(iokit::IoMasterPort);

impl BatteryManager for IoKitManager {
    type Iterator = IoKitIterator;

    fn new() -> Result<Self> {
        let port = iokit::IoMasterPort::new()?;

        Ok(Self(port))
    }

    fn refresh(&self, device: &mut <Self::Iterator as BatteryIterator>::Device) -> Result<()> {
        device.refresh()
    }
}

impl Deref for IoKitManager {
    type Target = iokit::IoMasterPort;

    fn deref(&self) -> &iokit::IoMasterPort {
        &self.0
    }
}

impl fmt::Debug for IoKitManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MacOSManager").field("io_master_port", &self.0).finish()
    }
}
