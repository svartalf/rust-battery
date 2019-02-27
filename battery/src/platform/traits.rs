use std::io;
use std::time::Duration;

use crate::{Battery, State, Technology};

pub trait BatteryManager: Default + Sized {
    fn refresh(&mut self, battery: &mut Battery) -> io::Result<()>;
}

pub trait BatteryIterator: Iterator<Item=Battery> + Sized {}

pub trait BatteryDevice: Sized {
    fn capacity(&self) -> f32 {
        let full = self.energy_full() as f32;
        let full_design = self.energy_full_design() as f32;

        (full / full_design) * 100.0
    }

    fn energy(&self) -> u32;

    fn energy_full(&self) -> u32;

    fn energy_full_design(&self) -> u32;

    fn energy_rate(&self) -> u32;

    fn percentage(&self) -> f32;

    fn state(&self) -> State;

    fn voltage(&self) -> u32;

    fn temperature(&self) -> Option<f32>;

    fn vendor(&self) -> Option<&str>;

    fn model(&self) -> Option<&str>;

    fn serial_number(&self) -> Option<&str>;

    fn technology(&self) -> Technology;

    fn cycle_count(&self) -> Option<u32>;

    // Default implementation for `time_to_full` and `time_to_empty`
    // uses calculation based on the current energy flow,
    // but if device provides by itself provides these **instant** values (do not use average values),
    // it would be easier and cheaper to return them instead of making some calculations

    fn time_to_full(&self) -> Option<Duration> {
        let energy_rate = self.energy_rate();
        match self.state() {
            // In some cases energy_rate can be 0 while Charging, for example just after
            // plugging in the charger. Assume that the battery doesn't have time_to_full in such
            // cases, to avoid divison by zero. See https://github.com/svartalf/rust-battery/pull/5
            State::Charging if energy_rate != 0 => {
                // Some drivers might report that `energy_full` is lower than `energy`,
                // but battery is still charging. What should we do in that case?
                // As for now, assuming that battery is fully charged, since we can't guess,
                // how much time left.
                let energy_left = match self.energy_full().checked_sub(self.energy()) {
                    Some(value) => value,
                    None => return None,
                };
                let time_to_full = 3600 * energy_left / energy_rate;
                if time_to_full > (20 * 60 * 60) {
                    None
                } else {
                    Some(Duration::from_secs(u64::from(time_to_full)))
                }
            },
            _ => None,
        }
    }

    fn time_to_empty(&self) -> Option<Duration> {
        let energy_rate = self.energy_rate();
        match self.state() {
            // In some cases energy_rate can be 0 while Discharging, for example just after
            // unplugging the charger. Assume that the battery doesn't have time_to_empty in such
            // cases, to avoid divison by zero. See https://github.com/svartalf/rust-battery/pull/5
            State::Discharging if energy_rate != 0 => {
                let time_to_empty = 3600 * self.energy() / energy_rate;
                if time_to_empty > (240 * 60 * 60) { // Ten days for discharging
                    None
                } else {
                    Some(Duration::from_secs(u64::from(time_to_empty)))
                }
            },
            _ => None,
        }
    }

}

