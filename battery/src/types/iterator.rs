use std::fmt;

use crate::platform::Iterator as PlatformIterator;
use crate::{Battery, Result};

/// An iterator that yields [batteries] available in system.
///
/// This struct is created by the [Manager::batteries](struct.Manager.html#method.batteries) method.
/// See its documentation for more.
///
/// [batteries]: struct.Battery.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Batteries(PlatformIterator);

impl Iterator for Batteries {
    type Item = Result<Battery>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(Ok(device)) => Some(Ok(device.into())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl From<PlatformIterator> for Batteries {
    fn from(inner: PlatformIterator) -> Batteries {
        Batteries(inner)
    }
}

impl fmt::Debug for Batteries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Batteries").field("impl", &self.0).finish()
    }
}
