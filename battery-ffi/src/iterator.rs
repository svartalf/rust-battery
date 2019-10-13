use std::ptr;

use crate::{Batteries, Battery};

/// Gets next iteration over batteries iterator.
///
/// Caller is required to call [battery_free](fn.battery_free.html) in order
/// to properly free memory for the returned battery instance.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`.
///
/// # Returns
///
/// Returns pointer to next battery.
///
/// If there is no batteries left to iterate or some error happened, this function will return `NULL`.
///
/// Caller is required to differentiate between these two cases and should check
/// if there was any error with [battery_have_last_error](fn.battery_have_last_error.html).
///
/// If there is no batteries left, `battery_have_last_error` will return `0`.
#[no_mangle]
pub unsafe extern "C" fn battery_iterator_next(ptr: *mut Batteries) -> *mut Battery {
    assert!(!ptr.is_null());
    let iterator = &mut *ptr;

    match iterator.next() {
        None => {
            crate::errors::clear_last_error();
            ptr::null_mut()
        }
        Some(Ok(battery)) => Box::into_raw(Box::new(battery)),
        Some(Err(e)) => {
            crate::errors::set_last_error(e);
            ptr::null_mut()
        }
    }
}

/// Frees previously created batteries iterator.
#[no_mangle]
pub unsafe extern "C" fn battery_iterator_free(ptr: *mut Batteries) {
    if ptr.is_null() {
        return;
    }

    let _ = Box::from_raw(ptr);
}
