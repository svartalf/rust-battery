use std::iter;

use crate::Battery;
use super::device::IoCtlDevice;

#[derive(Debug)]
pub struct IoCtlIterator(pub Option<IoCtlDevice>);

impl iter::Iterator for IoCtlIterator {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(device) => Some(Battery::from(device))
        }
    }
}
