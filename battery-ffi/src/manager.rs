use crate::{Batteries, Battery, Manager};

/// Creates new batteries manager instance.
///
/// Returns opaque pointer to it. Caller is required to call [battery_manager_free](fn.battery_manager_free.html)
/// to properly free memory.
#[no_mangle]
pub extern "C" fn battery_manager_new() -> *mut Manager {
    Box::into_raw(Box::new(Manager::new()))
}

/// Creates an iterator over batteries from manager instance.
///
/// See [iterator_next](fn.battery_iterator_next.html) function for iterating over batteries.
///
/// # Panics
///
/// This function will panic if passed pointer is `NULL`
#[no_mangle]
pub unsafe extern "C" fn battery_manager_iter(ptr: *mut Manager) -> *mut Batteries {
    assert!(!ptr.is_null());
    let manager = &*ptr;

    Box::into_raw(Box::new(manager.iter()))
}

/// Refreshes battery information.
///
/// # Panics
///
/// This function will panic if any passed pointer is `NULL`
///
/// # Returns
///
/// `0` if everything is okay, `1` if refresh failed and `battery_ptr` contains stale information.
pub unsafe extern "C" fn battery_manager_refresh(
    manager_ptr: *mut Manager,
    battery_ptr: *mut Battery,
) -> libc::c_int {
    assert!(!manager_ptr.is_null());
    let manager = &mut *manager_ptr;

    assert!(!battery_ptr.is_null());
    let mut battery = &mut *battery_ptr;

    // TODO: Should there be better error handling?
    match manager.refresh(&mut battery) {
        Ok(_) => 0,
        Err(_) => 1,
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
