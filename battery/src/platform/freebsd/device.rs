use std::io;
use std::convert::AsRef;
use std::fs::OpenOptions;

use crate::{State, Technology};
use crate::platform::traits::BatteryDevice;
use crate::units::{Energy, Power, ElectricPotential, ThermodynamicTemperature};
use super::acpi;

#[derive(Debug)]
pub struct IoCtlDevice {
    state: State,
    technology: Technology,

    energy_rate: Power,
    voltage: ElectricPotential,

    design_capacity: Energy,
    max_capacity: Energy,
    current_capacity: Energy,

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

        let voltage = millivolt!(bst.voltage());

        // FreeBSD returns battery info (bif) either in mA or mV, and we need the mW.
        // mA values are multiplied by `bif.dvol`,
        // as in `sys/dev/acpica/acpi_battery.c:acpi_battery_get_battinfo` function
        let design_voltage = millivolt!(bif.design_voltage());
        let energy_rate = match bif.units() {
            acpi::Units::MilliWatts => milliwatt!(bst.rate()),
            acpi::Units::MilliAmperes => milliampere!(bst.rate()) * design_voltage,
        };
        let current_capacity = match bif.units() {
            acpi::Units::MilliWatts => milliwatt_hour!(bst.capacity()),
            acpi::Units::MilliAmperes => milliampere_hour!(bst.capacity()) * design_voltage,
        };
        let design_capacity = match bif.units() {
            acpi::Units::MilliWatts => milliwatt_hour!(bif.design_capacity()),
            acpi::Units::MilliAmperes => milliampere_hour!(bif.design_capacity()) * design_voltage,
        };
        let max_capacity = match bif.units() {
            acpi::Units::MilliWatts => milliwatt_hour!(bif.last_full_capacity()),
            acpi::Units::MilliAmperes => milliampere_hour!(bif.last_full_capacity()) * design_voltage,
        };

        let device = IoCtlDevice {
            manufacturer: bif.oem(),
            model: bif.model(),
            serial_number: bif.serial(),
            state: bst.state(),
            technology: bif.technology(),
            energy_rate,
            voltage,
            design_capacity,
            max_capacity,
            current_capacity,
        };

        Ok(device)
    }
}

impl BatteryDevice for IoCtlDevice {
    fn energy(&self) -> Energy {
        self.current_capacity
    }

    fn energy_full(&self) -> Energy {
        self.max_capacity
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
