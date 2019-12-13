use std::convert::AsRef;
use std::fmt;

use super::acpi;
use crate::platform::traits::BatteryDevice;
use crate::units::{ElectricPotential, Energy, Power, ThermodynamicTemperature};
use crate::{Result, State, Technology};

#[derive(Default)]
pub struct IoCtlDevice {
    unit: libc::c_int,
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
    pub fn new(unit: libc::c_int, bif: acpi::AcpiBif, bst: acpi::AcpiBst) -> IoCtlDevice {
        let mut device = IoCtlDevice {
            unit,
            ..Default::default()
        };

        device.manufacturer = bif.oem();
        device.model = bif.model();
        device.serial_number = bif.serial();
        device.technology = bif.technology();

        device.refresh(bif, bst).expect("unreachable");

        device
    }

    pub fn unit(&self) -> libc::c_int {
        self.unit
    }

    pub fn refresh(&mut self, bif: acpi::AcpiBif, bst: acpi::AcpiBst) -> Result<()> {
        let voltage = millivolt!(bst.voltage());

        // FreeBSD returns battery info (bif) either in mA or mV, and we need the mW.
        // mA values are multiplied by `bif.dvol`,
        // as in `sys/dev/acpica/acpi_battery.c:acpi_battery_get_battinfo` function
        let design_voltage = millivolt!(bif.design_voltage());
        self.energy_rate = match bif.units() {
            acpi::Units::MilliWatts => milliwatt!(bst.rate()),
            acpi::Units::MilliAmperes => milliampere!(bst.rate()) * design_voltage,
        };
        self.current_capacity = match bif.units() {
            acpi::Units::MilliWatts => milliwatt_hour!(bst.capacity()),
            acpi::Units::MilliAmperes => milliampere_hour!(bst.capacity()) * design_voltage,
        };
        self.design_capacity = match bif.units() {
            acpi::Units::MilliWatts => milliwatt_hour!(bif.design_capacity()),
            acpi::Units::MilliAmperes => milliampere_hour!(bif.design_capacity()) * design_voltage,
        };
        self.max_capacity = match bif.units() {
            acpi::Units::MilliWatts => milliwatt_hour!(bif.last_full_capacity()),
            acpi::Units::MilliAmperes => milliampere_hour!(bif.last_full_capacity()) * design_voltage,
        };
        self.state = bst.state();
        self.voltage = voltage;

        Ok(())
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

impl fmt::Debug for IoCtlDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FreeBSDDevice").field("unit", &self.unit).finish()
    }
}
