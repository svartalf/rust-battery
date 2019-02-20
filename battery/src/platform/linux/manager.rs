use std::io;
use std::default::Default;

use crate::{Battery};
use crate::platform::traits::{BatteryManager};

use super::SysFsIterator;

static SYSFS_ROOT: &'static str = "/sys/class/power_supply";

#[derive(Debug)]
pub struct SysFsManager;

impl SysFsManager {
    pub fn iter(&self) -> SysFsIterator {
        SysFsIterator::from_path(SYSFS_ROOT)
    }
}

impl BatteryManager for SysFsManager {
    fn refresh(&mut self, battery: &mut Battery) -> io::Result<()> {
        battery.get_mut_ref().refresh()
    }
}

impl Default for SysFsManager {
    fn default() -> SysFsManager {
        SysFsManager{}
    }
}
