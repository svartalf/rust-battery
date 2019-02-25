// For keys reference see: https://developer.apple.com/documentation/kernel/iopmpowersource?language=objc
// Additional keys worth to implement later:
//  * "Cycle Count"
//  * "ChargerData" ->
//    - ChargingVoltage
//    - ChargingCurrent
//    - NotChargingReason (?)

use std::io;
use std::str;
use std::convert::AsRef;
use std::time::Duration;

use crate::types::{State, Technology};
use crate::platform::traits::BatteryDevice;
use super::iokit;

#[derive(Debug)]
pub struct IoKitDevice{
    source: iokit::PowerSource,

    fully_charged: bool,
    external_connected: bool,
    is_charging: bool,

    voltage: u32, // mV
    amperage: u32, // mA
    temperature: Option<f32>,
    cycle_count: Option<u32>,

    design_capacity: u32, // mAh
    max_capacity: u32, // mAh
    current_capacity: u32,  // mAh

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
        let external_connected = ps.get_bool(b"ExternalConnected")
            .expect("IOKit is not providing required data");
        let is_charging = ps.get_bool(b"IsCharging")
            .expect("IOKit is not providing required data");

        let voltage = ps.get_u32(b"Voltage")
            .expect("IOKit is not providing required data");
        let amperage = ps.get_isize(b"Amperage")
            .expect("IOKit is not providing required data")
            .abs() as u32;
        let design_capacity = ps.get_u32(b"DesignCapacity")
            .expect("IOKit is not providing required data");
        let max_capacity = ps.get_u32(b"MaxCapacity")
            .expect("IOKit is not providing required data");
        let current_capacity = ps.get_u32(b"CurrentCapacity")
            .expect("IOKit is not providing required data");
        let temperature = ps.get_isize(b"Temperature")
            .map(|value| value as f32 / 100.0);
        let cycle_count = ps.get_u32(b"CycleCount");
        let instant_time_to_empty = ps.get_isize(b"InstantTimeToEmpty")
            .and_then(|val| {
                if val == 65535 {
                    None
                } else {
                    Some(val * 60)
                }
            });
        let instant_time_to_full = ps.get_isize(b"InstantTimeToFull")
            .and_then(|val| {
                if val == 65535 {
                    None
                } else {
                    Some(val * 60)
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
            temperature,
            cycle_count,
            design_capacity,
            max_capacity,
            current_capacity,
            manufacturer,
            model,
            serial_number,
            instant_time_to_empty,
            instant_time_to_full,
        })
    }
}

impl BatteryDevice for IoKitDevice {
    fn energy(&self) -> u32 {
        self.current_capacity * self.voltage
    }

    fn energy_full(&self) -> u32 {
        self.max_capacity * self.voltage
    }

    fn energy_full_design(&self) -> u32 {
        self.design_capacity * self.voltage
    }

    fn energy_rate(&self) -> u32 {
        self.amperage * self.voltage
    }

    fn percentage(&self) -> f32 {
        self.current_capacity as f32 / self.max_capacity as f32 * 100.0
    }

    fn state(&self) -> State {
        match () {
            _ if !self.external_connected => State::Discharging,
            _ if self.is_charging => State::Charging,
            _ if self.current_capacity == 0 => State::Empty,
            _ if self.fully_charged => State::Full,
            _ => State::Unknown,
        }
    }

    fn voltage(&self) -> u32 {
        self.voltage
    }

    fn temperature(&self) -> Option<f32> {
        self.temperature
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
        self.cycle_count
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
