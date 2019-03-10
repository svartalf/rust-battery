use std::f32;
use std::io;
use std::path::Path;

use lazycell::LazyCell;
use num_traits::identities::Zero;

use super::fs;
use crate::units::power::{microwatt, watt};
use crate::units::{
    Bound, ElectricCharge, ElectricPotential, Energy, Power, Ratio, ThermodynamicTemperature,
};
use crate::{Result, Error, State, Technology};

#[derive(Debug)]
pub struct InstantData {
    pub state_of_health: Ratio,
    pub state_of_charge: Ratio,

    pub energy: Energy,
    pub energy_full: Energy,
    pub energy_full_design: Energy,
    pub energy_rate: Power,
    pub voltage: ElectricPotential,
    pub state: State,
    pub temperature: Option<ThermodynamicTemperature>,
    pub cycle_count: Option<u32>,
}

pub struct DataBuilder<'p> {
    root: &'p Path,

    design_voltage: LazyCell<ElectricPotential>,
    energy: LazyCell<Energy>,
    energy_full: LazyCell<Energy>,
    energy_full_design: LazyCell<Energy>,
    energy_rate: LazyCell<Power>,

    state_of_health: LazyCell<Ratio>,
    state_of_charge: LazyCell<Ratio>,

    state: LazyCell<State>,
}

impl<'p> DataBuilder<'p> {
    pub fn new(path: &'p Path) -> DataBuilder<'p> {
        DataBuilder {
            root: path,

            design_voltage: LazyCell::new(),
            energy: LazyCell::new(),
            energy_full: LazyCell::new(),
            energy_full_design: LazyCell::new(),
            energy_rate: LazyCell::new(),
            state_of_health: LazyCell::new(),
            state_of_charge: LazyCell::new(),
            state: LazyCell::new(),
        }
    }

    pub fn collect(self) -> Result<InstantData> {
        Ok(InstantData {
            state_of_charge: *self.state_of_charge()?,
            state_of_health: *self.state_of_health()?,
            energy: *self.energy()?,
            energy_full: *self.energy_full()?,
            energy_full_design: *self.energy_full_design()?,
            energy_rate: *self.energy_rate()?,
            voltage: self.voltage()?,
            state: *self.state()?,
            temperature: self.temperature()?,
            cycle_count: self.cycle_count()?,
        })
    }

    fn design_voltage(&self) -> Result<&ElectricPotential> {
        self.design_voltage.try_borrow_with(|| {
            let value = [
                "voltage_max_design",
                "voltage_min_design",
                "voltage_present",
                "voltage_now",
            ]
            .iter()
            .filter_map(|filename| match fs::voltage(self.root.join(filename)) {
                Ok(Some(value)) => Some(value),
                _ => None,
            })
            .next();
            match value {
                Some(voltage) => Ok(voltage),
                None => Err(io::Error::from(io::ErrorKind::NotFound).into()),
            }
        })
    }

    // Not cached because used only once
    // IO errors are ignored, since later calculations will handle `None` result
    fn energy_now(&self) -> Option<Energy> {
        ["energy_now", "energy_avg"]
            .iter()
            .filter_map(|filename| match fs::energy(self.root.join(filename)) {
                Ok(Some(value)) => Some(value),
                _ => None,
            })
            .next()
    }

    // Not cached because used only once.
    // IO errors are ignored, since later calculations will handle `None` result
    fn charge_now(&self) -> Option<ElectricCharge> {
        ["charge_now", "charge_avg"]
            .iter()
            .filter_map(|filename| match fs::charge(self.root.join(filename)) {
                Ok(Some(value)) => Some(value),
                _ => None,
            })
            .next()
    }

    // Not cached because used only once
    fn charge_full(&self) -> ElectricCharge {
        ["charge_full", "charge_full_design"]
            .iter()
            .filter_map(|filename| match fs::charge(self.root.join(filename)) {
                Ok(Some(value)) => Some(value),
                _ => None,
            })
            .next()
            .unwrap_or_else(|| microampere_hour!(0.0))
    }

    pub fn state_of_health(&self) -> Result<&Ratio> {
        self.state_of_health.try_borrow_with(|| {
            let energy_full = self.energy_full()?;
            if !energy_full.is_zero() {
                let energy_full_design = self.energy_full_design()?;
                Ok((*energy_full / *energy_full_design).into_bounded())
            } else {
                Ok(percent!(100.0))
            }
        })
    }

    fn energy(&self) -> Result<&Energy> {
        self.energy.try_borrow_with(|| {
            match self.energy_now() {
                Some(energy) => Ok(energy),
                None => match self.charge_now() {
                    Some(charge) => Ok(charge * *self.design_voltage()?),
                    None => {
                        match fs::get::<f32, _>(self.root.join("capacity")) {
                            Ok(Some(capacity)) => Ok(*self.energy_full()? * percent!(capacity).into_bounded()),
                            _ => Err(Error::not_found("Unable to calculate device energy value")),
                        }
                    }
                },
            }
        })
    }

    fn energy_full(&self) -> Result<&Energy> {
        self.energy_full
            .try_borrow_with(|| match fs::energy(self.root.join("energy_full")) {
                Ok(Some(value)) => Ok(value),
                Ok(None) => match fs::charge(self.root.join("charge_full")) {
                    Ok(Some(value)) => Ok(value * *self.design_voltage()?),
                    Ok(None) => Ok(*self.energy_full_design()?),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            })
    }

    fn energy_full_design(&self) -> Result<&Energy> {
        // Based on the `upower` source it seems to impossible not to have any of needed files,
        // so fallback to `0 mWh` was removed, error will be propagated instead.
        self.energy_full_design.try_borrow_with(|| {
            match fs::energy(self.root.join("energy_full_design")) {
                Ok(Some(value)) => Ok(value),
                Ok(None) => match fs::charge(self.root.join("charge_full_design")) {
                    Ok(Some(value)) => Ok(value * *self.design_voltage()?),
                    Ok(None) => Ok(*self.energy_full_design()?),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        })
    }

    fn energy_rate(&self) -> Result<&Power> {
        self.energy_rate.try_borrow_with(|| {
            let value = match fs::power(self.root.join("power_now"))? {
                Some(power) => Some(power),
                None => {
                    match fs::get::<f32, _>(self.root.join("current_now"))? {
                        Some(current_now) => {
                            // If charge_full exists, then current_now is always reported in µA.
                            // In the legacy case, where energy only units exist, and power_now isn't present
                            // current_now is power in µW.
                            // Source: upower
                            if !self.charge_full().is_zero() {
                                // µA then
                                Some(microampere!(current_now) * *self.design_voltage()?)
                            } else {
                                // µW :|
                                Some(microwatt!(current_now))
                            }
                        }
                        None => None,
                    }
                }
            };

            let value = value
                // Sanity check if power is greater than 100W (upower)
                .map(|power| {
                    if power.get::<watt>() > 100.0 {
                        watt!(0.0)
                    } else {
                        power
                    }
                })
                // Some batteries give out massive rate values when nearly empty (upower)
                .map(|power| {
                    if power.get::<microwatt>() < 10.0 {
                        watt!(0.0)
                    } else {
                        power
                    }
                })
                // ACPI gives out the special 'Ones' (Constant Ones Object) value for rate
                // when it's unable to calculate the true rate. We should set the rate zero,
                // and wait for the BIOS to stabilise.
                // Source: upower
                //
                // It come as an `0xffff` originally, but we are operating with `Power` now,
                // so this `Ones` value is recalculated a little.
                .map(|power| {
                    // TODO: There might be a chance that we had lost a precision during the conversion
                    // from the microwatts into default watts, so this should be fixed
                    if (power.get::<watt>() - 65535.0).abs() < f32::EPSILON {
                        watt!(0.0)
                    } else {
                        power
                    }
                })
                .unwrap_or_else(|| microwatt!(0.0));

            // TODO: Calculate energy_rate manually, if hardware fails.
            // if value < 0.01 {
            //    // Check upower `up_device_supply_calculate_rate` function
            // }

            Ok(value)
        })
    }

    fn state_of_charge(&self) -> Result<&Ratio> {
        self.state_of_charge.try_borrow_with(|| {
            match fs::get::<f32, _>(self.root.join("capacity")) {
                Ok(Some(capacity)) => Ok(percent!(capacity).into_bounded()),
                Ok(None) if self.energy_full()?.is_sign_positive() => {
                    Ok(*self.energy()? / *self.energy_full()?)
                }
                // Same as upower, falling back to 0.0%
                Ok(None) => Ok(percent!(0.0)),
                Err(e) => Err(e),
            }
        })
    }

    fn state(&self) -> Result<&State> {
        self.state
            .try_borrow_with(|| match fs::get::<State, _>(self.root.join("status")) {
                Ok(Some(state)) => Ok(state),
                Ok(None) => Ok(State::Unknown),
                Err(e) => Err(e),
            })
    }

    fn voltage(&self) -> Result<ElectricPotential> {
        let mut value = ["voltage_now", "voltage_avg"]
            .iter()
            .filter_map(|filename| match fs::voltage(self.root.join(filename)) {
                Ok(Some(value)) => Some(value),
                _ => None,
            });

        match value.next() {
            Some(value) => Ok(value),
            None => Err(Error::not_found("Unable to calculate device voltage value")),
        }
    }

    fn temperature(&self) -> Result<Option<ThermodynamicTemperature>> {
        match fs::get::<f32, _>(self.root.join("temp")) {
            Ok(Some(value)) => Ok(Some(celsius!(value / 10.0))),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn cycle_count(&self) -> Result<Option<u32>> {
        fs::get::<u32, _>(self.root.join("cycle_count"))
    }

    // Following methods are not cached in the struct

    pub fn manufacturer(&self) -> Result<Option<String>> {
        fs::get_string(self.root.join("manufacturer"))
    }

    pub fn model(&self) -> Result<Option<String>> {
        fs::get_string(self.root.join("model_name"))
    }

    pub fn serial_number(&self) -> Result<Option<String>> {
        fs::get_string(self.root.join("serial_number"))
    }

    pub fn technology(&self) -> Result<Technology> {
        match fs::get::<Technology, _>(self.root.join("technology")) {
            Ok(Some(tech)) => Ok(tech),
            Ok(None) => Ok(Technology::Unknown),
            Err(e) => Err(e),
        }
    }
}
