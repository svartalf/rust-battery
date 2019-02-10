use std::str;
use std::fmt;

/// Battery type
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Technology {
    Unknown,
    LithiumIon,
    LeadAcid,
    LithiumPolymer,
    NickelMetalHydride,
    LithiumIronPhosphate,
}

impl str::FromStr for Technology {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tech = match s {
            _ if s.eq_ignore_ascii_case("li-ion") => Technology::LithiumIon,
            _ if s.eq_ignore_ascii_case("lion") => Technology::LithiumIon,
            _ if s.eq_ignore_ascii_case("pb") => Technology::LeadAcid,
            _ if s.eq_ignore_ascii_case("pbac") => Technology::LeadAcid,
            _ if s.eq_ignore_ascii_case("lip") => Technology::LithiumPolymer,
            _ if s.eq_ignore_ascii_case("lipo") => Technology::LithiumPolymer,
            _ if s.eq_ignore_ascii_case("li-poly") => Technology::LithiumPolymer,
            _ if s.eq_ignore_ascii_case("nimh") => Technology::NickelMetalHydride,
            _ if s.eq_ignore_ascii_case("life") => Technology::LithiumIronPhosphate,
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
            Technology::LithiumIronPhosphate => "lithium-iron-phosphate",
        };

        write!(f, "{}", display)
    }
}
