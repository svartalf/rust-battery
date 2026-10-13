//! An owned string allocated and freed by the battery library.
use core::{ffi::CStr, ptr::NonNull};

use co3::{ReprC, ffi::c_char, rust_spec::RustSpec};

/// An owned UTF-8 string returned by the battery library.
#[derive(RustSpec, ReprC)]
#[rust_spec(custom_drop)]
#[repr_c(is_valid = |ptr| unsafe {
    // If battery doesn't nul terminate this is UB
    CStr::from_ptr(ptr.as_ptr().cast()) }.to_str().is_ok()
)]
#[repr(transparent)]
pub struct RemoteString(NonNull<c_char>);

impl Clone for RemoteString {
    fn clone(&self) -> Self {
        let ptr = self.as_c_str().to_owned().into_raw();
        Self(unsafe { NonNull::new_unchecked(ptr.cast()) })
    }
}

impl RemoteString {
    // Extracts a `CStr` slice containing the entire string.
    fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_ptr().cast()) }
    }

    // Extracts a string slice containing the entire `String`.
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_c_str().to_bytes()) }
    }
}
