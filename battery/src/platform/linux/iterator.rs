use std::fmt;
use std::fs::{self, ReadDir};
use std::rc::Rc;

use super::{SysFsDevice, SysFsManager};
use crate::platform::traits::*;
use crate::Result;

pub struct SysFsIterator {
    #[allow(dead_code)]
    manager: Rc<SysFsManager>,
    entries: ReadDir,
}

impl BatteryIterator for SysFsIterator {
    type Manager = SysFsManager;
    type Device = SysFsDevice;

    fn new(manager: Rc<Self::Manager>) -> Result<Self> {
        let entries = fs::read_dir(manager.path())?;

        Ok(SysFsIterator {
            manager,
            entries,
        })
    }
}

impl Iterator for SysFsIterator {
    type Item = Result<<Self as BatteryIterator>::Device>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.entries.next() {
                None => None,
                // Unable to access sysfs for some reasons
                Some(Err(e)) => Some(Err(e.into())),
                Some(Ok(entry)) => {
                    let path = entry.path();
                    match SysFsDevice::is_system_battery(&path) {
                        Ok(true) => Some(SysFsDevice::try_from(path)),
                        Ok(false) => continue,
                        Err(e) => Some(Err(e)),
                    }
                }
            };
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.entries.size_hint()
    }
}

impl fmt::Debug for SysFsIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (start, end) = self.size_hint();
        f.debug_struct("LinuxIterator")
            .field("start", &start)
            .field("end", &end)
            .finish()
    }
}
