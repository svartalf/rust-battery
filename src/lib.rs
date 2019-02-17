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
//! For a quick example see either [get](fn.get.html) function or `main.rs` file in the sources.

#[macro_use] extern crate cfg_if;

#[cfg(target_os = "windows")]
#[macro_use] extern crate winapi;

mod types;
mod platform;

pub use types::{Battery, State, Technology};
pub use platform::get;
