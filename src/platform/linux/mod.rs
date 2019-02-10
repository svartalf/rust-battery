use std::io;
use std::fs;
use std::iter;
use std::path::PathBuf;

use crate::{Battery};

mod device;
mod sysfs;

static SYSFS_ROOT: &'static str = "/sys/class/power_supply";

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct SysFs {
    adapters: Vec<PathBuf>,
}

impl SysFs {
    pub fn new() -> SysFs {
        let adapters = match SysFs::get_adapters() {
            Ok(list) => list,
            Err(_) => vec![],
        };

        SysFs {
            adapters,
        }
    }

    fn get_adapters() -> io::Result<Vec<PathBuf>> {
        let adapters = fs::read_dir(SYSFS_ROOT)?
            .filter_map(|entry| {
                match entry {
                    Ok(dir) => {
                        // TODO: Might worth to check if name starts with `BAT`, same as `psutil` does
                        let path = dir.path();
                        match fs::read_to_string(path.join("type")) {
                            Ok(ref content) if content == "Battery\n" => Some(path),
                            _ => None,
                        }
                    }
                    Err(_) => None
                }
            }).collect();

        Ok(adapters)
    }
}

impl iter::Iterator for SysFs {
    type Item = Battery<device::SysFsDevice>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.adapters.pop() {
            None => None,
            Some(path) => {
                let device = device::SysFsDevice::new(path);
                Some(Battery::new(device))
            }
        }
    }
}