use std::u16;
use std::time::Duration;

use crate::{Technology, State};
use crate::types::Device;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IoRegDevice {
    amperage: i32,
    current_capacity: u32,
    max_capacity: u32,
    design_capacity: u32,

    external_connected: bool,
    is_charging: bool,
    fully_charged: bool,

    temperature: u16,
    avg_time_to_empty: u16,
    avg_time_to_full: u16,

    device_name: String,
    battery_serial_number: String,

    voltage: u32,
}

impl Device for IoRegDevice {
    fn capacity(&self) -> f64 {
        (self.energy_full() / self.energy_full_design()) * 100.0
    }

    fn energy(&self) -> f64 {
        f64::from(self.current_capacity) * self.voltage() / 1_000.0
    }

    fn energy_full(&self) -> f64 {
        f64::from(self.max_capacity) * self.voltage() / 1_000.0
    }

    fn energy_full_design(&self) -> f64 {
        f64::from(self.design_capacity) * self.voltage() / 1_000.0
    }

    fn energy_rate(&self) -> f64 {
        f64::from(self.amperage).abs() * self.voltage() / 1_000.0
    }

    fn percentage(&self) -> f64 {
        100.0 * self.energy() / self.energy_full()
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

    fn voltage(&self) -> f64 {
        f64::from(self.voltage) / 1000.0
    }

    fn temperature(&self) -> f64 {
        f64::from(self.temperature) / 100.0
    }

    fn vendor(&self) -> Option<&str> {
        // TODO: it seems that ioreg output is missing manufacturer?
        None
    }

    fn model(&self) -> Option<&str> {
        Some(self.device_name.as_ref())
    }

    fn serial_number(&self) -> Option<&str> {
        Some(self.battery_serial_number.as_ref())
    }

    fn technology(&self) -> Technology {
        // TODO: it seems that ioreg does not provide battery technology field?
        Technology::Unknown
    }

    fn time_to_full(&self) -> Option<Duration> {
        if self.avg_time_to_full == u16::MAX {
            None
        } else {
            Some(Duration::from_secs(u64::from(self.avg_time_to_full)))
        }
    }

    fn time_to_empty(&self) -> Option<Duration> {
        if self.avg_time_to_empty == u16::MAX {
            None
        } else {
            Some(Duration::from_secs(u64::from(self.avg_time_to_empty)))
        }
    }
}
