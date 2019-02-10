use std::fmt;
use std::time::Duration;

use crate::{Device, State, Technology};

/// Battery instant information representation.
///
/// Consequent calls of the same method will return the same value.
pub struct Battery<T: Device> {
    device: T,
}

impl<T> Battery<T> where T: Device {
    pub(crate) fn new(device: T) -> Battery<T> {
        Battery {
            device,
        }
    }

    /// Gets battery current state.
    ///
    /// See [State](enum.State.html) enum for possible values.
    pub fn state(&self) -> State {
        self.device.state()
    }

    /// Gets battery technology.
    ///
    /// See [Technology](enum.Technology.html) enum for possible values.
    pub fn technology(&self) -> Technology {
        self.device.technology()
    }

    /// Gets battery vendor.
    ///
    /// Might not exist.
    pub fn vendor(&self) -> Option<&str> {
        self.device.vendor()
    }

    /// Gets battery model.
    ///
    /// Might not exist.
    pub fn model(&self) -> Option<&str> {
        self.device.model()
    }

    /// Gets battery serial number.
    ///
    /// Might not exist.
    pub fn serial_number(&self) -> Option<&str> {
        self.device.serial_number()
    }

    /// Gets battery capacity in `0.0..100.0` percents range.
    pub fn capacity(&self) -> f64 {
        self.device.capacity()
    }

    /// Gets battery temperature in Celsius degrees.
    pub fn temperature(&self) -> f64 {
        self.device.temperature()
    }

    pub fn percentage(&self) -> f64 {
        self.device.percentage()
    }

    /// `Wh`
    pub fn energy(&self) -> f64 {
        self.device.energy()
    }

    pub fn energy_full(&self) -> f64 {
        self.device.energy_full()
    }

    pub fn energy_full_design(&self) -> f64 {
        self.device.energy_full_design()
    }

    pub fn energy_rate(&self) -> f64 {
        self.device.energy_rate()
    }

    /// Gets a battery voltage (in `V`).
    pub fn voltage(&self) -> f64 {
        self.device.voltage()
    }

    /// Gets a remaining time till full battery.
    ///
    /// This is an instant value and may different vastly from call to call.
    /// Any aggregation should be made by caller.
    ///
    /// If battery is not charging at the moment, this method will return `None`.
    pub fn time_to_full(&self) -> Option<Duration> {
        self.device.time_to_full()
    }

    /// Gets a remaining time till empty battery.
    ///
    /// This is an instant value and may different vastly from call to call.
    /// Any aggregation should be made by caller.
    ///
    /// If battery is not discharging at the moment, this method will return `None`.
    pub fn time_to_empty(&self) -> Option<Duration> {
        self.device.time_to_empty()
    }

}

impl<T> fmt::Debug for Battery<T> where T: Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Battery")
            // static info
            .field("vendor", &self.vendor())
            .field("model", &self.model())
            .field("serial_number", &self.serial_number())
            .field("technology", &self.technology())

            // common information
            .field("state", &self.state())
            .field("capacity", &self.capacity())
            .field("temperature", &self.temperature())
            .field("percentage", &self.percentage())

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
