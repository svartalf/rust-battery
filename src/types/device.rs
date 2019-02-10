use std::time::Duration;

use crate::{State, Technology};


pub trait Device {
    fn capacity(&self) -> f64;

    fn energy(&self) -> f64;

    fn energy_full(&self) -> f64;

    fn energy_full_design(&self) -> f64;

    fn energy_rate(&self) -> f64;

    fn percentage(&self) -> f64;

    fn state(&self) -> State;

    fn voltage(&self) -> f64;

    fn temperature(&self) -> f64;

    fn vendor(&self) -> Option<&str>;

    fn model(&self) -> Option<&str>;

    fn serial_number(&self) -> Option<&str>;

    fn technology(&self) -> Technology;

    fn time_to_full(&self) -> Option<Duration>;

    fn time_to_empty(&self) -> Option<Duration>;

}
