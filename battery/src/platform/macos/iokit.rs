use std::io;
use std::i32;
use std::mem;
use std::time::Duration;
use std::ffi::{CStr, CString};

use CoreFoundation_sys as core;
use IOKit_sys as iokit;
use mach::{port, kern_return};

use super::traits::DataSource;

const IOPM_SERVICE_NAME: *const libc::c_char = b"IOPMPowerSource\0".as_ptr() as *const libc::c_char;

#[derive(Debug)]
pub struct PowerSource(core::dictionary::CFMutableDictionaryRef);

impl PowerSource {
    pub fn get_props() -> io::Result<PowerSource> {
        let mut master_port: port::mach_port_t = port::MACH_PORT_NULL;

        let res = unsafe {
            iokit::IOMasterPort(iokit::kIOMasterPortDefault, &mut master_port)
        };
        if res != kern_return::KERN_SUCCESS {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        };

        // `IOServiceMatchingService` consumes `service`, so we do not need to CFRelease it manually
        let service = unsafe {
            iokit::IOServiceMatching(IOPM_SERVICE_NAME)
        };

        // It is required to release this object later
        let battery_service = unsafe {
            iokit::IOServiceGetMatchingService(master_port, service)
        };

        let mut props: core::dictionary::CFMutableDictionaryRef = unsafe {
            mem::uninitialized()
        };
        let prop_res = unsafe {
            iokit::IORegistryEntryCreateCFProperties(battery_service, &mut props,
            core::kCFAllocatorDefault, 0)
        };
        if prop_res != kern_return::KERN_SUCCESS {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        // Uncomment this to see all existing keys in the `props`.
        // Will write to stderr. Do not use in production.
        //
        // unsafe { core::CFShow(props as *const libc::c_void); }

        unsafe {
            iokit::IOObjectRelease(battery_service);
        }

        Ok(PowerSource(props))
    }

    pub fn get_bool(&self, key: &[u8]) -> Option<bool> {
        if let Some(value_ptr) = self.get_dict_value_ptr(key) {
            unsafe {
                debug_assert!(core::CFGetTypeID(value_ptr) == core::CFBooleanGetTypeID());
            }

            match unsafe { core::CFBooleanGetValue(value_ptr as core::CFBooleanRef) } {
                0 => Some(false),
                1 => Some(true),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    pub fn get_isize(&self, key: &[u8]) -> Option<isize> {
        if let Some(value_ptr) = self.get_dict_value_ptr(key) {
            unsafe {
                debug_assert!(core::CFGetTypeID(value_ptr) == core::CFNumberGetTypeID());
            }

            let mut value = 0isize;
            let res = unsafe {
                core::CFNumberGetValue(
                    value_ptr as core::CFNumberRef,
                    core::kCFNumberNSIntegerType,
                    &mut value as *mut _ as *mut libc::c_void
                )
            };
            if res == 1 {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_u32(&self, key: &[u8]) -> Option<u32> {
        if let Some(value_ptr) = self.get_dict_value_ptr(key) {
            unsafe {
                debug_assert!(core::CFGetTypeID(value_ptr) == core::CFNumberGetTypeID());
            }

            let mut value = 0u32;
            let res = unsafe {
                core::CFNumberGetValue(
                    value_ptr as core::CFNumberRef,
                    core::kCFNumberIntType,
                    &mut value as *mut _ as *mut libc::c_void
                )
            };
            if res == 1 {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_i32(&self, key: &[u8]) -> Option<i32> {
        if let Some(value_ptr) = self.get_dict_value_ptr(key) {
            unsafe {
                debug_assert!(core::CFGetTypeID(value_ptr) == core::CFNumberGetTypeID());
            }

            let mut value = 0i32;
            let res = unsafe {
                core::CFNumberGetValue(
                    value_ptr as core::CFNumberRef,
                    core::kCFNumberIntType,
                    &mut value as *mut _ as *mut libc::c_void
                )
            };
            if res == 1 {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_string(&self, key: &[u8]) -> Option<String> {
        if let Some(value_ptr) = self.get_dict_value_ptr(key) {
            unsafe {
                debug_assert!(core::CFGetTypeID(value_ptr) == core::CFStringGetTypeID());
            }

            let mut buf = Vec::with_capacity(64);
            let result = unsafe {
                core::CFStringGetCString(
                    value_ptr as core::CFStringRef,
                    buf.as_mut_ptr(),
                    64,
                    core::kCFStringEncodingUTF8,
                )
            };
            if result == 0 {
                return None;
            }

            let value = unsafe {
                CStr::from_ptr(buf.as_ptr()).to_string_lossy()
            };

            Some(value.to_string())
        } else {
            None
        }
    }

    fn get_dict_value_ptr(&self, key: &[u8]) -> Option<*const libc::c_void> {
        let cstring = CString::new(key).expect("Malformed input for CString");
        let cfstring = unsafe {
            core::CFStringCreateWithCString(
                core::kCFAllocatorDefault,
                cstring.as_ptr(),
                core::kCFStringEncodingUTF8,
            )
        };

        if cfstring.is_null() {
            // TODO: Trace allocation error
            return None;
        }

        let value_ptr = unsafe {
            core::dictionary::CFDictionaryGetValue(self.0, cfstring as *const libc::c_void)
        };
        if value_ptr.is_null() {
            None
        } else {
            Some(value_ptr)
        }
    }

}

impl DataSource for PowerSource {
    fn new() -> io::Result<PowerSource> where Self: Sized {
        PowerSource::get_props()
    }

    fn fully_charged(&self) -> bool {
        self.get_bool(b"FullyCharged")
            .expect("IOKit is not providing required data")
    }

    fn external_connected(&self) -> bool {
        self.get_bool(b"ExternalConnected")
            .expect("IOKit is not providing required data")
    }

    fn is_charging(&self) -> bool {
        self.get_bool(b"IsCharging")
            .expect("IOKit is not providing required data")
    }

    fn voltage(&self) -> u32 {
        self.get_u32(b"Voltage")
            .expect("IOKit is not providing required data")
    }

    fn amperage(&self) -> i32 {
        self.get_i32(b"Amperage")
            .expect("IOKit is not providing required data")
    }

    fn design_capacity(&self) -> u32 {
        self.get_u32(b"DesignCapacity")
            .expect("IOKit is not providing required data")
    }

    fn max_capacity(&self) -> u32 {
        self.get_u32(b"MaxCapacity")
            .expect("IOKit is not providing required data")
    }

    fn current_capacity(&self) -> u32 {
        self.get_u32(b"CurrentCapacity")
            .expect("IOKit is not providing required data")
    }

    fn temperature(&self) -> Option<f32> {
        self.get_isize(b"Temperature")
            .map(|value| value as f32 / 100.0)
    }

    fn cycle_count(&self) -> Option<u32> {
        self.get_u32(b"CycleCount")
    }

    fn time_remaining(&self) -> Option<Duration> {
        self.get_i32(b"TimeRemaining")
            .and_then(|val| {
                if val == i32::MAX {
                    None
                } else {
                    // TODO: Is it possible to have negative `TimeRemaining`?
                    let seconds = val.abs() as u64 * 60;
                    Some(Duration::from_secs(seconds))
                }
            })
    }

    fn manufacturer(&self) -> Option<String> {
        self.get_string(b"Manufacturer")
    }

    fn device_name(&self) -> Option<String> {
        self.get_string(b"DeviceName")
    }

    fn serial_number(&self) -> Option<String> {
        self.get_string(b"BatterySerialNumber")
    }
}

impl Drop for PowerSource {
    fn drop(&mut self) {
        unsafe {
            core::CFRelease(self.0 as *const libc::c_void)
        }
    }
}
