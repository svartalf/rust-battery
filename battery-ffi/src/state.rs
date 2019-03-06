use battery::State as RawState;

/// Possible battery states.
///
/// Enum members are prefixed here in order to not have "redeclaration of enumerator" error in C.
#[repr(u8)]
pub enum State {
    // DO NOT RE-ORDER VALUES IN THIS ENUM, IT WILL AFFECT FFI USERS!
    StateUnknown = 0,
    StateCharging = 1,
    StateDischarging = 2,
    StateEmpty = 3,
    StateFull = 4,
}

impl From<RawState> for State {
    fn from(s: RawState) -> Self {
        match s {
            RawState::Unknown => State::StateUnknown,
            RawState::Charging => State::StateCharging,
            RawState::Discharging => State::StateDischarging,
            RawState::Empty => State::StateEmpty,
            RawState::Full => State::StateFull,
            _ => State::StateUnknown,
        }
    }
}
