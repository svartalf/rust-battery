//! This crate provides C ABI interface for [battery](https://crates.io/crate/battery) crate.
//!
//! # Bindings generation
//!
//! Among library creation this crate generates `battery_ffi.h` file,
//! which might be useful for automatic bindings generation or just with plain `C`/`C++` development.
//!
//! After build it will be located somewhere at `target/*/build/battery-ffi-*/out/`,
//! depending on build profile (`debug`/`release`) and build hash.
//!
//! # Examples
//!
//! ```c
//! #include "battery_ffi.h"
//!
//! void main() {
//!    Manager *manager = battery_manager_new();
//!    Batteries *iterator = battery_manager_iter(manager);
//!    while (true) {
//!        Battery *battery = battery_iterator_next(iterator);
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

// cbindgen==0.8.0 fails to export typedefs for opaque pointers
// from the battery crate, if this line is missing
extern crate battery as battery_lib;

mod battery;
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
pub use self::iterator::*;
pub use self::manager::*;
pub use self::state::*;
pub use self::technology::*;
