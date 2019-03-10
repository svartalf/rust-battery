use std::ptr;

use crate::{Batteries, Battery, Manager};

/// Creates new batteries manager instance.
///
/// # Returns
///
/// Returns opaque pointer to manager instance.
/// Caller is required to call [battery_manager_free](fn.battery_manager_free.html)
/// to properly free memory.
///
/// `NULL` pointer might be returned if manager creation had failed.
/// Caller can check [battery_last_error_message](fn.battery_last_error_message.html)
/// for error details.
#[no_mangle]
pub extern "C" fn battery_manager_new() -> *mut Manager {
    match Manager::new() {
        Ok(manager) => Box::into_raw(Box::new(manager)),
        Err(e) => {
            crate::errors::set_last_error(e);
            ptr::null_mut()
        }
    }
}

/// Creates an iterator over batteries from manager instance.
///
/// See [iterator_next](fn.battery_iterator_next.html) function for iterating over batteries.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
///
/// # Returns
///
/// `NULL` pointer will be returned if iterator creation had failed.
/// Caller can check [battery_last_error_message](fn.battery_last_error_message.html)
/// for error details.
#[no_mangle]
pub unsafe extern "C" fn battery_manager_iter(ptr: *mut Manager) -> *mut Batteries {
    assert!(!ptr.is_null());
    let manager = &*ptr;

    match manager.batteries() {
        Ok(iterator) => Box::into_raw(Box::new(iterator)),
        Err(e) => {
            crate::errors::set_last_error(e);
            ptr::null_mut()
        }
    }
}

/// Refreshes battery information.
///
/// # Panics
///
/// This function will panic if any passed pointer is `NULL`
///
/// # Returns
///
/// `0` if everything is okay, `-1` if refresh failed and `battery_ptr` contains stale information.
pub unsafe extern "C" fn battery_manager_refresh(manager_ptr: *mut Manager, battery_ptr: *mut Battery) -> libc::c_int {
    assert!(!manager_ptr.is_null());
    let manager = &mut *manager_ptr;

    assert!(!battery_ptr.is_null());
    let mut battery = &mut *battery_ptr;

    match manager.refresh(&mut battery) {
        Ok(_) => 0,
        Err(e) => {
            crate::errors::set_last_error(e);
            -1
        }
    }
}

/// Frees manager instance.
#[no_mangle]
pub unsafe extern "C" fn battery_manager_free(ptr: *mut Manager) {
    if ptr.is_null() {
        return;
    }

    Box::from_raw(ptr);
}
