//! This crate provides cross-platform information about batteries.
//!
//! Gives access to a system independent battery state, capacity, charge and voltage values
//! recalculated as necessary to be returned in `mW`, `mWh` or `mV` units.
//!
//! ## Supported platforms
//!
//! * Linux 2.6.39+
//! * MacOS (10.10+ probably, needs to be confirmed)
//! * Windows 7+
//!
//! ## Examples
//!
//! For a quick example see [Manager](struct.Manager.html) type.
//!
//! For a real-live example see [battery-cli](https://crates.io/crate/battery-cli) crate.

#[macro_use] extern crate cfg_if;

#[cfg(target_os = "windows")]
#[macro_use] extern crate winapi;

mod types;
mod platform;

pub use types::{Manager, Batteries, Battery, State, Technology};
