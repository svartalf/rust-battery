//! This crate provides C ABI interface for [battery](https://crates.io/crate/battery) crate.
//!
//! # Bindings generation
//!
//! Among library creation this crate generates `battery_ffi.h` file, enabled by default by `cbindgen` feature,
//! which might be useful for automatic bindings generation or just with plain `C`/`C++` development.
//!
//! After build it will be located somewhere at `target/*/build/battery-ffi-*/out/`,
//! depending on build profile (`debug`/`release`) and build hash.
//!
//! Disabling `cbindgen` feature might speed up compilation a little bit,
//! especially if you don't need the header file.
//!
//! # Examples
//!
//! ```c
//! #include "battery_ffi.h"
//!
//! void main() {
//!    Manager *manager = battery_manager_new();
//!    // .. handle `manager == NULL` here ..
//!    Batteries *iterator = battery_manager_iter(manager);
//!    // .. handle `iterator == NULL` here ..
//!    while (true) {
//!        Battery *battery = battery_iterator_next(iterator);
//!        // .. handle possible error here ..
//!        if (battery == NULL) {
//!            break;
//!        }
//!
//!        // Use some `battery_get_*` functions here
//!
//!        battery_free(battery);
//!    }
//!
//!    battery_iterator_free(iterator);
//!    battery_manager_free(manager);
//! }
//! ```
//!
//! Also, check the `examples/` directory in the repository for examples with C and Python.

// cbindgen==0.8.0 fails to export typedefs for opaque pointers
// from the battery crate, if this line is missing
extern crate battery as battery_lib;

mod battery;
mod errors;
mod iterator;
mod manager;
mod state;
mod technology;

/// Opaque struct representing battery manager.
///
/// End users should consider it as a some memory somewhere in the heap,
/// and work with it only via library methods.
pub type Manager = battery_lib::Manager;

/// Opaque struct representing batteries iterator.
///
/// End users should consider it as a some memory somewhere in the heap,
/// and work with it only via library methods.
pub type Batteries = battery_lib::Batteries;

/// Opaque struct representing battery.
///
/// End users should consider it as a some memory somewhere in the heap,
/// and work with it only via library methods.
pub type Battery = battery_lib::Battery;

pub use self::battery::*;
pub use self::errors::{battery_last_error_length, battery_last_error_message};
pub use self::iterator::*;
pub use self::manager::*;
pub use self::state::*;
pub use self::technology::*;
