use std::io;

use uom::si::time::{day, hour};
use num_traits::identities::Zero;

use crate::units::{ElectricPotential, Energy, Power, Ratio, ThermodynamicTemperature, Time};
use crate::{Battery, State, Technology};

pub trait BatteryManager: Default + Sized {
    fn refresh(&mut self, battery: &mut Battery) -> io::Result<()>;
}

pub trait BatteryIterator: Iterator<Item = Battery> + Sized {}

pub trait BatteryDevice: Sized {
    fn state_of_health(&self) -> Ratio {
        self.energy_full() / self.energy_full_design()
    }

    fn state_of_charge(&self) -> Ratio {
        self.energy() / self.energy_full()
    }

    fn energy(&self) -> Energy;

    fn energy_full(&self) -> Energy;

    fn energy_full_design(&self) -> Energy;

    fn energy_rate(&self) -> Power;

    fn state(&self) -> State;

    fn voltage(&self) -> ElectricPotential;

    fn temperature(&self) -> Option<ThermodynamicTemperature>;

    fn vendor(&self) -> Option<&str>;

    fn model(&self) -> Option<&str>;

    fn serial_number(&self) -> Option<&str>;

    fn technology(&self) -> Technology;

    fn cycle_count(&self) -> Option<u32>;

    // Default implementation for `time_to_full` and `time_to_empty`
    // uses calculation based on the current energy flow,
    // but if device provides by itself provides these **instant** values (do not use average values),
    // it would be easier and cheaper to return them instead of making some calculations

    fn time_to_full(&self) -> Option<Time> {
        let energy_rate = self.energy_rate();
        match self.state() {
            // In some cases energy_rate can be 0 while Charging, for example just after
            // plugging in the charger. Assume that the battery doesn't have time_to_full in such
            // cases, to avoid divison by zero. See https://github.com/svartalf/rust-battery/pull/5
            State::Charging if !energy_rate.is_zero() => {
                // Some drivers might report that `energy_full` is lower than `energy`,
                // but battery is still charging. What should we do in that case?
                // As for now, assuming that battery is fully charged, since we can't guess,
                // how much time left.
                let energy_left = match self.energy_full() - self.energy() {
                    value if value.is_sign_positive() => value,
                    _ => return None,
                };

                let time_to_full = energy_left / energy_rate;
                if time_to_full.get::<hour>() > 10.0 {
                    // Ten hours for charging is too much
                    None
                } else {
                    Some(time_to_full)
                }
            }
            _ => None,
        }
    }

    fn time_to_empty(&self) -> Option<Time> {
        let energy_rate = self.energy_rate();
        match self.state() {
            // In some cases energy_rate can be 0 while Discharging, for example just after
            // unplugging the charger. Assume that the battery doesn't have time_to_empty in such
            // cases, to avoid divison by zero. See https://github.com/svartalf/rust-battery/pull/5
            State::Discharging if !energy_rate.is_zero() => {
                let time_to_empty = self.energy() / energy_rate;
                if time_to_empty.get::<day>() > 10.0 {
                    // Ten days for discharging is too much
                    None
                } else {
                    Some(time_to_empty)
                }
            }
            _ => None,
        }
    }
}

/// For values in `0…1` ratio (or `0…100` %).
///
/// Method `bound` caps value into this range.
pub trait Bound: Sized {
    fn into_bounded(self) -> Self;
}

impl Bound for Ratio {
    #[inline]
    fn into_bounded(mut self) -> Self {
        if self.value < 0.0 {
            self.value = 0.0;
        }

        if self.value > 1.0 {
            self.value = 1.0;
        }

        self
    }
}
