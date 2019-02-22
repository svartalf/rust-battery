use std::ptr;
use std::f32;
use std::u32;
use std::ffi::CString;

use crate::Battery;
use crate::technology::Technology;
use crate::state::State;

/// Returns battery percentage.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_percentage(ptr: *mut Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.percentage()
}

/// Returns battery energy (in `mWh`).
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_energy(ptr: *mut Battery) -> libc::uint32_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy()
}

/// Returns battery energy (in `mWh`) when it is considered full.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_energy_full(ptr: *mut Battery) -> libc::uint32_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy_full()
}

/// Returns battery energy (in `mWh`) designed to hold when it is considered full.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_energy_full_design(ptr: *mut Battery) -> libc::uint32_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy_full_design()
}

/// Returns battery energy rate (in `mW`).
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_energy_rate(ptr: *mut Battery) -> libc::uint32_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.energy_rate()
}

/// Returns battery voltage (in `mV`)
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_voltage(ptr: *mut Battery) -> libc::uint32_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.voltage()
}

/// Returns battery capacity in `0.0`..`100.0` percents range.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_capacity(ptr: *mut Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.capacity()
}

/// Returns battery state.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_state(ptr: *mut Battery) -> State {
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
pub unsafe extern fn battery_get_technology(ptr: *mut Battery) -> Technology {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    battery.technology().into()
}

/// Returns battery temperature.
///
/// # Returns
///
/// If value is not available, function returns max possible value for `float` type (`1E+37`).
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_temperature(ptr: *mut Battery) -> libc::c_float {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.temperature() {
        None => f32::MAX,
        Some(temp) => temp,
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
pub unsafe extern fn battery_get_cycle_count(ptr: *mut Battery) -> libc::uint32_t {
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
pub unsafe extern fn battery_get_vendor(ptr: *mut Battery) -> *mut libc::c_char {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.vendor() {
        Some(ref vendor) => {
            let c_str = CString::new(*vendor).unwrap();
            c_str.into_raw()
        },
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
pub unsafe extern fn battery_get_model(ptr: *mut Battery) -> *mut libc::c_char {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.model() {
        Some(ref model) => {
            let c_str = CString::new(*model).unwrap();
            c_str.into_raw()
        },
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
pub unsafe extern fn battery_get_serial_number(ptr: *mut Battery) -> *mut libc::c_char {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.serial_number() {
        Some(ref sn) => {
            let c_str = CString::new(*sn).unwrap();
            c_str.into_raw()
        },
        None => ptr::null_mut(),
    }
}

/// Returns battery time to full.
///
/// # Returns
///
/// If battery is not charging at the moment, this function will return `0`,
/// otherwise it will return seconds amount.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_time_to_full(ptr: *mut Battery) -> libc::uint64_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.time_to_full() {
        None => 0,
        Some(duration) => duration.as_secs(),
    }
}

/// Returns battery time to empty.
///
/// # Returns
///
/// If battery is not discharging at the moment, this function will return `0`,
/// otherwise it will return seconds amount.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern fn battery_get_time_to_empty(ptr: *mut Battery) -> libc::uint64_t {
    assert!(!ptr.is_null());
    let battery = &*ptr;

    match battery.time_to_empty() {
        None => 0,
        Some(duration) => duration.as_secs(),
    }
}

/// Frees battery instance.
///
/// Caller is required to call this function when battery pointer is not needed anymore
/// in order to properly free memory.
#[no_mangle]
pub unsafe extern fn battery_free(ptr: *mut Battery) {
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
pub unsafe extern fn battery_str_free(ptr: *mut libc::c_char) {
    if ptr.is_null() {
        return;
    }

    CString::from_raw(ptr);
}
