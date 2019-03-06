use std::f32;
use std::ffi::CString;
use std::ptr;
use std::u32;

use crate::state::State;
use crate::technology::Technology;
use crate::Battery;

use battery::units::electric_potential::volt;
use battery::units::energy::joule;
use battery::units::power::watt;
use battery::units::ratio::percent;
use battery::units::thermodynamic_temperature::kelvin;
use battery::units::time::second;

/// Returns battery state of charge as a percentage value from `0.0` to `100.0`.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_state_of_charge(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.state_of_charge().get::<percent>()
}

/// Returns battery energy (in `joule`).
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_energy(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy().get::<joule>()
}

/// Returns battery energy (in `joule`) when it is considered full.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_energy_full(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy_full().get::<joule>()
}

/// Returns battery energy (in `joule`) designed to hold when it is considered full.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_energy_full_design(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy_full_design().get::<joule>()
}

/// Returns battery energy rate (in `W`).
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_energy_rate(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy_rate().get::<watt>()
}

/// Returns battery voltage (in `V`)
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_voltage(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.voltage().get::<volt>()
}

/// Returns battery state of health as a percentage value from `0.0` to `100.0`.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_state_of_health(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.state_of_health().get::<percent>()
}

/// Returns battery state.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_state(ptr: *const Battery) -> State {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.state().into()
}

/// Returns battery technology.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_technology(ptr: *const Battery) -> Technology {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.technology().into()
}

/// Returns battery temperature in Kelvin.
///
/// # Returns
///
/// If value is not available, function returns `NaN`.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_temperature(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.temperature() {
        None => f32::NAN,
        Some(temp) => temp.get::<kelvin>(),
    }
}

/// Returns battery cycles count.
///
/// # Returns
///
/// If value is not available, function returns max possible value for `uint32` type (`4294967295`).
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_cycle_count(ptr: *const Battery) -> libc::uint32_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.cycle_count() {
        None => u32::MAX,
        Some(value) => value,
    }
}

/// Returns battery vendor.
///
/// Caller is required to free returned value with [battery_str_free](fn.battery_str_free.html)
/// function after using it.
///
/// # Returns
///
/// This function might return `NULL` if vendor data is not available.
/// Calling [battery_str_free](fn.battery_str_free.html) is not required in that case,
/// yet it will not lead to any error.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_vendor(ptr: *const Battery) -> *mut libc::c_char {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.vendor() {
        Some(vendor) => {
            let c_str = CString::new(vendor).unwrap();
            c_str.into_raw()
        }
        None => ptr::null_mut(),
    }
}

/// Returns battery model.
///
/// Caller is required to free returned value with [battery_str_free](fn.battery_str_free.html)
/// function after using it.
///
/// # Returns
///
/// This function might return `NULL` if model data is not available.
/// Calling [battery_str_free](fn.battery_str_free.html) is not required in that case,
/// yet it will not lead to any error.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_model(ptr: *const Battery) -> *mut libc::c_char {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.model() {
        Some(model) => {
            let c_str = CString::new(model).unwrap();
            c_str.into_raw()
        }
        None => ptr::null_mut(),
    }
}

/// Returns battery serial number.
///
/// Caller is required to free returned value with [battery_str_free](fn.battery_str_free.html)
/// function after using it.
///
/// # Returns
///
/// This function might return `NULL` if serial number data is not available.
/// Calling [battery_str_free](fn.battery_str_free.html) is not required in that case,
/// yet it will not lead to any error.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_serial_number(ptr: *const Battery) -> *mut libc::c_char {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.serial_number() {
        Some(sn) => {
            let c_str = CString::new(sn).unwrap();
            c_str.into_raw()
        }
        None => ptr::null_mut(),
    }
}

/// Returns battery time to full.
///
/// # Returns
///
/// If battery is not charging at the moment, this function will return `NaN`,
/// otherwise it will return seconds amount.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_time_to_full(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.time_to_full() {
        None => f32::NAN,
        Some(duration) => duration.get::<second>(),
    }
}

/// Returns battery time to empty.
///
/// # Returns
///
/// If battery is not discharging at the moment, this function will return `NaN`,
/// otherwise it will return seconds amount.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_get_time_to_empty(ptr: *const Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.time_to_empty() {
        None => f32::NAN,
        Some(duration) => duration.get::<second>(),
    }
}

/// Frees battery instance.
///
/// Caller is required to call this function when battery pointer is not needed anymore
/// in order to properly free memory.
#[no_mangle]
pub unsafe extern "C" fn battery_free(ptr: *mut Battery) {
    if ptr.is_null() {
        return;
    }

    Box::from_raw(ptr);
}

/// Frees battery information string value.
///
/// Caller is required to call this function for return values for the following functions:
///  * [battery_vendor](fn.battery_vendor.html)
///  * [battery_model](fn.battery_model.html)
///  * [battery_serial_number](fn.battery_serial_number.html)
#[no_mangle]
pub unsafe extern "C" fn battery_str_free(ptr: *mut libc::c_char) {
    if ptr.is_null() {
        return;
    }

    CString::from_raw(ptr);
}
