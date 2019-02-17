use std::str;
use std::fmt;

/// Possible battery technologies.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
}

impl str::FromStr for Technology {
    type Err = ();

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
        };

        write!(f, "{}", display)
    }
}
