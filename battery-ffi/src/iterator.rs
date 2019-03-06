use std::ptr;

use crate::{Batteries, Battery};

/// Gets next iteration over batteries iterator.
///
/// Caller is required to call [battery_free](fn.battery_free.html) in order
/// to properly free memory for the returned battery instance.
///
/// Caller is required to call [battery_iterator_free](fn.battery_iterator_free.html)
/// if order to properly free memory for the returned batteries iterator instance.
///
/// # Panics
///
/// This function will panic if any passed pointer is `NULL`
///
/// # Returns
///
/// If there is no batteries left to iterate, this function returns `NULL`,
/// otherwise it returns pointer to next battery.
#[no_mangle]
pub unsafe extern "C" fn battery_iterator_next(ptr: *mut Batteries) -> *mut Battery {
    assert!(!ptr.is_null());
    let iterator = &mut *ptr;

    match iterator.next() {
        None => ptr::null_mut(),
        Some(battery) => Box::into_raw(Box::new(battery)),
    }
}

/// Frees previously created batteries iterator.
#[no_mangle]
pub unsafe extern "C" fn battery_iterator_free(ptr: *mut Batteries) {
    if ptr.is_null() {
        return;
    }

    Box::from_raw(ptr);
}
