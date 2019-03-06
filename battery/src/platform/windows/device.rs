use std::io;
use std::convert::AsRef;

use crate::units::{ElectricPotential, ThermodynamicTemperature, Power, Energy};
use crate::{State, Technology};
use crate::platform::traits::BatteryDevice;
use super::ffi::{DeviceHandle, BatteryQueryInformation};

#[derive(Debug)]
pub struct PowerDevice {
    // Used later for information refreshing
    tag: BatteryQueryInformation,

    technology: Technology,
    state: State,
    voltage: ElectricPotential,
    energy_rate: Power,
    capacity: Energy,
    design_capacity: Energy,
    full_charged_capacity: Energy,
    temperature: Option<ThermodynamicTemperature>,
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
            Some(value) => milliwatt!(value),
        };
        let capacity = match status.capacity() {
            None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
            // TODO: Get rid of `* 0.001` when uom will have the milliwatt_hour type
            Some(value) => milliwatt_hour!(value),
        };
        let voltage = match status.voltage() {
            None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
            Some(value) => millivolt!(value),
        };
        let temperature = match handle.temperature() {
            Ok(value) => Some(decikelvin!(value)),
            Err(_) => None,
        };

        Ok(PowerDevice {
            tag: handle.tag,
            technology: info.technology(),
            state: status.state(),
            energy_rate: rate,
            design_capacity: milliwatt_hour!(info.designed_capacity()),
            full_charged_capacity: milliwatt_hour!(info.full_charged_capacity()),
            cycle_count: info.cycle_count(),
            capacity,
            voltage,
            temperature,
            device_name,
            manufacturer,
            serial_number,
        })
    }

    pub fn tag(&self) -> &BatteryQueryInformation {
        &self.tag
    }

}

impl BatteryDevice for PowerDevice {
    fn energy(&self) -> Energy {
        self.capacity
    }

    fn energy_full(&self) -> Energy {
        self.full_charged_capacity
    }

    fn energy_full_design(&self) -> Energy {
        self.design_capacity
    }

    fn energy_rate(&self) -> Power {
        self.energy_rate
    }

    fn state(&self) -> State {
        self.state
    }

    fn voltage(&self) -> ElectricPotential {
        self.voltage
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.temperature
    }

    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count
    }

    fn vendor(&self) -> Option<&str> {
        self.manufacturer.as_ref().map(AsRef::as_ref)
    }

    fn model(&self) -> Option<&str> {
        self.device_name.as_ref().map(AsRef::as_ref)
    }

    fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_ref().map(AsRef::as_ref)
    }

    fn technology(&self) -> Technology {
        self.technology
    }
}
