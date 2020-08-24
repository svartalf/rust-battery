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
//! For a quick example see the [Manager](struct.Manager.html) type documentation
//! or [`simple.rs`](https://github.com/svartalf/rust-battery/blob/master/battery/examples/simple.rs)
//! file in the `examples/` folder.
//!
//! [battop](https://crates.io/crates/battop) crate is using this library as a knowledge source,
//! so check it out too for a real-life example.

#![deny(unused)]
#![deny(unstable_features)]
#![deny(bare_trait_objects)]
#![allow(clippy::manual_non_exhaustive)]  // MSRV is 1.36
#![doc(html_root_url = "https://docs.rs/battery/0.7.6")]

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
pub mod errors;
mod platform;

pub use self::errors::{Error, Result};
pub use self::types::{Batteries, Battery, Manager, State, Technology};
