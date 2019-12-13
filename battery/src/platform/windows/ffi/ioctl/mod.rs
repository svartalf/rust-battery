#![allow(clippy::unreadable_literal)]

// Each sub-module represents a C-level struct to respective IOCTL request
// and idiomatic Rust struct around it.

use winapi::shared::minwindef;

mod info;
mod query_info;
mod status;
mod wait_status;

pub use self::info::BatteryInformation;
pub use self::query_info::BatteryQueryInformation;
pub use self::status::BatteryStatus;
pub use self::wait_status::BatteryWaitStatus;

// Following values are based on the https://www.ioctls.net data
pub const IOCTL_BATTERY_QUERY_TAG: minwindef::DWORD = 0x294040;
pub const IOCTL_BATTERY_QUERY_INFORMATION: minwindef::DWORD = 0x294044;
pub const IOCTL_BATTERY_QUERY_STATUS: minwindef::DWORD = 0x29404c;

pub mod info_level {
    #![allow(non_camel_case_types, non_upper_case_globals)]

    /// For some reasons, "winapi==0.3.6" `ENUM!` macro fails to compile with
    /// error: no rules expected the token `@`
    /// so defining `BATTERY_QUERY_INFORMATION_LEVEL` "enum" manually.

    pub type BATTERY_QUERY_INFORMATION_LEVEL = u32;

    //    pub const BatteryInformation: BATTERY_QUERY_INFORMATION_LEVEL = 0;
    //    pub const BatteryGranularityInformation: BATTERY_QUERY_INFORMATION_LEVEL = 1;
    pub const BatteryTemperature: BATTERY_QUERY_INFORMATION_LEVEL = 2;
    //    pub const BatteryEstimatedTime: BATTERY_QUERY_INFORMATION_LEVEL = 3;
    pub const BatteryDeviceName: BATTERY_QUERY_INFORMATION_LEVEL = 4;
    //    pub const BatteryManufactureDate: BATTERY_QUERY_INFORMATION_LEVEL = 5;
    pub const BatteryManufactureName: BATTERY_QUERY_INFORMATION_LEVEL = 6;
    //    pub const BatteryUniqueID: BATTERY_QUERY_INFORMATION_LEVEL = 7;
    pub const BatterySerialNumber: BATTERY_QUERY_INFORMATION_LEVEL = 8;
}
