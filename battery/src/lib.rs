//! This crate provides cross-platform information about batteries.
//!
//! Gives access to a system independent battery state, capacity, charge and voltage values
//! recalculated as necessary to be returned in [SI measurement units](https://www.bipm.org/en/measurement-units/).
//!
//! ## Supported platforms
//!
//! * Linux 2.6.39+
//! * MacOS 10.10+
//! * Windows 7+
//! * FreeBSD
//! * DragonFlyBSD
//!
//! ## Examples
//!
//! For a quick example see [Manager](struct.Manager.html) type or `simple.rs` file in the `examples/` folder.
//!
//! For a real-live example see [battery-cli](https://crates.io/crates/battery-cli) crate.

#![deny(unused)]
#![deny(unstable_features)]

#[macro_use]
extern crate cfg_if;

#[cfg(target_os = "windows")]
#[macro_use]
extern crate winapi;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
#[macro_use]
extern crate nix;

mod types;
#[macro_use]
pub mod units;
mod platform;

pub use self::types::{Batteries, Battery, Manager, State, Technology};
