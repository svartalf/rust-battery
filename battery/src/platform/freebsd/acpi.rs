// https://github.com/freebsd/freebsd/blob/master/sys/dev/acpica/acpiio.h
// https://github.com/freebsd/freebsd/blob/master/sys/dev/acpica/acpi_battery.c

use std::default::Default;
use std::ffi::CStr;
use std::fs;
use std::mem;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::str::FromStr;

use crate::{Result, State, Technology};

const ACPI_CMBAT_MAXSTRLEN: usize = 32;

// This one const is not defined in FreeBSD sources,
// but we are defining it for consistency.
const ACPI_BATT_STAT_FULL: u32 = 0x0000;
// Following are declared at `sys/dev/acpica/acpiio.h`
const ACPI_BATT_STAT_DISCHARG: u32 = 0x0001;
const ACPI_BATT_STAT_CHARGING: u32 = 0x0002;
const ACPI_BATT_STAT_CRITICAL: u32 = 0x0004;
const ACPI_BATT_STAT_INVALID: u32 = ACPI_BATT_STAT_DISCHARG | ACPI_BATT_STAT_CHARGING;
const ACPI_BATT_STAT_BST_MASK: u32 = ACPI_BATT_STAT_INVALID | ACPI_BATT_STAT_CRITICAL;
const ACPI_BATT_STAT_NOT_PRESENT: u32 = ACPI_BATT_STAT_BST_MASK;

const ACPI_BATT_UNKNOWN: u32 = 0xffff_ffff;

/// For `AcpiBif` struct capacity is in mWh, rate in mW.
const ACPI_BIF_UNITS_MW: u32 = 0;
/// For `AcpiBif` struct capacity is in mAh, rate in mA.
const ACPI_BIF_UNITS_MA: u32 = 1;

#[derive(Debug, Eq, PartialEq)]
pub enum Units {
    MilliWatts,
    MilliAmperes,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiBif {
    units: u32,                          // mW or mA, see `ACPI_BIF_UNITS_*`
    dcap: u32,                           // design capacity,
    lfcap: u32,                          // last full capacity,
    btech: u32,                          // battery technology,
    dvol: u32,                           // design voltage (mV),
    wcap: u32,                           // warn capacity,
    lcap: u32,                           // low capacity,
    gra1: u32,                           // granularity 1 (warn to low)
    gra2: u32,                           // granularity 2 (full to warn)
    model: [u8; ACPI_CMBAT_MAXSTRLEN],   // model identifier
    serial: [u8; ACPI_CMBAT_MAXSTRLEN],  // serial number
    type_: [u8; ACPI_CMBAT_MAXSTRLEN],   // type
    oeminfo: [u8; ACPI_CMBAT_MAXSTRLEN], // OEM information
}

impl AcpiBif {
    // int acpi_battery_bif_valid(struct acpi_bif *bif)
    pub fn is_valid(&self) -> bool {
        self.lfcap != 0
    }

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
            },
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
    state: u32, // battery state
    rate: u32,  // present rate
    cap: u32,   // remaining capacity
    volt: u32,  // present voltage
}

impl AcpiBst {
    // int acpi_battery_bst_valid(struct acpi_bst *bst)
    pub fn is_valid(&self) -> bool {
        self.state != ACPI_BATT_STAT_NOT_PRESENT && self.cap != ACPI_BATT_UNKNOWN && self.volt != ACPI_BATT_UNKNOWN
    }

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
            _ => State::Unknown,
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
    unit: i32, // Device unit or ACPI_BATTERY_ALL_UNITS
    bif: AcpiBif,
    bst: AcpiBst,
}

impl Default for AcpiBatteryIoctlArg {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

ioctl_read!(acpiio_batt_get_units, b'B', 0x01, libc::c_int);
ioctl_readwrite!(acpiio_batt_get_bif, b'B', 0x10, AcpiBatteryIoctlArg);
ioctl_readwrite!(acpiio_batt_get_bst, b'B', 0x11, AcpiBatteryIoctlArg);

#[derive(Debug)]
pub struct AcpiDevice(RawFd);

impl AcpiDevice {
    pub fn new() -> Result<AcpiDevice> {
        let file = fs::OpenOptions::new().read(true).open("/dev/acpi")?;

        Ok(AcpiDevice(file.into_raw_fd()))
    }

    /// Count of the available batteries
    pub fn count(&self) -> Result<libc::c_int> {
        let mut arg = 0i32;
        unsafe { acpiio_batt_get_units(self.0, &mut arg as *mut _)? };

        Ok(arg)
    }

    /// # Returns
    ///
    /// * `Ok(Some(bif))` - successfully fetched bif
    /// * `Ok(None)` - bif was fetched but it is invalid; it is not an error, because we want to skip it silently
    /// * `Err(e)` - FFI call failed
    pub fn bif(&self, unit: libc::c_int) -> Result<Option<AcpiBif>> {
        let mut arg = AcpiBatteryIoctlArg::default();
        unsafe {
            arg.unit = unit;
            acpiio_batt_get_bif(self.0, &mut arg as *mut _)?
        };
        let info = unsafe { arg.bif };

        if info.is_valid() { Ok(Some(info)) } else { Ok(None) }
    }

    /// # Returns
    ///
    /// * `Ok(Some(bst))` - successfully fetched bst
    /// * `Ok(None)` - bst was fetched but it is invalid; it is not an error, because we want to skip it silently
    /// * `Err(e)` - FFI call failed
    pub fn bst(&self, unit: i32) -> Result<Option<AcpiBst>> {
        let mut arg = AcpiBatteryIoctlArg::default();
        unsafe {
            arg.unit = unit;
            acpiio_batt_get_bst(self.0, &mut arg as *mut _)?
        };
        let info = unsafe { arg.bst };

        if info.is_valid() { Ok(Some(info)) } else { Ok(None) }
    }
}

impl AsRawFd for AcpiDevice {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl Drop for AcpiDevice {
    fn drop(&mut self) {
        unsafe {
            fs::File::from_raw_fd(self.0);
        }
    }
}
