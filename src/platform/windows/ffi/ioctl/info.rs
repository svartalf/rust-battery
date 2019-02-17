//! https://docs.microsoft.com/en-us/windows/desktop/power/battery-information-str

#![allow(non_snake_case, clippy::unreadable_literal)]

use std::mem;
use std::ops;
use std::default::Default;
use std::str::{self, FromStr};

use winapi::shared::{ntdef};
use crate::Technology;

pub const BATTERY_CAPACITY_RELATIVE: ntdef::ULONG = 0x40000000;
pub const BATTERY_SYSTEM_BATTERY: ntdef::ULONG = 0x80000000;

STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] #[derive(Debug)] struct BATTERY_INFORMATION {
    Capabilities: ntdef::ULONG,
    Technology: ntdef::UCHAR,
    Reserved: [ntdef::UCHAR; 3],
    Chemistry: [ntdef::UCHAR; 4],
    DesignedCapacity: ntdef::ULONG, // mWh
    FullChargedCapacity: ntdef::ULONG, // mWh
    DefaultAlert1: ntdef::ULONG,
    DefaultAlert2: ntdef::ULONG,
    CriticalBias: ntdef::ULONG,
    CycleCount: ntdef::ULONG,
}}

impl Default for BATTERY_INFORMATION {
    #[inline]
    fn default() -> Self {
        unsafe {
            mem::zeroed()
        }
    }
}

#[derive(Debug)]
pub struct BatteryInformation(BATTERY_INFORMATION);

impl Default for BatteryInformation {
    fn default() -> Self {
        BatteryInformation(BATTERY_INFORMATION::default())
    }
}

impl ops::Deref for BatteryInformation {
    type Target = BATTERY_INFORMATION;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for BatteryInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BatteryInformation {
    pub fn is_system_battery(&self) -> bool {
        (self.0.Capabilities & BATTERY_SYSTEM_BATTERY) != 0
    }

    pub fn is_relative(&self) -> bool {
        (self.0.Capabilities & BATTERY_CAPACITY_RELATIVE) != 0
    }

    pub fn technology(&self) -> Technology {
        let raw = unsafe { str::from_utf8_unchecked(&self.0.Chemistry) };
        match Technology::from_str(raw) {
            Ok(tech) => tech,
            Err(_) => Technology::Unknown,
        }
    }

    // Originally `mWh`,matches `Battery::energy_full_design` result
    pub fn designed_capacity(&self) -> u32 {
        self.0.DesignedCapacity
    }

    // Originally `mWh`, matches `Battery::energy_full` result
    pub fn full_charged_capacity(&self) -> u32 {
        self.0.FullChargedCapacity
    }

    #[allow(dead_code)]
    pub fn cycle_count(&self) -> u32 {
        self.0.CycleCount
    }

}
