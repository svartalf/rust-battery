use std::fmt;
use std::rc::Rc;

use crate::platform::traits::*;
use crate::platform::Iterator as PlatformIterator;
use crate::platform::Manager as PlatformManager;
use crate::{Batteries, Battery, Result};

/// Manager for batteries available in system.
///
/// Allows fetching and updating [batteries] information.
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
///
/// [batteries]: struct.Battery.html
pub struct Manager {
    inner: Rc<PlatformManager>,
}

impl Manager {
    /// Creates new manager value.
    pub fn new() -> Result<Manager> {
        let inner = PlatformManager::new()?;

        Ok(Manager {
            inner: Rc::new(inner),
        })
    }

    /// Returns an iterator over available batteries.
    ///
    /// There are no guarantees provided for [batteries] ordering,
    /// multiple calls to this method might result in any particular order
    /// depending on underline OS implementation.
    ///
    /// [batteries]: struct.Battery.html
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
