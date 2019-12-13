use std::fmt;
use std::ops::Range;
use std::rc::Rc;

use super::{IoCtlDevice, IoCtlManager};
use crate::platform::traits::BatteryIterator;
use crate::Result;

pub struct IoCtlIterator {
    manager: Rc<IoCtlManager>,
    range: Range<libc::c_int>,
}

impl Iterator for IoCtlIterator {
    type Item = Result<IoCtlDevice>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.range.next() {
                None => return None,
                Some(idx) => {
                    let bif = self.manager.bif(idx);
                    let bst = self.manager.bst(idx);

                    match (bif, bst) {
                        (Ok(Some(bif)), Ok(Some(bst))) => {
                            return Some(Ok(IoCtlDevice::new(idx, bif, bst)));
                        }
                        (Err(e), _) => return Some(Err(e)),
                        (_, Err(e)) => return Some(Err(e)),
                        // If bif or bst is invalid (`Ok(None)` here),
                        // silently skipping it, same as FreeBSD does
                        _ => continue,
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some((self.range.end - self.range.start) as usize))
    }
}

impl BatteryIterator for IoCtlIterator {
    type Manager = IoCtlManager;
    type Device = IoCtlDevice;

    fn new(manager: Rc<Self::Manager>) -> Result<Self> {
        let batteries = manager.count()?;

        Ok(Self {
            manager,
            range: (0..batteries),
        })
    }
}

impl fmt::Debug for IoCtlIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FreeBSDIterator")
            .field("start", &self.range.start)
            .field("end", &self.range.end)
            .finish()
    }
}
