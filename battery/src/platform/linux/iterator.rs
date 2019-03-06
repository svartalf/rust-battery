use std::io;
use std::fs;
use std::iter;
use std::path::Path;

use crate::Battery;
use crate::platform::traits::BatteryIterator;
use super::SysFsDevice;
use super::sysfs;

#[derive(Debug)]
pub struct SysFsIterator {
    // TODO: It is not cool to store all results at once, should keep iterator instead
    entries: Vec<io::Result<fs::DirEntry>>,
}

impl SysFsIterator {
    pub fn from_path<T>(root: T) -> SysFsIterator where T: AsRef<Path> {
        let entries = match fs::read_dir(root.as_ref()) {
            Ok(entries) => entries.collect(),
            Err(_) => vec![],
        };

        SysFsIterator {
            entries,
        }
    }
}

impl iter::Iterator for SysFsIterator {
    type Item = Battery;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.entries.pop() {
                None => return None, // Nothing to iterate anymore
                Some(Err(_)) => continue, // Unable to access the sysfs somehow // TODO: trace!()
                Some(Ok(entry)) => {
                    let path = entry.path();
                    let type_ = fs::read_to_string(path.join("type"));
                    let scope = sysfs::scope(path.join("scope"));
                    match type_ {
                        Ok(ref content) if content == "Battery\n" && scope == sysfs::Scope::System => {
                            let inner = SysFsDevice::new(path);

                            return Some(Battery::from(inner));
                        },
                        _ => continue, // it is not a battery
                    }
                }
            }
        }
    }
}

impl BatteryIterator for SysFsIterator {}
