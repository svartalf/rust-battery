use std::fmt::Debug;

use crate::Result;
use crate::units::{ElectricPotential, ElectricCurrent, ElectricCharge, ThermodynamicTemperature, Time};

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
    fn voltage(&self) -> ElectricPotential;

    /// kIOPMPSAmperageKey, mA
    fn amperage(&self) -> ElectricCurrent;

    /// kIOPMPSDesignCapacityKey, mAh
    ///
    /// Does not seems to be declared in the documentation anymore.
    fn design_capacity(&self) -> ElectricCharge;

    /// kIOPMPSMaxCapacityKey, mAh
    fn max_capacity(&self) -> ElectricCharge;

    /// kIOPMPSCurrentCapacityKey, mAh
    fn current_capacity(&self) -> ElectricCharge;

    /// kIOPMPSBatteryTemperatureKey
    fn temperature(&self) -> Option<ThermodynamicTemperature>;

    /// kIOPMPSCycleCountKey
    fn cycle_count(&self) -> Option<u32>;

    /// kIOPMPSTimeRemainingKey, minutes
    fn time_remaining(&self) -> Option<Time>;

    /// kIOPMPSManufacturerKey
    fn manufacturer(&self) -> Option<&str>;

    /// kIOPMPSModelKey
    fn device_name(&self) -> Option<&str>;

    /// kIOPMPSSerialKey
    fn serial_number(&self) -> Option<&str>;
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

    fn voltage(&self) -> ElectricPotential {
        (**self).voltage()
    }

    fn amperage(&self) -> ElectricCurrent {
        (**self).amperage()
    }

    fn design_capacity(&self) -> ElectricCharge {
        (**self).design_capacity()
    }

    fn max_capacity(&self) -> ElectricCharge {
        (**self).max_capacity()
    }

    fn current_capacity(&self) -> ElectricCharge {
        (**self).current_capacity()
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        (**self).temperature()
    }

    fn cycle_count(&self) -> Option<u32> {
        (**self).cycle_count()
    }

    fn time_remaining(&self) -> Option<Time> {
        (**self).time_remaining()
    }

    fn manufacturer(&self) -> Option<&str> {
        (**self).manufacturer()
    }

    fn device_name(&self) -> Option<&str> {
        (**self).device_name()
    }

    fn serial_number(&self) -> Option<&str> {
        (**self).serial_number()
    }
}
