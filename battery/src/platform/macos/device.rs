// For keys reference see: https://developer.apple.com/documentation/kernel/iopmpowersource?language=objc
// Additional keys worth to implement later:
//  * "ChargerData" ->
//    - ChargingVoltage
//    - ChargingCurrent
//    - NotChargingReason (?)

use std::str;
use std::boxed::Box;
use std::convert::AsRef;
use num_traits::identities::Zero;


use crate::units::{ElectricPotential, ThermodynamicTemperature, Time, Power, Energy};
use crate::types::{State, Technology};
use crate::platform::traits::BatteryDevice;
use super::traits::DataSource;

#[derive(Debug)]
pub struct IoKitDevice {
    source: Box<dyn DataSource>,

    manufacturer: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
}

impl IoKitDevice {
    pub fn get_mut_ref(&mut self) -> &mut dyn DataSource {
        &mut self.source
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
        self.manufacturer.as_ref().map(AsRef::as_ref)
    }

    fn model(&self) -> Option<&str> {
        self.model.as_ref().map(AsRef::as_ref)
    }

    fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_ref().map(AsRef::as_ref)
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
        let manufacturer = ds.manufacturer();
        let model = ds.device_name();
        let serial_number = ds.serial_number();

        IoKitDevice {
            source: Box::new(ds),

            manufacturer,
            model,
            serial_number,
        }
    }
}
