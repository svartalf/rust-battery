use std::convert::AsRef;
use std::fmt;

use super::ffi::{BatteryQueryInformation, DeviceHandle};
use crate::platform::traits::BatteryDevice;
use crate::units::{ElectricPotential, Energy, Power, ThermodynamicTemperature};
use crate::{Error, Result, State, Technology};

#[derive(Default)]
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
    pub fn try_from(mut handle: DeviceHandle) -> Result<Option<PowerDevice>> {
        let info = handle.information()?;
        if info.is_relative() {
            // We can't support batteries with relative data so far
            return Ok(None);
        }

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

        let mut device = PowerDevice {
            tag: handle.tag.clone(),
            technology: info.technology(),
            device_name,
            manufacturer,
            serial_number,
            ..Default::default()
        };
        device.refresh(handle)?;
        Ok(Some(device))
    }

    pub fn refresh(&mut self, mut handle: DeviceHandle) -> Result<()> {
        let info = handle.information()?;

        let status = handle.status()?;
        let rate = match status.rate() {
            None => return Err(Error::invalid_data("Device rate value is unknown")),
            Some(value) => milliwatt!(value),
        };
        let capacity = match status.capacity() {
            None => return Err(Error::invalid_data("Device capacity value is unknown")),
            Some(value) => milliwatt_hour!(value),
        };
        let voltage = match status.voltage() {
            None => return Err(Error::invalid_data("Device voltage value is unknown")),
            Some(value) => millivolt!(value),
        };
        let temperature = match handle.temperature() {
            Ok(value) => Some(decikelvin!(value)),
            Err(_) => None,
        };

        self.state = status.state();
        self.energy_rate = rate;
        self.design_capacity = milliwatt_hour!(info.designed_capacity());
        self.full_charged_capacity = milliwatt_hour!(info.full_charged_capacity());
        self.cycle_count = info.cycle_count();
        self.capacity = capacity;
        self.voltage = voltage;
        self.temperature = temperature;

        Ok(())
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

    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count
    }
}

impl fmt::Debug for PowerDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WindowsDevice")
            .field("tag", &self.tag.battery_tag())
            .finish()
    }
}
