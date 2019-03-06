use std::default;
use std::io;
use std::iter;

use crate::platform::traits::BatteryManager;
use crate::platform::Iterator as InnerIterator;
use crate::platform::Manager as InnerManager;
use crate::Battery;

/// Manager for batteries available in system.
///
/// Knows how to fetch them and update information.
///
/// # Example
///
/// ```edition2018
/// # use battery::Manager;
/// # fn main() {
/// for battery in Manager::new().iter() {
///     println!("{:#?}", battery);
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct Manager(InnerManager);

impl Manager {
    /// Creates new manager instance.
    pub fn new() -> Manager {
        Manager(InnerManager::default())
    }

    /// Gets an iterator over available [batteries](struct.Battery.html).
    pub fn iter(&self) -> Batteries {
        Batteries(self.0.iter())
    }

    /// Refresh battery information in-place.
    pub fn refresh(&mut self, battery: &mut Battery) -> io::Result<()> {
        self.0.refresh(battery)
    }
}

impl default::Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator that yields batteries available in system.
///
/// This struct is created by the [iter](struct.Manager.html#method.iter) method on [Manager](struct.Manager.html).
/// See its documentation for more.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct Batteries(InnerIterator);

impl iter::Iterator for Batteries {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
