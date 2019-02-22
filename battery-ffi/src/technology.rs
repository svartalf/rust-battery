use battery::Technology as RawTech;

/// Possible battery technologies.
///
/// New members might be added to this enum in the next versions,
/// so users are required to properly handle that case.
///
/// Enum members are prefixed here in order to not have "redeclaration of enumerator" error in C.
#[repr(u8)]
pub enum Technology {
    TechnologyUnknown = 0,
    TechnologyLithiumIon = 1,
    TechnologyLeadAcid = 2,
    TechnologyLithiumPolymer = 3,
    TechnologyNickelMetalHydride = 4,
    TechnologyNickelCadmium = 5,
    TechnologyNickelZinc = 6,
    TechnologyLithiumIronPhosphate = 7,
    TechnologyRechargeableAlkalineManganese = 8,
}

impl From<RawTech> for Technology {
    fn from(s: RawTech) -> Self {
        match s {
            RawTech::Unknown => Technology::TechnologyUnknown,
            RawTech::LithiumIon => Technology::TechnologyLithiumIon,
            RawTech::LeadAcid => Technology::TechnologyLeadAcid,
            RawTech::LithiumPolymer => Technology::TechnologyLithiumPolymer,
            RawTech::NickelMetalHydride => Technology::TechnologyNickelMetalHydride,
            RawTech::NickelCadmium => Technology::TechnologyNickelCadmium,
            RawTech::NickelZinc => Technology::TechnologyNickelZinc,
            RawTech::LithiumIronPhosphate => Technology::TechnologyLithiumIronPhosphate,
            RawTech::RechargeableAlkalineManganese => Technology::TechnologyRechargeableAlkalineManganese,
        }
    }
}
