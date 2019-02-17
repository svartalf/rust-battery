//! https://docs.microsoft.com/en-us/windows/desktop/power/battery-status-str

#![allow(non_snake_case)]

use std::mem;
use std::ops;
use std::default::Default;

use winapi::shared::{ntdef};

STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] #[derive(Debug)] struct BATTERY_WAIT_STATUS {
    BatteryTag: ntdef::ULONG,
    Timeout: ntdef::ULONG,
    PowerState: ntdef::ULONG,
    LowCapacity: ntdef::ULONG,
    HighCapacity: ntdef::ULONG,
}}

impl Default for BATTERY_WAIT_STATUS {
    #[inline]
    fn default() -> Self {
        unsafe {
            mem::zeroed()
        }
    }
}

#[derive(Debug)]
pub struct BatteryWaitStatus(BATTERY_WAIT_STATUS);

impl Default for BatteryWaitStatus {
    fn default() -> Self {
        BatteryWaitStatus(BATTERY_WAIT_STATUS::default())
    }
}

impl ops::Deref for BatteryWaitStatus {
    type Target = BATTERY_WAIT_STATUS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for BatteryWaitStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
