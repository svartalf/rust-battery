//! https://docs.microsoft.com/en-us/windows/desktop/power/battery-query-information-str

#![allow(non_snake_case)]

use std::mem;
use std::ops;
use std::default::Default;

use winapi::shared::{ntdef};

use super::info_level;

STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] #[derive(Debug)] struct BATTERY_QUERY_INFORMATION {
    BatteryTag: ntdef::ULONG,
    InformationLevel: info_level::BATTERY_QUERY_INFORMATION_LEVEL,
    AtRate: ntdef::LONG,
}}

impl Default for BATTERY_QUERY_INFORMATION {
    #[inline]
    fn default() -> Self {
        unsafe {
            mem::zeroed()
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatteryQueryInformation(BATTERY_QUERY_INFORMATION);

impl Default for BatteryQueryInformation {
    fn default() -> Self {
        BatteryQueryInformation(BATTERY_QUERY_INFORMATION::default())
    }
}

impl ops::Deref for BatteryQueryInformation {
    type Target = BATTERY_QUERY_INFORMATION;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for BatteryQueryInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
