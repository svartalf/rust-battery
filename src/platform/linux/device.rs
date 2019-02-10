use std::str::FromStr;
use std::path::PathBuf;
use std::default::Default;
use std::time::Duration;

use lazy_init::Lazy;

use crate::{Device, State, Technology};
use super::sysfs;

const DESIGN_VOLTAGE_PROBES: [&str; 4] = [
    "voltage_max_design",
    "voltage_min_design",
    "voltage_present",
    "voltage_now",
];

#[derive(Default)]
pub struct SysFsDevice {
    root: PathBuf,

    design_voltage: Lazy<f64>,
    energy: Lazy<f64>,
    energy_full: Lazy<f64>,
    energy_full_design: Lazy<f64>,
    energy_rate: Lazy<f64>,
    voltage: Lazy<f64>,
    percentage: Lazy<f64>,

    temperature: Lazy<f64>,

    state: Lazy<State>,
    technology: Lazy<Technology>,
    manufacturer: Lazy<Option<String>>,
    model_name: Lazy<Option<String>>,
    serial_number: Lazy<Option<String>>,

}

impl SysFsDevice {
    pub fn new(root: PathBuf) -> SysFsDevice {
        SysFsDevice {
            root,
            ..Default::default()
        }
    }
}

impl SysFsDevice {

    fn design_voltage(&self) -> f64 {
        *self.design_voltage.get_or_create(|| {
            DESIGN_VOLTAGE_PROBES.iter()
            .filter_map(|filename| {
                match sysfs::get_f64(self.root.join(filename)) {
                    Ok(value) if value > 1.0 => Some(value / 1_000_000.0),
                    _ => None,
                }
            })
            .next()
            // Same to `upower`, using 10V as an approximation
            .unwrap_or(10.0)
        })
    }

    fn charge_full(&self) -> f64 {
        ["charge_full", "charge_full_design"].iter()
            .filter_map(|filename| {
                match sysfs::get_f64(self.root.join(filename)) {
                    Ok(value) if value > 0.01 => Some(value),
                    _ => None,
                }
            })
            .next()
            .unwrap_or(0.0)
    }

}

impl Device for SysFsDevice {
    fn capacity(&self) -> f64 {
        let energy_full = self.energy_full();
        if energy_full > 0.0 {
            let capacity = (energy_full / self.energy_full_design()) * 100.0;
            set_bounds(capacity)
        } else {
            100.0
        }
    }

    fn energy(&self) -> f64 {
        *self.energy.get_or_create(|| {
            let mut value = ["energy_now", "energy_avg"].iter()
                .filter_map(|filename| {
                    match sysfs::get_f64(self.root.join(filename)) {
                        Ok(energy) => Some(energy / 1_000_000.0),
                        Err(_) => None,
                    }
                })
                .next()
                .unwrap_or(0.0);

            if value < 0.01 {
                value = ["charge_now", "charge_avg"].iter()
                    .filter_map(|filename| {
                        match sysfs::get_f64(self.root.join(filename)) {
                            Ok(charge) => Some(charge / 1_000_000.0 * self.design_voltage()),
                            Err(_) => None,
                        }
                    })
                    .next()
                    .unwrap_or(0.0);
            }

            if value < 0.1 {
                value = self.energy_full() * self.percentage() / 100.0;
            }

            value
        })
    }

    fn energy_full(&self) -> f64 {
        *self.energy_full.get_or_create(|| {
            let res = match sysfs::get_f64(self.root.join("energy_full")) {
                Ok(energy) => energy / 1_000_000.0,
                Err(_) => match sysfs::get_f64(self.root.join("charge_full")) {
                    Ok(charge) => charge / 1_000_000.0 * self.design_voltage(),
                    Err(_) => 0.0,
                }
            };

            if res < 0.01 {
                self.energy_full_design()
            } else {
                res
            }
        })

    }

    fn energy_full_design(&self) -> f64 {
        *self.energy_full_design.get_or_create(|| {
            match sysfs::get_f64(self.root.join("energy_full_design")) {
                Ok(energy) => energy / 1_000_000.0,
                Err(_) => match sysfs::get_f64(self.root.join("charge_full_design")) {
                    Ok(charge) => charge / 1_000_000.0 * self.design_voltage(),
                    Err(_) => 0.0,
                }
            }
        })
    }

    fn energy_rate(&self) -> f64 {
        *self.energy_rate.get_or_create(|| {
            let mut value = match sysfs::get_f64(self.root.join("power_now")) {
                // Same as `0.01` checks everywhere, but we do not need to divide first
                Ok(power) if power > 10_000.0 => {
                    (power / 1_000_000.0).abs()
                },
                _ => {
                    match sysfs::get_f64(self.root.join("current_now")) {
                        Ok(current_now) => {
                            // If charge_full exists, then current_now is always reported in uA.
                            // In the legacy case, where energy only units exist, and power_now isn't present
                            // current_now is power in uW.
                            // Source: upower
                            let mut current = (current_now / 1_000_000.0).abs();
                            if self.charge_full() != 0.0 {
                                current *= self.design_voltage();
                            }
                            current
                        },
                        Err(_) => {
                            0.0
                        },
                    }
                }
            };

            // ACPI gives out the special 'Ones' value for rate when it's unable
            // to calculate the true rate. We should set the rate zero, and wait
            // for the BIOS to stabilise.
            // Source: upower
            // TODO: Uncomment and fix
            // if value == 0xffff {
            //    value = 0.0;
            // }

            // Sanity check, same as upower does
            if value > 100.0 {
                value = 0.0;
            }

            // TODO: Calculate energy_rate manually, if hardware fails.
            // if value < 0.01 {
            //    // Check upower `up_device_supply_calculate_rate` function
            // }

            // Some batteries give out massive rate values when nearly empty
            if value < 0.01 {
                value = 0.0;
            }

            value
        })
    }

    // V
    fn voltage(&self) -> f64 {
        *self.voltage.get_or_create(|| {
            ["voltage_now", "voltage_avg"].iter()
                .filter_map(|filename| {
                    match sysfs::get_f64(self.root.join(filename)) {
                        Ok(voltage) if voltage > 0.01 => Some(voltage / 1_000_000.0),
                        _ => None,
                    }
                })
                .next()
                .unwrap_or(0.0) // TODO: Check if it is really unreachable
        })
    }

    // 0.0..100.0
    fn percentage(&self) -> f64 {
        *self.percentage.get_or_create(|| {
            match sysfs::get_f64(self.root.join("capacity")) {
                Ok(capacity) => set_bounds(capacity),
                _ if self.energy_full() > 0.0 => set_bounds(100.0 * self.energy() / self.energy_full()),
                Err(_) => 0.0,
            }
        })
    }

    fn state(&self) -> State {
        *self.state.get_or_create(|| {
            sysfs::get_string(self.root.join("status"))
                .and_then(|x| State::from_str(&x))
                .unwrap_or(State::Unknown)
        })
    }

    fn temperature(&self) -> f64 {
        *self.temperature.get_or_create(|| {
            sysfs::get_f64(self.root.join("temp"))
                .and_then(|temp| Ok(temp / 10.0))
                .unwrap_or(0.0)
        })
    }

    fn vendor(&self) -> Option<&str> {
        self.manufacturer.get_or_create(|| {
            match sysfs::get_string(self.root.join("manufacturer")) {
                Ok(vendor) => Some(vendor),
                Err(_) => None,
            }
        }).as_ref().map(|str| str.as_ref())
    }

    fn model(&self) -> Option<&str> {
        self.model_name.get_or_create(|| {
            match sysfs::get_string(self.root.join("model_name")) {
                Ok(model) => Some(model),
                Err(_) => None,
            }
        }).as_ref().map(|str| str.as_ref())
    }

    fn serial_number(&self) -> Option<&str> {
        self.serial_number.get_or_create(|| {
            match sysfs::get_string(self.root.join("serial_number")) {
                Ok(serial) => Some(serial),
                Err(_) => None,
            }
        }).as_ref().map(|str| str.as_ref())
    }

    fn technology(&self) -> Technology {
        *self.technology.get_or_create(|| {
            match sysfs::get_string(self.root.join("technology")) {
                Ok(ref tech) => Technology::from_str(tech).unwrap_or(Technology::Unknown),
                Err(_) => Technology::Unknown,
            }
        })
    }

    fn time_to_full(&self) -> Option<Duration> {
        match self.state() {
            State::Charging => {
                let time_to_full = 3600.0 * (self.energy_full() - self.energy()) / self.energy_rate();
                if time_to_full > (20.0 * 60.0 * 60.0) {
                    None
                } else {
                    Some(Duration::from_secs(time_to_full as u64))
                }
            },
            _ => None,
        }
    }

    fn time_to_empty(&self) -> Option<Duration> {
        match self.state() {
            State::Discharging => {
                let time_to_empty = 3600.0 * self.energy() / self.energy_rate();
                if time_to_empty > (240.0 * 60.0 * 60.0) { // Ten days for discharging
                    None
                } else {
                    Some(Duration::from_secs(time_to_empty as u64))
                }
            },
            _ => None,
        }
    }
}

#[inline]
fn set_bounds(value: f64) -> f64 {
    if value < 0.0 {
        return 0.0;
    }
    if value > 100.0 {
        return 100.0;
    }

    value
}
