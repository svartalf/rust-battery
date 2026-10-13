#[cfg(feature = "export")]
use std::ffi::{CString, c_float};
use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::platform::traits::*;
use crate::platform::Device;
use crate::units::{ElectricPotential, Energy, Power, Ratio, ThermodynamicTemperature, Time};
use crate::{State, Technology};

/// Battery instant information representation.
///
/// Consequent calls of the same method will return the same value.\
/// See the [Manager::refresh](struct.Manager.html#method.refresh) method,
/// which can be used to update information hold in the current `Battery`.
///
/// Almost all methods are returning values in the [SI measurement units](https://www.bipm.org/en/measurement-units/),
/// represented as a units from the [uom](https://crates.io/crates/uom) crate.\
/// If you are unfamiliar with `uom`, check the [units](./units/) module documentation for a few examples
/// of how to get the values from them.
pub struct Battery(Device)
where
    Device: BatteryDevice;

impl Battery {
    /// Battery current state.
    ///
    /// See [State](enum.State.html) enum for possible values.
    pub fn state(&self) -> State {
        self.0.state()
    }

    /// Battery technology.
    ///
    /// See [Technology](enum.Technology.html) enum for possible values.
    pub fn technology(&self) -> Technology {
        self.0.technology()
    }

    fn state_of_charge_value(&self) -> Ratio {
        self.0.state_of_charge()
    }

    fn energy_value(&self) -> Energy {
        self.0.energy()
    }

    fn energy_full_value(&self) -> Energy {
        self.0.energy_full()
    }

    fn energy_full_design_value(&self) -> Energy {
        self.0.energy_full_design()
    }

    fn energy_rate_value(&self) -> Power {
        self.0.energy_rate()
    }

    fn voltage_value(&self) -> ElectricPotential {
        self.0.voltage()
    }

    fn state_of_health_value(&self) -> Ratio {
        self.0.state_of_health()
    }

    fn temperature_value(&self) -> Option<ThermodynamicTemperature> {
        self.0.temperature()
    }

    fn cycle_count_value(&self) -> Option<u32> {
        self.0.cycle_count()
    }

    fn vendor_value(&self) -> Option<&str> {
        self.0.vendor()
    }

    fn model_value(&self) -> Option<&str> {
        self.0.model()
    }

    fn serial_number_value(&self) -> Option<&str> {
        self.0.serial_number()
    }

    fn time_to_full_value(&self) -> Option<Time> {
        self.0.time_to_full()
    }

    fn time_to_empty_value(&self) -> Option<Time> {
        self.0.time_to_empty()
    }
}

#[cfg(not(feature = "export"))]
impl Battery {
    /// Battery state of charge.
    ///
    /// The *State of Charge* (or *SOC*) is an expression of the battery capacity
    /// as a percentage of maximum capacity.
    ///
    /// In plain english: it is how much energy your battery has (expressed in percents).
    /// This is an exactly that value which operating systems and desktop managers
    /// are displaying in the taskbar near the clock.
    ///
    /// Roughly it can be calculated as `battery.energy() / battery.energy_full()`,
    /// but you should always use [Battery::state_of_charge](#method.state_of_charge)
    /// instead of the manual calculation, because many device drivers are providing
    /// this value more precisely, and this method takes that into account.
    ///
    /// See also:
    ///  * [https://en.wikipedia.org/wiki/State_of_charge](https://en.wikipedia.org/wiki/State_of_charge)
    ///  * [https://www.mpoweruk.com/soc.htm](https://www.mpoweruk.com/soc.htm)
    pub fn state_of_charge(&self) -> Ratio {
        self.state_of_charge_value()
    }

    /// Amount of energy currently available in the battery.
    pub fn energy(&self) -> Energy {
        self.energy_value()
    }

    /// Amount of energy in the battery when it's considered full.
    pub fn energy_full(&self) -> Energy {
        self.energy_full_value()
    }

    /// Amount of energy the battery is designed to hold when it's considered full.
    pub fn energy_full_design(&self) -> Energy {
        self.energy_full_design_value()
    }

    /// Amount of energy being drained from the battery.
    pub fn energy_rate(&self) -> Power {
        self.energy_rate_value()
    }

    /// Battery voltage.
    pub fn voltage(&self) -> ElectricPotential {
        self.voltage_value()
    }

    /// Gets battery state of health.
    ///
    /// The *State of Health* (or *SOH*) is an indication of the point
    /// which has been reached in the life cycle of the battery
    /// and a measure of its condition relative to a fresh battery.
    ///
    /// In plain english: this is how much energy in percents your battery can hold when fully charged.
    /// New battery - 100 %, old and degraded battery - notably lower amount of percents.
    /// See also:
    ///
    ///  * [https://en.wikipedia.org/wiki/State_of_health](https://en.wikipedia.org/wiki/State_of_health)
    ///  * [https://www.mpoweruk.com/soh.htm](https://www.mpoweruk.com/soh.htm)
    pub fn state_of_health(&self) -> Ratio {
        self.state_of_health_value()
    }

    /// Battery temperature.
    pub fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.temperature_value()
    }

    /// Number of charge/discharge cycles.
    pub fn cycle_count(&self) -> Option<u32> {
        self.cycle_count_value()
    }

    /// Battery vendor.
    pub fn vendor(&self) -> Option<&str> {
        self.vendor_value()
    }

    /// Battery model.
    pub fn model(&self) -> Option<&str> {
        self.model_value()
    }

    /// Battery serial number.
    pub fn serial_number(&self) -> Option<&str> {
        self.serial_number_value()
    }

    /// Remaining time till full battery.
    ///
    /// This is an instant value and may different vastly from call to call.
    /// Any aggregation should be made by caller.
    ///
    /// If battery is not charging at the moment, this method will return `None`.
    pub fn time_to_full(&self) -> Option<Time> {
        self.time_to_full_value()
    }

    /// Remaining time till empty battery.
    ///
    /// This is an instant value and may different vastly from call to call.
    /// Any aggregation should be made by caller.
    ///
    /// If battery is not discharging at the moment, this method will return `None`.
    pub fn time_to_empty(&self) -> Option<Time> {
        self.time_to_empty_value()
    }
}

#[cfg(feature = "export")]
impl Battery {
    pub fn state_of_charge(&self) -> c_float {
        self.state_of_charge_value()
            .get::<crate::units::ratio::percent>()
    }

    pub fn energy(&self) -> c_float {
        self.energy_value().get::<crate::units::energy::joule>()
    }

    pub fn energy_full(&self) -> c_float {
        self.energy_full_value().get::<crate::units::energy::joule>()
    }

    pub fn energy_full_design(&self) -> c_float {
        self.energy_full_design_value()
            .get::<crate::units::energy::joule>()
    }

    pub fn energy_rate(&self) -> c_float {
        self.energy_rate_value().get::<crate::units::power::watt>()
    }

    pub fn voltage(&self) -> c_float {
        self.voltage_value()
            .get::<crate::units::electric_potential::volt>()
    }

    pub fn state_of_health(&self) -> c_float {
        self.state_of_health_value()
            .get::<crate::units::ratio::percent>()
    }

    pub fn time_to_full(&self) -> c_float {
        self.time_to_full_value()
            .map_or(f32::NAN, |value| value.get::<crate::units::time::second>())
    }

    pub fn time_to_empty(&self) -> c_float {
        self.time_to_empty_value()
            .map_or(f32::NAN, |value| value.get::<crate::units::time::second>())
    }

    pub fn temperature(&self) -> c_float {
        self.temperature_value().map_or(f32::NAN, |value| {
            value.get::<crate::units::thermodynamic_temperature::kelvin>()
        })
    }

    pub fn cycle_count(&self) -> u32 {
        self.cycle_count_value().unwrap_or(u32::MAX)
    }

    pub fn vendor(&self) -> Option<CString> {
        self.vendor_value().and_then(|value| CString::new(value).ok())
    }

    pub fn model(&self) -> Option<CString> {
        self.model_value().and_then(|value| CString::new(value).ok())
    }

    pub fn serial_number(&self) -> Option<CString> {
        self.serial_number_value().and_then(|value| CString::new(value).ok())
    }
}

impl fmt::Debug for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Battery")
            .field("impl", &self.0)
            // static info
            .field("vendor", &self.vendor())
            .field("model", &self.model())
            .field("serial_number", &self.serial_number())
            .field("technology", &self.technology())
            // common information
            .field("state", &self.state())
            .field("capacity", &self.state_of_health())
            .field("temperature", &self.temperature())
            .field("percentage", &self.state_of_charge())
            .field("cycle_count", &self.cycle_count())
            // energy stats
            .field("energy", &self.energy())
            .field("energy_full", &self.energy_full())
            .field("energy_full_design", &self.energy_full_design())
            .field("energy_rate", &self.energy_rate())
            .field("voltage", &self.voltage())
            // charge stats
            .field("time_to_full", &self.time_to_full())
            .field("time_to_empty", &self.time_to_empty())
            .finish()
    }
}

impl From<Device> for Battery {
    fn from(device: Device) -> Battery {
        Battery(device)
    }
}

impl Deref for Battery {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Battery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
