//! This crate provides cross-platform information about batteries.
//!
//! Gives access to a system independent battery state, capacity, charge and voltage values
//! recalculated as necessary to be returned in W, Wh or V units.
//!
//! ## Supported platforms
//!
//! * Linux 2.6.39+
//! * MacOS (10.10+ probably, needs to be confirmed)

#[macro_use] extern crate cfg_if;

mod types;
mod platform;

pub use types::{Battery, State, Technology};
pub use platform::get;
