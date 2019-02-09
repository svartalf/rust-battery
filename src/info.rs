use std::io;
use std::str;
use std::fmt;

/// Possible battery state values.
///
/// Unknown can mean either controller returned unknown,
/// or not able to retrieve state due to some error.
#[derive(Debug)]
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

/// Battery information representation
#[derive(Debug)]
pub struct Battery {
    pub(crate) state: State,
	pub(crate) current: f64,
    pub(crate) full: f64,
	pub(crate) design: f64,
	pub(crate) charge_rate: f64,
	pub(crate) voltage: f64,
	pub(crate) design_voltage: f64,
}

impl Battery {
    /// Current battery state
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Current (momentary) capacity (in `mWh`).
    pub fn current(&self) -> &f64 {
        &self.current
    }

	/// Last known full capacity (in `mWh`).
    pub fn full(&self) -> &f64 {
        &self.full
    }

	/// Reported design capacity (in `mWh`).
    pub fn design(&self) -> &f64 {
        &self.design
    }

	/// Current (momentary) charge rate (in `mW`).
	/// It is always non-negative, consult `state()` method to check
	/// whether it means charging or discharging.
	pub fn charge_rate(&self) -> &f64 {
        &self.charge_rate
    }

    /// Design voltage (in `V`).
	/// Some systems (e.g. macOS) do not provide a separate
	/// value for this. In such cases, or if getting this fails,
	/// but getting `voltage()` succeeds, this field will have
	/// the same value as `voltage()`, for convenience.
    pub fn design_voltage(&self) -> &f64 {
        &self.design_voltage
    }

    /// Current voltage (in V).
    pub fn voltage(&self) -> &f64 {
        &self.voltage
    }

}