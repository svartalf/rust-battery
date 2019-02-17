use std::io;
use std::time::Duration;

use crate::{State, Technology};
use crate::types::Device;
use super::ffi::DeviceHandle;

#[derive(Debug)]
pub struct PowerDevice {
    technology: Technology,
    state: State,
    voltage: f64,
    capacity: f64,
    energy_rate: f64,
    design_capacity: u64,
    full_charged_capacity: u64,
    temperature: f64,
    device_name: Option<String>,
    manufacturer: Option<String>,
    serial_number: Option<String>,
}

impl PowerDevice {

    pub fn try_from(mut handle: DeviceHandle) -> io::Result<PowerDevice> {
        let info = handle.information()?;
        if info.is_relative() {
            // We can't support batteries with relative data so far
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        let status = handle.status()?;
        let device_name = match handle.device_name() {
            Ok(name) => Some(name),
            Err(_) => None,
        };
        let manufacturer = match handle.manufacture_name() {
            Ok(name) => Some(name),
            Err(_) => None,
        };
        let serial_number = match handle.serial_number() {
            Ok(value) => Some(value),
            Err(_) => None,
        };
        let rate = match status.rate() {
            None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
            Some(value) => value,
        };
        let capacity = match status.capacity() {
            None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
            Some(value) => value,
        };
        let voltage = match status.capacity() {
            None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
            Some(value) => value,
        };
        // TODO: Handle temperature errors
        let temperature = handle.temperature().unwrap_or(0.0);

        Ok(PowerDevice {
            technology: info.technology(),
            state: status.state(),
            energy_rate: rate,
            design_capacity: info.designed_capacity(),
            full_charged_capacity: info.full_charged_capacity(),
            capacity,
            voltage,
            temperature,
            device_name,
            manufacturer,
            serial_number,
        })
    }

}

impl Device for PowerDevice {
    fn capacity(&self) -> f64 {
        (self.energy_full() / self.energy_full_design()) * 100.0
    }

    fn energy(&self) -> f64 {
        self.capacity
    }

    fn energy_full(&self) -> f64 {
        self.full_charged_capacity as f64
    }

    fn energy_full_design(&self) -> f64 {
        self.design_capacity as f64
    }

    fn energy_rate(&self) -> f64 {
        0.0
    }

    fn percentage(&self) -> f64 {
        set_bounds(100.0 * self.energy() / self.energy_full())
    }

    fn state(&self) -> State {
        self.state
    }

    fn voltage(&self) -> f64 {
        self.voltage
    }

    fn temperature(&self) -> f64 {
        self.temperature
    }

    fn vendor(&self) -> Option<&str> {
        self.manufacturer.as_ref().map(|v| v.as_ref())
    }

    fn model(&self) -> Option<&str> {
        self.device_name.as_ref().map(|v| v.as_ref())
    }

    fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_ref().map(|v| v.as_ref())
    }

    fn technology(&self) -> Technology {
        self.technology
    }

    fn time_to_full(&self) -> Option<Duration> {
        match self.state() {
            State::Charging => {
                let time_to_full = 3600.0 * (self.energy_full() - self.energy()) / self.energy_rate();
                if time_to_full > (20.0 * 60.0 * 60.0) {
                    None
                } else {
                    Some(Duration::from_secs(time_to_full as u64))
                }
            },
            _ => None,
        }
    }

    fn time_to_empty(&self) -> Option<Duration> {
        match self.state() {
            State::Discharging => {
                let time_to_empty = 3600.0 * self.energy() / self.energy_rate();
                if time_to_empty > (240.0 * 60.0 * 60.0) { // Ten days for discharging
                    None
                } else {
                    Some(Duration::from_secs(time_to_empty as u64))
                }
            },
            _ => None,
        }
    }
}

#[inline]
fn set_bounds(value: f64) -> f64 {
    if value < 0.0 {
        return 0.0;
    }
    if value > 100.0 {
        return 100.0;
    }

    value
}
