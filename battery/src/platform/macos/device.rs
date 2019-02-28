// For keys reference see: https://developer.apple.com/documentation/kernel/iopmpowersource?language=objc
// Additional keys worth to implement later:
//  * "ChargerData" ->
//    - ChargingVoltage
//    - ChargingCurrent
//    - NotChargingReason (?)

use std::str;
use std::boxed::Box;
use std::convert::AsRef;
use std::time::Duration;

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

// Note about `mWh` values calculation, used in `energy`, `energy_full` and `energy_full_design`
// method, which caused https://github.com/svartalf/rust-battery/issues/8 bug
//
// Formula: mWh = mAh * V
//
// But `self.source.voltage()` returns `mV`, not the `V` units.
impl BatteryDevice for IoKitDevice {
    fn energy(&self) -> u32 {
        let voltage = self.source.voltage() as f32 / 1_000.0;  // V units

        (self.source.current_capacity() as f32 * voltage) as u32
    }

    fn energy_full(&self) -> u32 {
        let voltage = self.source.voltage() as f32 / 1_000.0;  // V units

        (self.source.max_capacity() as f32 * voltage) as u32
    }

    fn energy_full_design(&self) -> u32 {
        let voltage = self.source.voltage() as f32 / 1_000.0;  // V units

        (self.source.design_capacity() as f32 * voltage) as u32
    }

    fn energy_rate(&self) -> u32 {
        let voltage = self.source.voltage() as f32 / 1_000.0;  // V units

        (self.source.amperage().abs() as f32 * voltage) as u32
    }

    fn percentage(&self) -> f32 {
        100.0 * ((self.energy() as f32) / (self.energy_full() as f32))
    }

    fn state(&self) -> State {
        match () {
            _ if !self.source.external_connected() => State::Discharging,
            _ if self.source.is_charging() => State::Charging,
            _ if self.source.current_capacity() == 0 => State::Empty,
            _ if self.source.fully_charged() => State::Full,
            _ => State::Unknown,
        }
    }

    fn voltage(&self) -> u32 {
        self.source.voltage()
    }

    fn temperature(&self) -> Option<f32> {
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

    fn time_to_full(&self) -> Option<Duration> {
        if self.state() == State::Charging {
            self.source.time_remaining()
        } else {
            None
        }
    }

    fn time_to_empty(&self) -> Option<Duration> {
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
