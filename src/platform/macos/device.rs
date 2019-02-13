// For keys reference see: https://developer.apple.com/documentation/kernel/iopmpowersource?language=objc
// Additional keys worth to implement later:
//  * "Cycle Count"
//  * "ChargerData" ->
//    - ChargingVoltage
//    - ChargingCurrent
//    - NotChargingReason (?)

use std::io;
use std::str;
use std::time::Duration;

use crate::types::{State, Technology, Device};
use super::iokit;

#[derive(Debug)]
pub struct IoKitDevice{
    source: iokit::PowerSource,

    fully_charged: bool,
    external_connected: bool,
    is_charging: bool,

    voltage: f64,
    amperage: f64,
    temperature: isize,

    design_capacity: f64,
    max_capacity: f64,
    current_capacity: f64,

    manufacturer: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,

    instant_time_to_empty: Option<isize>,
    instant_time_to_full: Option<isize>,
}

impl IoKitDevice {
    pub fn new() -> io::Result<IoKitDevice> {
        let ps = iokit::PowerSource::new()?;

        let fully_charged = ps.get_bool(b"FullyCharged")
            .expect("IOKit is not providing required data");
        let external_connected = ps.get_bool(b"FullyCharged")
            .expect("IOKit is not providing required data");
        let is_charging = ps.get_bool(b"IsCharging")
            .expect("IOKit is not providing required data");

        let voltage = ps.get_f64(b"Voltage")
            .expect("IOKit is not providing required data");
        let amperage = ps.get_f64(b"Amperage")
            .expect("IOKit is not providing required data");
        let design_capacity = ps.get_f64(b"DesignCapacity")
            .expect("IOKit is not providing required data");
        let max_capacity = ps.get_f64(b"MaxCapacity")
            .expect("IOKit is not providing required data");
        let current_capacity = ps.get_f64(b"CurrentCapacity")
            .expect("IOKit is not providing required data");
        let temperature = ps.get_isize(b"Temperature")
            .expect("IOKit is not providing required data");

        let instant_time_to_empty = ps.get_isize(b"InstantTimeToEmpty")
            .and_then(|val| {
                if val == 65535 {
                    None
                } else {
                    Some(val)
                }
            });
        let instant_time_to_full = ps.get_isize(b"InstantTimeToFull")
            .and_then(|val| {
                if val == 65535 {
                    None
                } else {
                    Some(val)
                }
            });

        let manufacturer = ps.get_string(b"Manufacturer");
        let model = ps.get_string(b"DeviceName");
        let serial_number = ps.get_string(b"BatterySerialNumber");

        Ok(IoKitDevice{
            source: ps,

            fully_charged,
            external_connected,
            is_charging,
            voltage,
            amperage,
            design_capacity,
            max_capacity,
            current_capacity,
            temperature,
            manufacturer,
            model,
            serial_number,
            instant_time_to_empty,
            instant_time_to_full,
        })
    }
}

impl Device for IoKitDevice {
    fn capacity(&self) -> f64 {
        (self.energy_full() / self.energy_full_design()) * 100.0
    }

    fn energy(&self) -> f64 {
        self.current_capacity * self.voltage() / 1_000.0
    }

    fn energy_full(&self) -> f64 {
        self.max_capacity * self.voltage() / 1_000.0
    }

    fn energy_full_design(&self) -> f64 {
        self.design_capacity * self.voltage() / 1_000.0
    }

    fn energy_rate(&self) -> f64 {
        self.amperage.abs() * self.voltage() / 1_000.0
    }

    fn percentage(&self) -> f64 {
        100.0 * self.energy() / self.energy_full()
    }

    fn state(&self) -> State {
        match () {
            _ if !self.external_connected => State::Discharging,
            _ if self.is_charging => State::Charging,
            _ if self.current_capacity == 0.0 => State::Empty,
            _ if self.fully_charged => State::Full,
            _ => State::Unknown,
        }
    }

    fn voltage(&self) -> f64 {
        self.voltage / 1_000.0
    }

    fn temperature(&self) -> f64 {
        self.temperature as f64 / 100.0
    }

    fn vendor(&self) -> Option<&str> {
        self.manufacturer.as_ref().map(|v| v.as_ref())
    }

    fn model(&self) -> Option<&str> {
        self.model.as_ref().map(|v| v.as_ref())
    }

    fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_ref().map(|v| v.as_ref())
    }

    fn technology(&self) -> Technology {
        Technology::Unknown
    }

    fn time_to_full(&self) -> Option<Duration> {
        self.instant_time_to_full.and_then(|value| {
            // TODO: Possible `value` invalid cast
            Some(Duration::from_secs(value as u64))
        })
    }

    fn time_to_empty(&self) -> Option<Duration> {
        self.instant_time_to_empty.and_then(|value| {
            // TODO: Possible `value` invalid cast
            Some(Duration::from_secs(value as u64))
        })
    }
}
