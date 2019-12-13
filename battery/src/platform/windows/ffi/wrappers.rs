// Wrappers around the FFI things that should be freed later.
// It is better to Drop than free them manually.

use std::ops;

use winapi::ctypes::c_void;
use winapi::shared::ntdef;
use winapi::um::{handleapi, setupapi, winbase};

#[derive(Debug)]
pub struct InterfaceDetailData(setupapi::PSP_DEVICE_INTERFACE_DETAIL_DATA_W);

impl From<setupapi::PSP_DEVICE_INTERFACE_DETAIL_DATA_W> for InterfaceDetailData {
    fn from(p: setupapi::PSP_DEVICE_INTERFACE_DETAIL_DATA_W) -> Self {
        Self(p)
    }
}

impl ops::Deref for InterfaceDetailData {
    type Target = setupapi::PSP_DEVICE_INTERFACE_DETAIL_DATA_W;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for InterfaceDetailData {
    fn drop(&mut self) {
        let res = unsafe { winbase::LocalFree(self.0 as *mut c_void) };
        debug_assert_eq!(res, ntdef::NULL, "Unable to free device interface detail data");
    }
}

#[derive(Debug)]
pub struct Handle(ntdef::HANDLE);

impl From<ntdef::HANDLE> for Handle {
    fn from(handle: ntdef::HANDLE) -> Self {
        Self(handle)
    }
}

impl ops::Deref for Handle {
    type Target = ntdef::HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Handle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let res = unsafe { handleapi::CloseHandle(self.0) };
        debug_assert_ne!(res, 0, "Unable to close device handle");
    }
}
