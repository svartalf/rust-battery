use std::fmt;
use std::rc::Rc;

use crate::platform::traits::*;
use crate::platform::Iterator as PlatformIterator;
use crate::platform::Manager as PlatformManager;
use crate::{Batteries, Battery, Result};

/// Manager for batteries available in system.
///
/// Knows how to fetch them and update information.
///
/// # Example
///
/// ```edition2018
/// # use battery::{Result, Manager};
/// # fn main() -> Result<()> {
/// for battery in Manager::new()?.batteries()? {
///     println!("{:#?}", battery?);
/// }
/// # Ok(())
/// # }
/// ```
pub struct Manager {
    inner: Rc<PlatformManager>,
}

impl Manager {
    /// Creates new manager instance.
    pub fn new() -> Result<Manager> {
        let inner = PlatformManager::new()?;

        Ok(Manager {
            inner: Rc::new(inner),
        })
    }

    /// Gets an iterator over available [batteries](struct.Battery.html).
    pub fn batteries(&self) -> Result<Batteries> {
        let inner = PlatformIterator::new(self.inner.clone())?;

        Ok(Batteries::from(inner))
    }

    /// Refresh battery information in-place.
    pub fn refresh(&self, battery: &mut Battery) -> Result<()> {
        self.inner.refresh(battery)
    }
}

impl fmt::Debug for Manager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Manager").field("impl", &self.inner).finish()
    }
}
