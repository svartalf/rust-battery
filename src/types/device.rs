use std::time::Duration;

use crate::{State, Technology};


pub trait Device {
    // TODO: Cycle count

    fn capacity(&self) -> f32;

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
        match self.state() {
            State::Charging => {
                // TODO: Possible division by zero
                let time_to_full = 3600 * (self.energy_full() - self.energy()) / self.energy_rate();
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
        match self.state() {
            State::Discharging => {
                // TODO: Possible division by zero
                let time_to_empty = 3600 * self.energy() / self.energy_rate();
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
