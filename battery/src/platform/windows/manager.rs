use std::fmt;

use crate::Result;
use crate::platform::traits::BatteryManager;
use super::{ffi, PowerIterator, PowerDevice};


#[derive(Default)]
pub struct PowerManager;

impl BatteryManager for PowerManager {
    type Iterator = PowerIterator;

    fn new() -> Result<Self> {
        Ok(PowerManager{})
    }

    fn refresh(&self, device: &mut PowerDevice) -> Result<()> {
        let battery_tag = device.tag().clone();
        let di = ffi::DeviceIterator::new()?;
        let handle = di.prepare_handle()?;
        let device_handle = ffi::DeviceHandle {
            handle,
            tag: battery_tag,
        };
        device.refresh(device_handle)?;

        Ok(())
    }
}

impl fmt::Debug for PowerManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WindowsManager")
            .finish()
    }
}
