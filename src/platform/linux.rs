use std::io;
use std::fs;
use std::iter;
use std::str::FromStr;
use std::path::{Path, PathBuf};

use crate::{Battery, State};

static SYSFS_ROOT: &'static str = "/sys/class/power_supply";

fn read<T: AsRef<Path>>(path: T) -> io::Result<f64> {
    match fs::read_to_string(path.as_ref()) {
        Ok(ref content) => content.trim().parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        Err(e) => Err(e)
    }
}

// Since sysfs `power_supply` files structure is not fixed, we are trying to
// read as much different files as posslbe
fn multi_read<T: AsRef<Path>, P: AsRef<Path>>(root: T, probes: &[P]) -> io::Result<f64> {
    let root = root.as_ref();
    for probe in probes {
        let path = root.join(probe.as_ref());
        match read(path) {
            Ok(result) => return Ok(result),
            Err(_e) => continue,
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}


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

    fn parse<T: AsRef<Path>>(&self, root: T) -> io::Result<Battery> {
        let root = root.as_ref();

        let current = multi_read(root, &["energy_now", "charge_now"])?;
        let voltage = multi_read(root, &["voltage_now"])?;
        let full = multi_read(root, &["energy_full", "charge_full"])?;
        let design_voltage = multi_read(root, &["voltage_max_design", "voltage_min_design"])
            .unwrap_or(voltage);

        let voltage = voltage / 1_000.0 / 1_000.0;
        let design_voltage = design_voltage / 1_000.0 / 1_000.0;

        let design = match read(root.join("charge_full_design")) {
            Ok(value) => value * design_voltage, // Converting to amps
            Err(_e) => read(root.join("energy_full_design"))?
        };

        let charge_rate = match read(root.join("current_now")) {
            Ok(value) => value * voltage, // Converting to amps
            Err(_e) => read(root.join("power_now"))?
        };

        let state = match fs::read_to_string(root.join("status")) {
            Ok(ref content) => match State::from_str(content.trim()) {
                Ok(state) => state,
                Err(_) => State::Unknown
            },
            Err(_) => State::Unknown
        };

        Ok(Battery {
            state,
            current,
            full,
            design,
            charge_rate,
            voltage,
            design_voltage,
        })
    }

}

impl iter::Iterator for SysFs {
    type Item = io::Result<Battery>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.adapters.pop() {
            None => None,
            Some(ref path) => {
                match self.parse(path) {
                    Ok(battery) => Some(Ok(battery)),
                    Err(e) => Some(Err(e)),
                }
            }
        }
    }
}
