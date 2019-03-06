use std::io;
use std::fs;
use std::str::FromStr;
use std::path::Path;

use crate::units::{Energy, Power, ElectricPotential, ElectricCharge};

/// A power supply which doesn't have a "scope" attribute should be assumed to
/// have "System" scope.
#[derive(Debug, Eq, PartialEq)]
pub enum Scope {
    /// Powers a specific device, or tree of devices
    Device,
    /// Powers the whole system
    System,
    /// Unknown power topology
    Unknown,
}

impl FromStr for Scope {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("Device") => Ok(Scope::Device),
            _ if s.eq_ignore_ascii_case("System") => Ok(Scope::System),
            _ if s.eq_ignore_ascii_case("Unknown") => Ok(Scope::Unknown),
            _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }
}

/// Read µWh value from the `energy_` file and convert into `Energy` type.
pub fn energy<T: AsRef<Path>>(path: T) -> Option<Energy> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("energy_"));

    match get_f32(path) {
        Ok(value_uwh) => Some(microwatt_hour!(value_uwh)),
        Err(_) => None,
    }
}

/// Read µAh value from the `charge_` file and convert into `ElectricCharge` type.
pub fn charge<T: AsRef<Path>>(path: T) -> Option<ElectricCharge> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("charge_"));

    match get_f32(path) {
        Ok(value_uah) => Some(microampere_hour!(value_uah)),
        Err(_) => None,
    }
}

/// Read µV value from the `voltage_` file and convert into `ElectricPotential` type.
pub fn voltage<T: AsRef<Path>>(path: T) -> Option<ElectricPotential> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("voltage_"));

    match get_f32(path) {
        Ok(value_uv) if value_uv > 1.0 => Some(microvolt!(value_uv)),
        _ => None,
    }
}

/// Read µW value from the `power_` file and convert into `Power` type.
pub fn power<T: AsRef<Path>>(path: T) -> Option<Power> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("power_"));

    match get_f32(path) {
        Ok(value_uw) if value_uw > 10_000.0 => Some(microwatt!(value_uw)),
        _ => None,
    }
}

pub fn scope<T: AsRef<Path>>(path: T) -> Scope {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy() == "scope");

    get_string(path)
        .and_then(|str| Scope::from_str(&str))
        .unwrap_or(Scope::System)
}

pub fn get_string<T: AsRef<Path>>(path: T) -> io::Result<String> {
    match fs::read_to_string(path) {
        Err(e) => Err(e),
        Ok(ref content) => {
            let trimmed = content.trim();
            if trimmed.starts_with('\0') {
                Err(io::Error::from(io::ErrorKind::InvalidData))
            } else {
                Ok(trimmed.to_string())
            }
        }
    }
}

// TODO: Generic somehow?

pub fn get_f32<T: AsRef<Path>>(path: T) -> io::Result<f32> {
    get_string(path).and_then(|value| {
        value.parse::<f32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })
}

pub fn get_u32<T: AsRef<Path>>(path: T) -> io::Result<u32> {
    get_string(path).and_then(|value| {
        value.parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })
}
