use std::fmt::Debug;
use std::time::Duration;

use super::iokit::Result;

/// Used for IOPMPowerSource wrapper and for tests.
///
/// Only keys declared at https://developer.apple.com/documentation/kernel/iopmpowersource?language=objc
/// should be used in this trait and trait implementors, otherwise bugs might happen
/// as in [#11](https://github.com/svartalf/rust-battery/pull/11)
pub trait DataSource: Debug + 'static {
    fn refresh(&mut self) -> Result<()>;

    /// kIOPMFullyChargedKey
    ///
    /// Does not seems to be declared in the documentation anymore.
    fn fully_charged(&self) -> bool;

    /// kIOPMPSExternalConnectedKey
    fn external_connected(&self) -> bool;

    /// kIOPMPSIsChargingKey
    fn is_charging(&self) -> bool;

    /// kIOPMPSVoltageKey, mV
    fn voltage(&self) -> u32;

    /// kIOPMPSAmperageKey, mA
    fn amperage(&self) -> i32;

    /// kIOPMPSDesignCapacityKey, mAh
    ///
    /// Does not seems to be declared in the documentation anymore.
    fn design_capacity(&self) -> u32;

    /// kIOPMPSMaxCapacityKey, mAh
    fn max_capacity(&self) -> u32;

    /// kIOPMPSCurrentCapacityKey, mAh
    fn current_capacity(&self) -> u32;

    /// kIOPMPSBatteryTemperatureKey
    fn temperature(&self) -> Option<f32>;

    /// kIOPMPSCycleCountKey
    fn cycle_count(&self) -> Option<u32>;

    /// kIOPMPSTimeRemainingKey, minutes
    fn time_remaining(&self) -> Option<Duration>;

    /// kIOPMPSManufacturerKey
    fn manufacturer(&self) -> Option<String>;

    /// kIOPMPSModelKey
    fn device_name(&self) -> Option<String>;

    /// kIOPMPSSerialKey
    fn serial_number(&self) -> Option<String>;
}


impl<T> DataSource for Box<T> where T: DataSource + ?Sized {
    fn refresh(&mut self) -> Result<()> {
        (**self).refresh()
    }

    fn fully_charged(&self) -> bool {
        (**self).fully_charged()
    }

    fn external_connected(&self) -> bool {
        (**self).external_connected()
    }

    fn is_charging(&self) -> bool {
        (**self).is_charging()
    }

    fn voltage(&self) -> u32 {
        (**self).voltage()
    }

    fn amperage(&self) -> i32 {
        (**self).amperage()
    }

    fn design_capacity(&self) -> u32 {
        (**self).design_capacity()
    }

    fn max_capacity(&self) -> u32 {
        (**self).max_capacity()
    }

    fn current_capacity(&self) -> u32 {
        (**self).current_capacity()
    }

    fn temperature(&self) -> Option<f32> {
        (**self).temperature()
    }

    fn cycle_count(&self) -> Option<u32> {
        (**self).cycle_count()
    }

    fn time_remaining(&self) -> Option<Duration> {
        (**self).time_remaining()
    }

    fn manufacturer(&self) -> Option<String> {
        (**self).manufacturer()
    }

    fn device_name(&self) -> Option<String> {
        (**self).device_name()
    }

    fn serial_number(&self) -> Option<String> {
        (**self).serial_number()
    }
}
