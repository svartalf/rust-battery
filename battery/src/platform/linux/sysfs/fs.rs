use std::error;
use std::fs::read_to_string;
use std::io;
use std::path::Path;
use std::str::FromStr;

use super::{Scope, Type};
use crate::units::{ElectricCharge, ElectricPotential, Energy, Power};
use crate::Result;

// From the `errno.h`.
// Easier than building whole `libc` dep.
const ENODEV: i32 = 19;

/// Read µWh value from the `energy_` file and convert into `Energy` type.
pub fn energy<T: AsRef<Path>>(path: T) -> Result<Option<Energy>> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("energy_"));

    match get::<f32, _>(path) {
        Ok(Some(value_uwh)) => Ok(Some(microwatt_hour!(value_uwh))),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Read µAh value from the `charge_` file and convert into `ElectricCharge` type.
pub fn charge<T: AsRef<Path>>(path: T) -> Result<Option<ElectricCharge>> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("charge_"));

    match get::<f32, _>(path) {
        Ok(Some(value_uah)) if value_uah > 1.0 => Ok(Some(microampere_hour!(value_uah))),
        Ok(Some(_)) => Ok(None),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Read µV value from the `voltage_` file and convert into `ElectricPotential` type.
pub fn voltage<T: AsRef<Path>>(path: T) -> Result<Option<ElectricPotential>> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("voltage_"));

    match get::<f32, _>(path) {
        Ok(Some(value_uv)) if value_uv > 1.0 => Ok(Some(microvolt!(value_uv))),
        Ok(Some(_)) => Ok(None),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Read µW value from the `power_` file and convert into `Power` type.
pub fn power<T: AsRef<Path>>(path: T) -> Result<Option<Power>> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy().starts_with("power_"));

    match get::<f32, _>(path) {
        Ok(Some(value_uw)) if value_uw > 10_000.0 => Ok(Some(microwatt!(value_uw))),
        Ok(Some(_)) => Ok(None),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Read device `type` file and convert into `Type` enum.
pub fn type_<T: AsRef<Path>>(path: T) -> Result<Type> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy() == "type");

    match get::<Type, _>(path) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Ok(Type::Unknown),
        Err(e) => Err(e),
    }
}

/// Read device `scope` file and convert into `Scope` enum.
pub fn scope<T: AsRef<Path>>(path: T) -> Result<Scope> {
    let path = path.as_ref();
    debug_assert!(path.file_name().unwrap().to_string_lossy() == "scope");

    match get::<Scope, _>(path) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Ok(Scope::System),
        Err(e) => Err(e),
    }
}

/// ## Returns
///
/// Ok(Some(value)) - file was read properly
/// Ok(None) - file is missing
/// Err(_) - unable to access file for some reasons (except `NotFound` and `ENODEV`)
pub fn get_string<T: AsRef<Path>>(path: T) -> Result<Option<String>> {
    match read_to_string(path) {
        Ok(mut content) => {
            if content.starts_with('\0') {
                Err(io::Error::from(io::ErrorKind::InvalidData).into())
            } else {
                // In-place trim
                if content.ends_with('\n') {
                    content.truncate(content.len() - 1);
                }

                Ok(Some(content))
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        // Some drivers are creating the files, but attempt to read them
        // fails with a `ENODEV` error.
        // See https://github.com/svartalf/rust-battery/issues/28
        Err(ref e) if e.raw_os_error() == Some(ENODEV) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn get<V, T>(path: T) -> Result<Option<V>>
where
    T: AsRef<Path>,
    V: FromStr,
    <V as FromStr>::Err: error::Error + Sync + Send,
{
    match get_string(path) {
        Ok(Some(ref value)) => match V::from_str(value) {
            Ok(result) => Ok(Some(result)),
            Err(_) => Ok(None),
        },
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}
