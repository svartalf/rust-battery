#[cfg(feature = "export")]
use crate::OwnedBattery;
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

impl Batteries {
    fn next_result(&mut self) -> Option<Result<Battery>> {
        self.0.next().map(|result| result.map(Battery::from))
    }
}

impl Iterator for Batteries {
    #[cfg(not(feature = "co3"))]
    type Item = Result<Battery>;
    #[cfg(feature = "export")]
    type Item = Option<OwnedBattery>;

    #[cfg(not(feature = "co3"))]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_result()
    }

    #[cfg(feature = "export")]
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_result() {
            Some(result) => Some(crate::errors::into_ffi_option(result.map(Box::new))),
            None => {
                crate::errors::clear_ffi_error();
                None
            }
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

impl core::fmt::Debug for Batteries {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Batteries").field("impl", &self.0).finish()
    }
}
