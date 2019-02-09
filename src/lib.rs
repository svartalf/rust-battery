//! This crate provides cross-platform information about batteries.
//!
//! Gives access to a system independent battery state, capacity, charge and voltage values
//! recalculated as necessary to be returned in mW, mWh or V units.
//!
//! ## Supported platforms
//!
//! * Linux 2.6.39+

mod info;
mod platform;

pub use info::{Battery, State};
pub use platform::get;
