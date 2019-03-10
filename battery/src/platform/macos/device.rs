// For keys reference see: https://developer.apple.com/documentation/kernel/iopmpowersource?language=objc
// Additional keys worth to implement later:
//  * "ChargerData" ->
//    - ChargingVoltage
//    - ChargingCurrent
//    - NotChargingReason (?)

use std::fmt;
use std::str;
use std::boxed::Box;
use num_traits::identities::Zero;

use crate::Result;
use crate::units::{ElectricPotential, ThermodynamicTemperature, Time, Power, Energy};
use crate::types::{State, Technology};
use crate::platform::traits::BatteryDevice;
use super::traits::DataSource;

pub struct IoKitDevice {
    source: Box<dyn DataSource>,
}

impl IoKitDevice {
    pub fn get_mut_ref(&mut self) -> &mut dyn DataSource {
        &mut self.source
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.source.refresh()
    }
}

impl BatteryDevice for IoKitDevice {
    fn energy(&self) -> Energy {
        self.source.current_capacity() * self.source.voltage()
    }

    fn energy_full(&self) -> Energy {
        self.source.max_capacity() * self.source.voltage()
    }

    fn energy_full_design(&self) -> Energy {
        self.source.design_capacity() * self.source.voltage()
    }

    fn energy_rate(&self) -> Power {
        self.source.amperage() * self.source.voltage()
    }

    fn state(&self) -> State {
        match () {
            _ if !self.source.external_connected() => State::Discharging,
            _ if self.source.is_charging() => State::Charging,
            _ if self.source.current_capacity().is_zero() => State::Empty,
            _ if self.source.fully_charged() => State::Full,
            _ => State::Unknown,
        }
    }

    fn voltage(&self) -> ElectricPotential {
        self.source.voltage()
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.source.temperature()
    }

    fn vendor(&self) -> Option<&str> {
        self.source.manufacturer()
    }

    fn model(&self) -> Option<&str> {
        self.source.device_name()
    }

    fn serial_number(&self) -> Option<&str> {
        self.source.serial_number()
    }

    fn technology(&self) -> Technology {
        Technology::Unknown
    }

    fn cycle_count(&self) -> Option<u32> {
        self.source.cycle_count()
    }

    fn time_to_full(&self) -> Option<Time> {
        if self.state() == State::Charging {
            self.source.time_remaining()
        } else {
            None
        }
    }

    fn time_to_empty(&self) -> Option<Time> {
        if self.state() == State::Discharging {
            self.source.time_remaining()
        } else {
            None
        }
    }
}

impl<T> From<T> for IoKitDevice where T: DataSource {
    fn from(ds: T) -> IoKitDevice {
        IoKitDevice {
            source: Box::new(ds),
        }
    }
}

impl fmt::Debug for IoKitDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MacOSDevice")
            .field("source", &self.source)
            .finish()
    }
}
