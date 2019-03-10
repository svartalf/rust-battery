use std::fmt;
use std::io;
use std::str;

/// Possible battery state values.
///
/// Unknown can mean either controller returned unknown,
/// or not able to retrieve state due to some error.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum State {
    Unknown,
    Charging,
    Discharging,
    Empty,
    Full,

    // Awaiting for https://github.com/rust-lang/rust/issues/44109
    #[doc(hidden)]
    __Nonexhaustive,
}

impl str::FromStr for State {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Support strings that starts with `\0`
        // TODO: Support `not charging` value
        // Ref: `up_device_supply_get_state` function at
        //https://gitlab.freedesktop.org/upower/upower/blob/master/src/linux/up-device-supply.c#L452
        match s {
            _ if s.eq_ignore_ascii_case("Unknown") => Ok(State::Unknown),
            _ if s.eq_ignore_ascii_case("Empty") => Ok(State::Empty),
            _ if s.eq_ignore_ascii_case("Full") => Ok(State::Full),
            _ if s.eq_ignore_ascii_case("Charging") => Ok(State::Charging),
            _ if s.eq_ignore_ascii_case("Discharging") => Ok(State::Discharging),
            _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            State::Unknown => "unknown",
            State::Charging => "charging",
            State::Discharging => "discharging",
            State::Empty => "empty",
            State::Full => "full",
            _ => "unknown",
        };

        write!(f, "{}", display)
    }
}

impl Default for State {
    fn default() -> Self {
        State::Unknown
    }
}
