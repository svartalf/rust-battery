//! https://docs.microsoft.com/en-us/windows/desktop/power/battery-status-str

#![allow(non_snake_case, clippy::unreadable_literal)]

use std::default::Default;
use std::mem;
use std::ops;

use winapi::shared::ntdef;

use crate::State;

/// Current battery capacity is unknown.
const BATTERY_UNKNOWN_CAPACITY: ntdef::ULONG = 0xFFFFFFFF;
/// Current battery voltage is unknown.
const BATTERY_UNKNOWN_VOLTAGE: ntdef::ULONG = 0xFFFFFFFF;
/// Current battery rage is unknown.
#[allow(overflowing_literals)]
const BATTERY_UNKNOWN_RATE: ntdef::LONG = 0x80000000;

/// Indicates that the battery is currently charging.
const BATTERY_CHARGING: ntdef::ULONG = 0x00000004;
/// Indicates that battery failure is imminent. See the Remarks section for more information.
const BATTERY_CRITICAL: ntdef::ULONG = 0x00000008;
/// Indicates that the battery is currently discharging.
const BATTERY_DISCHARGING: ntdef::ULONG = 0x00000002;
/// Indicates that the system has access to AC power, so no batteries are being discharged.
const BATTERY_POWER_ON_LINE: ntdef::ULONG = 0x00000001;

STRUCT! {#[cfg_attr(target_arch = "x86", repr(packed))] #[derive(Debug)] struct BATTERY_STATUS {
    PowerState: ntdef::ULONG,
    Capacity: ntdef::ULONG, // mWh or BATTERY_UNKNOWN_CAPACITY
    Voltage: ntdef::ULONG, // mV or BATTERY_UNKNOWN_VOLTAGE
    Rate: ntdef::LONG, // mW, might be negative
}}

impl Default for BATTERY_STATUS {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[derive(Debug)]
pub struct BatteryStatus(BATTERY_STATUS);

impl Default for BatteryStatus {
    fn default() -> Self {
        BatteryStatus(BATTERY_STATUS::default())
    }
}

impl ops::Deref for BatteryStatus {
    type Target = BATTERY_STATUS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for BatteryStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BatteryStatus {
    #[inline]
    pub fn is_charging(&self) -> bool {
        (self.0.PowerState & BATTERY_CHARGING) != 0
    }

    #[inline]
    pub fn is_critical(&self) -> bool {
        (self.0.PowerState & BATTERY_CRITICAL) != 0
    }

    #[inline]
    pub fn is_discharging(&self) -> bool {
        (self.0.PowerState & BATTERY_DISCHARGING) != 0
    }

    #[inline]
    pub fn is_power_on_line(&self) -> bool {
        (self.0.PowerState & BATTERY_POWER_ON_LINE) != 0
    }

    pub fn state(&self) -> State {
        match () {
            _ if self.is_charging() => State::Charging,
            _ if self.is_critical() => State::Empty,
            _ if self.is_discharging() => State::Discharging,
            _ if self.is_power_on_line() && !self.is_charging() => State::Full,
            _ => State::Unknown,
        }
    }

    pub fn voltage(&self) -> Option<u32> {
        if self.0.Voltage == BATTERY_UNKNOWN_VOLTAGE {
            None
        } else {
            Some(self.0.Voltage)
        }
    }

    pub fn capacity(&self) -> Option<u32> {
        if self.0.Capacity == BATTERY_UNKNOWN_CAPACITY {
            None
        } else {
            Some(self.0.Capacity)
        }
    }

    pub fn rate(&self) -> Option<i32> {
        if self.0.Rate == BATTERY_UNKNOWN_RATE {
            None
        } else {
            Some(self.0.Rate.abs())
        }
    }
}
