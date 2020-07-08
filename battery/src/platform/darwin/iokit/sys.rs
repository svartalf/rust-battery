#![allow(non_camel_case_types, dead_code, unused)]

// Functions here are copied from the `IOKit-sys` (https://crates.io/crates/iokit-sys) crate
// and rewritten to use `core_foundation` types.

use core_foundation::base::{mach_port_t, CFAllocatorRef};
use core_foundation::dictionary::{CFDictionaryRef, CFMutableDictionaryRef};
use libc::c_char;
use mach::{boolean, kern_return};

pub type io_object_t = mach_port_t;
pub type io_registry_entry_t = io_object_t;
pub type io_service_t = io_object_t;
pub type io_iterator_t = io_object_t;

pub type IOOptionBits = u32;

pub const IOPM_SERVICE_NAME: *const c_char = b"IOPMPowerSource\0".as_ptr() as *const c_char;

extern "C" {
    // https://developer.apple.com/documentation/iokit/kiomasterportdefault
    pub static kIOMasterPortDefault: mach_port_t;

    // https://developer.apple.com/documentation/iokit/1514652-iomasterport
    // Should be deallocated with `mach_port_deallocate(mach_task_self(), masterPort)`
    pub fn IOMasterPort(bootstrapPort: mach_port_t, masterPort: *mut mach_port_t) -> kern_return::kern_return_t;

    // https://developer.apple.com/documentation/iokit/1514687-ioservicematching
    // The dictionary is commonly passed to IOServiceGetMatchingServices or IOServiceAddNotification
    // which will consume a reference, otherwise it should be released with CFRelease by the caller.
    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;

    // https://developer.apple.com/documentation/iokit/1514494-ioservicegetmatchingservices?language=objc
    // An `existing` iterator handle is returned on success, and should be released by the caller
    // when the iteration is finished.
    pub fn IOServiceGetMatchingServices(
        masterPort: mach_port_t,
        matching: CFDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return::kern_return_t;

    // https://developer.apple.com/documentation/iokit/1514310-ioregistryentrycreatecfpropertie
    // The caller should release `properties` with CFRelease.
    pub fn IORegistryEntryCreateCFProperties(
        entry: io_registry_entry_t,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> kern_return::kern_return_t;

    // https://developer.apple.com/documentation/iokit/1514741-ioiteratornext
    // The element should be released by the caller when it is finished.
    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;

    pub fn IOIteratorIsValid(iterator: io_iterator_t) -> boolean::boolean_t;

    pub fn IOObjectRelease(object: io_object_t) -> kern_return::kern_return_t;
}
