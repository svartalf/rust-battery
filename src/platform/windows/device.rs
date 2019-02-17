use std::io;

use crate::{State, Technology};
use crate::types::Device;
use super::ffi::DeviceHandle;

#[derive(Debug)]
pub struct PowerDevice {
    technology: Technology,
    state: State,
    voltage: u32,
    capacity: u32,
    energy_rate: u32,
    design_capacity: u32,
    full_charged_capacity: u32,
    temperature: Option<f32>,
    cycle_count: Option<u32>,
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
        let voltage = match status.voltage() {
            None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
            Some(value) => value,
        };
        let temperature = match handle.temperature() {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        Ok(PowerDevice {
            technology: info.technology(),
            state: status.state(),
            energy_rate: rate,
            design_capacity: info.designed_capacity(),
            full_charged_capacity: info.full_charged_capacity(),
            cycle_count: info.cycle_count(),
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
    fn capacity(&self) -> f32 {
        ((self.energy_full() / self.energy_full_design()) * 100) as f32
    }

    fn energy(&self) -> u32 {
        self.capacity
    }

    fn energy_full(&self) -> u32 {
        self.full_charged_capacity
    }

    fn energy_full_design(&self) -> u32 {
        self.design_capacity
    }

    fn energy_rate(&self) -> u32 {
        self.energy_rate
    }

    fn percentage(&self) -> f32 {
        set_bounds(100 * self.energy() / self.energy_full()) as f32
    }

    fn state(&self) -> State {
        self.state
    }

    fn voltage(&self) -> u32 {
        self.voltage
    }

    fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count
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
}

#[inline]
fn set_bounds(value: u32) -> u32 {
    if value > 100 {
        100
    } else {
        value
    }
}
