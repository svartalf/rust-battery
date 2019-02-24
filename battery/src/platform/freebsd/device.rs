use std::io;
use std::convert::AsRef;
use std::fs::OpenOptions;

use crate::{State, Technology};
use crate::platform::traits::BatteryDevice;
use super::acpi;

#[derive(Debug)]
pub struct IoCtlDevice {
    state: State,
    technology: Technology,

    energy_rate: u32,
    voltage: u32,

    design_capacity: u32,
    max_capacity: u32,
    current_capacity: u32,

    manufacturer: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,

}

impl IoCtlDevice {
    pub fn new() -> io::Result<IoCtlDevice> {
        let file = OpenOptions::new()
            .read(true)
            .open("/dev/acpi")?;
        let inner = acpi::AcpiDevice::new(file);
        let bif = inner.bif()?;
        let bst = inner.bst()?;

        let voltage = bst.voltage();
        let mut device = IoCtlDevice {
            state: bst.state(),
            technology: bif.technology(),
            energy_rate: bst.rate(),
            voltage: voltage,
            design_capacity: bif.design_capacity(),
            max_capacity: bif.last_full_capacity(),
            current_capacity: bst.capacity(),
            manufacturer: bif.oem(),
            model: bif.model(),
            serial_number: bif.serial(),
        };

        if bif.units() == acpi::Units::mA {
            device.energy_rate *= voltage;
            device.current_capacity *= voltage;
            device.design_capacity *= voltage;
            device.max_capacity *= voltage;
        }

        Ok(device)
    }
}

impl BatteryDevice for IoCtlDevice {
    fn energy(&self) -> u32 {
        self.current_capacity
    }

    fn energy_full(&self) -> u32 {
        self.max_capacity
    }

    fn energy_full_design(&self) -> u32 {
        self.design_capacity
    }

    fn energy_rate(&self) -> u32 {
        self.energy_rate
    }

    fn percentage(&self) -> f32 {
        (100 * self.energy() / self.energy_full()) as f32
    }

    fn state(&self) -> State {
        self.state
    }

    fn voltage(&self) -> u32 {
        self.voltage
    }

    fn temperature(&self) -> Option<f32> {
        None
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
        self.technology
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

}
