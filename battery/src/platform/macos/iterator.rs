use std::fmt;
use std::rc::Rc;

use crate::{Result};
use crate::platform::traits::{BatteryIterator};
use super::{iokit, IoKitDevice, IoKitManager};


pub struct IoKitIterator {
    #[allow(dead_code)]
    manager: Rc<IoKitManager>,
    inner: iokit::IoIterator,
}

impl Iterator for IoKitIterator {
    type Item = Result<IoKitDevice>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some(io_obj) => {
                match iokit::PowerSource::try_from(io_obj) {
                    Ok(source) => Some(Ok(source.into())),
                    Err(e) => Some(Err(e)),
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl BatteryIterator for IoKitIterator {
    type Manager = IoKitManager;
    type Device = IoKitDevice;

    fn new(manager: Rc<Self::Manager>) -> Result<Self> {
        let services = manager.get_services()?;

        Ok(Self{
            manager,
            inner: services,
        })
    }
}

impl fmt::Debug for IoKitIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (start, end) = self.size_hint();
        f.debug_struct("MacOSIterator")
            .field("start", &start)
            .field("end", &end)
            .finish()
    }
}
