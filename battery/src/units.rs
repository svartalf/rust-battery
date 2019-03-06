//! Partially re-exported [uom](https://crates.io/crates/uom) quantities and measurement units
//! used in the library public interface.
//!
//! Public methods for [Battery](struct.Battery.html) are returning these types.\
//! Internally values in these types are stored as a [SI measurement units](https://www.bipm.org/en/measurement-units/),
//! so, for example, if you want to get the battery energy in the `watt·hour` units
//! instead of the default `joules`, you will need the measurement unit from the corresponding module:
//!
//! ```edition2018
//! use battery::units::energy::watt_hour;
//!
//! for bat in battery::Manager::new().iter() {
//!     println!("Energy: {} Wh", bat.energy().get::<watt_hour>());
//! }
//! ```
//!
//! Same thing applies to other units (temperature is stored in Kelvins):
//!
//! ```edition2018
//! use battery::units::thermodynamic_temperature::degree_celsius;
//!
//! for bat in battery::Manager::new().iter() {
//!     if let Some(value) = bat.temperature() {
//!         println!("Temperature: {} °C", value.get::<degree_celsius>());
//!     }
//! }
//! ```
//!
//! percents:
//!
//! ```edition2018
//! use battery::units::ratio::percent;
//!
//! for bat in battery::Manager::new().iter() {
//!     println!("State of charge: {} %", bat.state_of_charge().get::<percent>());
//! }
//! ```
//!
//! or time:
//!
//! ```edition2018
//! use std::time::Duration;
//!
//! use battery::units::time::nanosecond;
//!
//! for bat in battery::Manager::new().iter() {
//!     if let Some(value) = bat.time_to_full() {
//!         let duration = Duration::from_nanos(value.get::<nanosecond>() as u64);
//!     }
//! }
//! ```

#![allow(unused_macros)]

// Re-exports for easier crate usage
pub use uom::si::f32::{
    ElectricCharge, ElectricCurrent, ElectricPotential, Energy, Power, Ratio,
    ThermodynamicTemperature, Time,
};
pub use uom::si::Unit;
pub use uom::si::{
    electric_charge, electric_current, electric_potential, energy, power, ratio,
    thermodynamic_temperature, time,
};

use num_traits::ToPrimitive;

// Macros and traits for a quicker conversion into uom types.
// Instead of macros there can be functions, but macros are visually different from function calls
// in the most editors, and since there are a lot of different measurement units used,
// I think it is a nice idea to highlight conversions, just to be sure proper units are used.

pub(crate) trait IntoQuantity<T>
where
    T: ToPrimitive,
{
    type Quantity;

    fn from_primitive(value: T) -> Self::Quantity;
}

macro_rules! impl_into_quantity {
    ($unit:path, $quantity:ty) => {
        impl<T> IntoQuantity<T> for $unit
        where
            T: ToPrimitive,
        {
            type Quantity = $quantity;

            fn from_primitive(value: T) -> Self::Quantity {
                match &value.to_f32() {
                    Some(value) => Self::Quantity::new::<$unit>(*value),
                    None => unreachable!(),
                }
            }
        }
    };
}

impl_into_quantity!(electric_charge::milliampere_hour, ElectricCharge);
impl_into_quantity!(electric_charge::microampere_hour, ElectricCharge);
impl_into_quantity!(energy::milliwatt_hour, Energy);
impl_into_quantity!(energy::microwatt_hour, Energy);
impl_into_quantity!(electric_current::milliampere, ElectricCurrent);
impl_into_quantity!(electric_current::microampere, ElectricCurrent);
impl_into_quantity!(power::watt, Power);
impl_into_quantity!(power::milliwatt, Power);
impl_into_quantity!(power::microwatt, Power);
impl_into_quantity!(electric_potential::volt, ElectricPotential);
impl_into_quantity!(electric_potential::millivolt, ElectricPotential);
impl_into_quantity!(electric_potential::microvolt, ElectricPotential);
impl_into_quantity!(
    thermodynamic_temperature::degree_celsius,
    ThermodynamicTemperature
);
impl_into_quantity!(
    thermodynamic_temperature::decikelvin,
    ThermodynamicTemperature
);
impl_into_quantity!(ratio::percent, Ratio);
impl_into_quantity!(time::second, Time);
impl_into_quantity!(time::minute, Time);

/// Create `ElectricCharge` quantity with `milliampere_hour` unit
macro_rules! milliampere_hour {
    ($value:expr) => {
        unit!($crate::units::electric_charge::milliampere_hour, $value)
    };
}

/// Create `ElectricCharge` quantity with `microampere_hour` unit
macro_rules! microampere_hour {
    ($value:expr) => {
        unit!($crate::units::electric_charge::microampere_hour, $value)
    };
}

/// Create `Energy` quantity with `milliwatt_hour` unit
macro_rules! milliwatt_hour {
    ($value:expr) => {
        unit!($crate::units::energy::milliwatt_hour, $value)
    };
}

/// Create `Energy` quantity with `microwatt_hour` unit
macro_rules! microwatt_hour {
    ($value:expr) => {
        unit!($crate::units::energy::microwatt_hour, $value)
    };
}

/// Create `ElectricCurrent` quantity with `milliampere` unit
macro_rules! milliampere {
    ($value:expr) => {
        unit!($crate::units::electric_current::milliampere, $value)
    };
}
/// Create `ElectricCurrent` quantity with `microampere` unit
macro_rules! microampere {
    ($value:expr) => {
        unit!($crate::units::electric_current::microampere, $value)
    };
}

/// Create `Power` quantity with `watt` unit
macro_rules! watt {
    ($value:expr) => {
        unit!($crate::units::power::watt, $value)
    };
}

/// Create `Power` quantity with `milliwatt` unit
macro_rules! milliwatt {
    ($value:expr) => {
        unit!($crate::units::power::milliwatt, $value)
    };
}

/// Create `Power` quantity with `microwatt` unit
macro_rules! microwatt {
    ($value:expr) => {
        unit!($crate::units::power::microwatt, $value)
    };
}

/// Create `ElectricPotential` quantity with `millivolt` unit
macro_rules! millivolt {
    ($value:expr) => {
        unit!($crate::units::electric_potential::millivolt, $value)
    };
}

/// Create `ElectricPotential` quantity with `microvolt` unit
macro_rules! microvolt {
    ($value:expr) => {
        unit!($crate::units::electric_potential::microvolt, $value)
    };
}

/// Create `ElectricPotential` quantity with `volt` unit
macro_rules! volt {
    ($value:expr) => {
        unit!($crate::units::electric_potential::volt, $value)
    };
}

/// Create `Ratio` quantity with `percent` unit
macro_rules! percent {
    ($value:expr) => {
        unit!($crate::units::ratio::percent, $value)
    };
}

/// Create `ThermodynamicTemperature` quantity with `degree_celsius` unit
macro_rules! celsius {
    ($value:expr) => {
        unit!(
            $crate::units::thermodynamic_temperature::degree_celsius,
            $value
        )
    };
}

/// Create `ThermodynamicTemperature` quantity with `decikelvin` unit
macro_rules! decikelvin {
    ($value:expr) => {
        unit!($crate::units::thermodynamic_temperature::decikelvin, $value)
    };
}

/// Create `Time` quantity with `second` unit
macro_rules! second {
    ($value:expr) => {
        unit!($crate::units::time::second, $value)
    };
}

/// Create `Time` quantity with `inute` unit
macro_rules! minute {
    ($value:expr) => {
        unit!($crate::units::time::minute, $value)
    };
}

macro_rules! unit {
    ($unit:ty, $value:expr) => {
        <$unit as $crate::units::IntoQuantity<_>>::from_primitive($value)
    };
}
