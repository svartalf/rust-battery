use std::io;
use std::str;
use std::fmt;

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
}

impl str::FromStr for State {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Case-insensitive
        // TODO: Support strings that starts with `\0`
        // TODO: Support `not charging` value
        // Ref: https://gitlab.freedesktop.org/upower/upower/blob/master/src/linux/up-device-supply.c#L452
        match s {
            "Unknown" => Ok(State::Unknown),
            "Empty" => Ok(State::Empty),
            "Full" => Ok(State::Full),
            "Charging" => Ok(State::Charging),
            "Discharging" => Ok(State::Discharging),
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
        };

        write!(f, "{}", display)
    }
}
