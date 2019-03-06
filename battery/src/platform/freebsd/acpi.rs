// https://github.com/freebsd/freebsd/blob/master/sys/dev/acpica/acpiio.h
// https://github.com/freebsd/freebsd/blob/master/sys/dev/acpica/acpi_battery.c

use std::io;
use std::mem;
use std::str::FromStr;
use std::default::Default;
use std::os::unix::io::{RawFd, IntoRawFd};
use std::ffi::CStr;

use nix::Error;

use crate::{State, Technology};

const ACPI_CMBAT_MAXSTRLEN: usize = 32;

// This one const is not defined in FreeBSD sources,
// but we are defining it for consistency.
const ACPI_BATT_STAT_FULL: u32 = 0x0000;
// Declared at `sys/dev/acpica/acpiio.h`
const ACPI_BATT_STAT_DISCHARG: u32 = 0x0001;
const ACPI_BATT_STAT_CHARGING: u32 = 0x0002;
const ACPI_BATT_STAT_CRITICAL: u32 = 0x0004;

/// FOr `AcpiBif` struct capacity is in mWh, rate in mW.
const ACPI_BIF_UNITS_MW: u32 = 0;
/// For `AcpiBif` struct capacity is in mAh, rate in mA.
const ACPI_BIF_UNITS_MA: u32 = 1;

fn map_nix_err(e: Error) -> io::Error {
    match e {
        Error::Sys(errno) => errno.into(),
        other => io::Error::new(io::ErrorKind::Other, other),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Units {
    MilliWatts,
    MilliAmperes,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiBif {
    units: u32,  // mW or mA
    dcap: u32,  // design capacity,
    lfcap: u32,  // last full capacity,
    btech: u32,  // battery technology,
    dvol: u32,  // design voltage (mV),
    wcap: u32,  // warn capacity,
    lcap: u32,  // low capacity,
    gra1: u32,  // granularity 1 (warn to low)
    gra2: u32,  // granularity 2 (full to warn)
    model: [u8; ACPI_CMBAT_MAXSTRLEN], // model identifier
    serial: [u8; ACPI_CMBAT_MAXSTRLEN],  // serial number
    type_: [u8; ACPI_CMBAT_MAXSTRLEN],  // type
    oeminfo: [u8; ACPI_CMBAT_MAXSTRLEN],  // OEM information
}

impl AcpiBif {
    pub fn units(&self) -> Units {
        match self.units {
            ACPI_BIF_UNITS_MW => Units::MilliWatts,
            ACPI_BIF_UNITS_MA => Units::MilliAmperes,
            _ => unreachable!("Unknown units from acpi_bif"),
        }
    }

    pub fn model(&self) -> Option<String> {
        self.get_string(&self.model)
    }

    pub fn serial(&self) -> Option<String> {
        self.get_string(&self.serial)
    }

    pub fn type_(&self) -> Option<String> {
        self.get_string(&self.type_)
    }

    pub fn technology(&self) -> Technology {
        match self.type_() {
            None => Technology::Unknown,
            Some(ref type_) => match Technology::from_str(type_) {
                Ok(tech) => tech,
                Err(_) => Technology::Unknown,
            }
        }
    }

    /// mV always
    #[inline]
    pub fn design_voltage(&self) -> u32 {
        self.dvol
    }

    pub fn oem(&self) -> Option<String> {
        self.get_string(&self.oeminfo)
    }

    /// Either mWh or mAh, depends on `self.units`
    #[inline]
    pub fn design_capacity(&self) -> u32 {
        self.dcap
    }

    /// Either mWh or mAh, depends on `self.units`
    #[inline]
    pub fn last_full_capacity(&self) -> u32 {
        self.lfcap
    }

    fn get_string(&self, bytes: &[u8]) -> Option<String> {
        let striped = match bytes.iter().position(|x| *x == 0x00) {
            Some(pos) => &bytes[..=pos],
            None => return None,
        };

        match CStr::from_bytes_with_nul(&striped) {
            Ok(cstr) => Some(cstr.to_string_lossy().to_string()),
            Err(_) => None,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiBst {
    state: u32,  // battery state
    rate: u32,  // present rate
    cap: u32,  // remaining capacity
    volt: u32,  // present voltage
}

impl AcpiBst {
    // based on `ACPI_BATT_STAT_*` defines
    #[inline]
    pub fn state(&self) -> State {
        match self.state {
            ACPI_BATT_STAT_FULL => State::Full,
            value if value & ACPI_BATT_STAT_DISCHARG != 0 => State::Discharging,
            value if value & ACPI_BATT_STAT_CHARGING != 0 => State::Charging,
            // This is probably a wrong state, because battery might be in critical state,
            // but charging at the moment. Might worth to investigate if it is possible
            // to implement `State::Critical` for all supported platforms.
            // In fact, right now this match arm is unreachable in most cases,
            // because previous arms will match first, but it would be harder to forget about it
            value if value & ACPI_BATT_STAT_CRITICAL != 0 => State::Discharging,
            _ => State::Unknown
        }
    }

    #[inline]
    pub fn rate(&self) -> u32 {
        self.rate
    }

    #[inline]
    pub fn capacity(&self) -> u32 {
        self.cap
    }

    #[inline]
    pub fn voltage(&self) -> u32 {
        self.volt
    }
}

#[repr(C)]
pub union AcpiBatteryIoctlArg {
    unit: i32,  // Device unit or ACPI_BATTERY_ALL_UNITS
    bif: AcpiBif,
    bst: AcpiBst,
}

impl Default for AcpiBatteryIoctlArg {
    fn default() -> Self {
        unsafe {
            mem::zeroed()
        }
    }
}

//ioctl_readwrite!(acpiio_batt_get_battinfo, b'B', 0x03, AcpiBatteryIoctlArg);
ioctl_readwrite!(acpiio_batt_get_bif, b'B', 0x10, AcpiBatteryIoctlArg);
ioctl_readwrite!(acpiio_batt_get_bst, b'B', 0x11, AcpiBatteryIoctlArg);

#[derive(Debug)]
pub struct AcpiDevice(RawFd);

impl AcpiDevice {
    pub fn new<T: IntoRawFd>(file: T) -> AcpiDevice {
        AcpiDevice(file.into_raw_fd())
    }

    pub fn bif(&self) -> io::Result<AcpiBif> {
        let mut arg = AcpiBatteryIoctlArg::default();
        unsafe {
            acpiio_batt_get_bif(self.0, &mut arg as *mut _).map_err(map_nix_err)?
        };
        let info = unsafe {
            arg.bif
        };

        Ok(info)
    }

    pub fn bst(&self) -> io::Result<AcpiBst> {
        let mut arg = AcpiBatteryIoctlArg::default();
        unsafe {
            acpiio_batt_get_bst(self.0, &mut arg as *mut _).map_err(map_nix_err)?
        };
        let info = unsafe {
            arg.bst
        };

        Ok(info)
    }
}
