use std::fmt;
use std::str;

#[cfg(feature = "co3")]
use co3::{rust_spec::RustSpec, ReprC};

use crate::Error;

/// Possible battery technologies.
#[cfg_attr(feature = "co3", derive(RustSpec, ReprC))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Technology {
    Unknown,
    LithiumIon,
    LeadAcid,
    LithiumPolymer,
    NickelMetalHydride,
    NickelCadmium,
    NickelZinc,
    LithiumIronPhosphate,
    RechargeableAlkalineManganese,

    // Awaiting for https://github.com/rust-lang/rust/issues/44109
    #[doc(hidden)]
    __Nonexhaustive = 255,
}

impl str::FromStr for Technology {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tech = match s {
            _ if s.eq_ignore_ascii_case("li-i") => Technology::LithiumIon,
            _ if s.eq_ignore_ascii_case("li-ion") => Technology::LithiumIon,
            _ if s.eq_ignore_ascii_case("lion") => Technology::LithiumIon,
            _ if s.eq_ignore_ascii_case("pb") => Technology::LeadAcid,
            _ if s.eq_ignore_ascii_case("pbac") => Technology::LeadAcid,
            _ if s.eq_ignore_ascii_case("lip") => Technology::LithiumPolymer,
            _ if s.eq_ignore_ascii_case("lipo") => Technology::LithiumPolymer,
            _ if s.eq_ignore_ascii_case("li-poly") => Technology::LithiumPolymer,
            _ if s.eq_ignore_ascii_case("nimh") => Technology::NickelMetalHydride,
            _ if s.eq_ignore_ascii_case("nicd") => Technology::NickelCadmium,
            _ if s.eq_ignore_ascii_case("nizn") => Technology::NickelZinc,
            _ if s.eq_ignore_ascii_case("life") => Technology::LithiumIronPhosphate,
            _ if s.eq_ignore_ascii_case("ram") => Technology::RechargeableAlkalineManganese,
            // TODO: warn!
            _ => Technology::Unknown,
        };

        Ok(tech)
    }
}

impl fmt::Display for Technology {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            Technology::Unknown => "unknown",
            Technology::LithiumIon => "lithium-ion",
            Technology::LeadAcid => "lead-acid",
            Technology::LithiumPolymer => "lithium-polymer",
            Technology::NickelMetalHydride => "nickel-metal-hydride",
            Technology::NickelCadmium => "nickel-cadmium",
            Technology::NickelZinc => "nickel-zinc",
            Technology::LithiumIronPhosphate => "lithium-iron-phosphate",
            Technology::RechargeableAlkalineManganese => "rechargeable-alkaline-manganese",
            _ => "unknown",
        };

        write!(f, "{}", display)
    }
}

impl Default for Technology {
    fn default() -> Self {
        Technology::Unknown
    }
}
