use std::mem;
use std::ops::{Deref, DerefMut};

use core_foundation::base::{mach_port_t, kCFNull, kCFAllocatorDefault, CFType, TCFType};
use core_foundation::dictionary::{CFDictionary, CFMutableDictionary, CFMutableDictionaryRef};
use core_foundation::string::CFString;
use mach::{port, mach_port, kern_return, traps};

use crate::Result;
use super::{sys};

#[derive(Debug)]
pub struct IoMasterPort(mach_port_t);

impl IoMasterPort {
    pub fn new() -> Result<IoMasterPort> {
        let mut master_port: port::mach_port_t = port::MACH_PORT_NULL;

        unsafe {
            kern_try!(sys::IOMasterPort(sys::kIOMasterPortDefault, &mut master_port));
        }

        Ok(IoMasterPort(master_port))
    }

    pub fn get_services(&self) -> Result<IoIterator> {
        let service = unsafe {
            let ret = sys::IOServiceMatching(sys::IOPM_SERVICE_NAME);
            assert_ne!(ret as *const _, kCFNull);

            ret
        };

        let mut iterator = IoIterator::default();

        unsafe {
            kern_try!(sys::IOServiceGetMatchingServices(self.0, service, &mut *iterator));
        }

        Ok(iterator)
    }
}

impl Drop for IoMasterPort {
    fn drop(&mut self) {
        let result = unsafe {
            mach_port::mach_port_deallocate(traps::mach_task_self(), self.0)
        };
        assert_eq!(result, kern_return::KERN_SUCCESS);
    }
}

#[derive(Debug)]
pub struct IoObject(sys::io_object_t);

impl IoObject {
    /// Returns typed dictionary with this object properties.
    /// In our case all keys are CFStrings, so there is no need to return
    /// untyped dict here.
    pub fn properties(&self) -> Result<CFDictionary<CFString, CFType>> {
        unsafe {
            let mut props: CFMutableDictionaryRef = mem::uninitialized();

            kern_try!(sys::IORegistryEntryCreateCFProperties(self.0, &mut props,
                kCFAllocatorDefault, 0));

            Ok(CFMutableDictionary::wrap_under_create_rule(props).to_immutable())
        }
    }
}

impl Drop for IoObject {
    fn drop(&mut self) {
        let result = unsafe {
            sys::IOObjectRelease(self.0)
        };
        assert_eq!(result, kern_return::KERN_SUCCESS);
    }
}

#[derive(Debug)]
pub struct IoIterator(sys::io_iterator_t);

impl Deref for IoIterator {
    type Target = sys::io_iterator_t;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IoIterator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Iterator for IoIterator {
    type Item = IoObject;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { sys::IOIteratorNext(self.0) } {
            0 => None,  // TODO: Should not there be some `NULL`?
            io_object => Some(IoObject(io_object))
        }
    }
}

impl Drop for IoIterator {
    fn drop(&mut self) {
        let result = unsafe {
            sys::IOObjectRelease(self.0)
        };
        assert_eq!(result, kern_return::KERN_SUCCESS);
    }
}

impl Default for IoIterator {
    // It is extremely unsafe and inner field MUST BE initialized
    // before the further `Drop::drop` call
    fn default() -> IoIterator {
        let inner = unsafe {
            mem::zeroed()
        };
        IoIterator(inner)
    }
}
