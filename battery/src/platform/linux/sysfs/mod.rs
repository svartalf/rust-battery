use std::io;
use std::str::FromStr;

pub mod fs;
mod source;

pub use self::source::{DataBuilder, InstantData};

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Battery,
    Mains,
    Ups,
    Usb,
    Unknown,
    __Nonexhaustive,
}

impl FromStr for Type {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match () {
            _ if s.eq_ignore_ascii_case("Battery") => Type::Battery,
            _ if s.eq_ignore_ascii_case("Mains") => Type::Mains,
            _ if s.eq_ignore_ascii_case("Ups") => Type::Ups,
            _ if s.eq_ignore_ascii_case("Usb") => Type::Usb,
            _ => Type::Unknown,
        };
        Ok(value)
    }
}

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
    __Nonexhaustive,
}

impl FromStr for Scope {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            _ if s.eq_ignore_ascii_case("Device") => Scope::Device,
            _ if s.eq_ignore_ascii_case("System") => Scope::System,
            _ if s.eq_ignore_ascii_case("Unknown") => Scope::Unknown,
            _ => Scope::Unknown,
        };

        Ok(value)
    }
}
