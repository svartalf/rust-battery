use std::io;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::platform::traits::*;
use crate::units::{ElectricPotential, Energy, Power, Ratio, ThermodynamicTemperature};
use crate::{Result, Error, State, Technology};

use super::sysfs::{fs, DataBuilder, InstantData, Scope, Type};

pub struct SysFsDevice {
    root: PathBuf,
    source: InstantData,

    // These fields are "cached" outside from DataBuilder/InstantData,
    // since they're can't change with refresh
    vendor: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    technology: Technology,
}

impl SysFsDevice {
    pub fn is_system_battery<T: AsRef<Path>>(path: T) -> Result<bool> {
        let path = path.as_ref();
        if fs::type_(path.join("type"))? == Type::Battery &&
            fs::scope(path.join("scope"))? == Scope::System {
                return Ok(true);
        }

        Ok(false)
    }

    pub fn try_from(root: PathBuf) -> Result<SysFsDevice> {
        let builder = DataBuilder::new(&root);
        let vendor = builder.manufacturer()?;
        let model = builder.model()?;
        let serial_number = builder.serial_number()?;
        let technology = builder.technology()?;

        let source = builder.collect()?;

        Ok(SysFsDevice {
            root,
            source,
            vendor,
            model,
            serial_number,
            technology,
        })
    }

    pub fn refresh(&mut self) -> Result<()> {
        // It is necessary to ensure that `self.root`
        // still exists and accessible.
        // See https://github.com/svartalf/rust-battery/issues/29
        if self.root.is_dir() {
            let builder = DataBuilder::new(&self.root);
            self.source = builder.collect()?;

            Ok(())
        } else {
            let inner = io::Error::from(io::ErrorKind::NotFound);
            let e = Error::new(
                inner,
                format!("Device directory `{:?}` is missing", self.root),
            );

            Err(e)
        }
    }
}

impl BatteryDevice for SysFsDevice {

    fn state_of_health(&self) -> Ratio {
        self.source.state_of_health
    }

    fn state_of_charge(&self) -> Ratio {
        self.source.state_of_charge
    }

    fn energy(&self) -> Energy {
        self.source.energy
    }

    fn energy_full(&self) -> Energy {
        self.source.energy_full
    }

    fn energy_full_design(&self) -> Energy {
        self.source.energy_full_design
    }

    fn energy_rate(&self) -> Power {
        self.source.energy_rate
    }

    fn state(&self) -> State {
        self.source.state
    }

    fn voltage(&self) -> ElectricPotential {
        self.source.voltage
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.source.temperature
    }

    fn vendor(&self) -> Option<&str> {
        self.vendor.as_ref().map(AsRef::as_ref)
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
        self.source.cycle_count
    }
}

impl fmt::Debug for SysFsDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LinuxDevice")
            .field("root", &self.root)
            .finish()
    }
}
