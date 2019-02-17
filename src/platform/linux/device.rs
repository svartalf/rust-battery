use std::str::FromStr;
use std::path::PathBuf;
use std::default::Default;

use lazy_init::Lazy;

use crate::{State, Technology};
use crate::types::Device;
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

    design_voltage: Lazy<u32>,
    energy: Lazy<u32>,
    energy_full: Lazy<u32>,
    energy_full_design: Lazy<u32>,
    energy_rate: Lazy<u32>,
    voltage: Lazy<u32>,  // mV
    percentage: Lazy<f32>, // 0.0 .. 100.0

    temperature: Lazy<Option<f32>>,
    cycle_count: Lazy<Option<u32>>,

    state: Lazy<State>,
    technology: Lazy<Technology>,
    manufacturer: Lazy<Option<String>>,
    model_name: Lazy<Option<String>>,
    serial_number: Lazy<Option<String>>,
}

impl SysFsDevice {
    pub fn new(root: PathBuf) -> SysFsDevice {
        let device = SysFsDevice {
            root,
            ..Default::default()
        };

        device.preload();

        device
    }
}

impl SysFsDevice {
    // With current design, `SysFsDevice` is not an instant representation of the battery stats
    // because of `Lazy` fields. End user might fetch needed data with a significant time difference
    // which will lead to an inconsistent results.
    // All results should be loaded at the same time; as for now, making a quick hack
    // and preloading all the stuff in once.
    // It seems that even with ignored results (`let _ = self...()`), rust still calls all required methods.
    fn preload(&self) {
        let _ = self.design_voltage();
        let _ = self.energy();
        let _ = self.energy_full();
        let _ = self.energy_full_design();
        let _ = self.energy_rate();
        let _ = self.voltage();
        let _ = self.percentage();
        let _ = self.temperature();
        let _ = self.state();
        let _ = self.technology();
        let _ = self.vendor();
        let _ = self.model();
        let _ = self.serial_number();
        let _ = self.cycle_count();
    }

    fn design_voltage(&self) -> u32 {
        *self.design_voltage.get_or_create(|| {
            DESIGN_VOLTAGE_PROBES.iter()
            .filter_map(|filename| {
                match sysfs::get_u32(self.root.join(filename)) {
                    Ok(value) if value > 1 => Some(value / 1_000_000),
                    _ => None,
                }
            })
            .next()
            // Same to `upower`, using 10V as an approximation
            .unwrap_or(10)
        })
    }

    fn charge_full(&self) -> u32 {
        ["charge_full", "charge_full_design"].iter() // µAh
            .filter_map(|filename| {
                match sysfs::get_u32(self.root.join(filename)) {
                    Ok(value) => Some(value / 1_000),
                    _ => None,
                }
            })
            .next()
            .unwrap_or(0)
    }

}

impl Device for SysFsDevice {
    fn capacity(&self) -> f32 {
        let energy_full = self.energy_full();
        if energy_full > 0 {
            let capacity = (energy_full as f32 / self.energy_full_design() as f32) * 100.0;
            set_bounds(capacity)
        } else {
            100.0
        }
    }

    fn energy(&self) -> u32 {
        *self.energy.get_or_create(|| {
            let mut value = ["energy_now", "energy_avg"].iter()
                .filter_map(|filename| {
                    match sysfs::get_u32(self.root.join(filename)) {
                        Ok(energy) => Some(energy / 1_000),
                        Err(_) => None,
                    }
                })
                .next();

            if value.is_none() {
                value = ["charge_now", "charge_avg"].iter()
                    .filter_map(|filename| {
                        match sysfs::get_u32(self.root.join(filename)) {
                            Ok(charge) => Some(charge / 1_000 * self.design_voltage()),
                            Err(_) => None,
                        }
                    })
                    .next();
            }

            match value {
                None => self.energy_full() * self.percentage() as u32 / 100,
                Some(energy) => energy,
            }
        })
    }

    fn energy_full(&self) -> u32 {
        *self.energy_full.get_or_create(|| {
            let res = match sysfs::get_u32(self.root.join("energy_full")) {
                Ok(energy) => energy / 1_000,
                Err(_) => match sysfs::get_u32(self.root.join("charge_full")) {
                    Ok(charge) => charge / 1_000 * self.design_voltage(),
                    Err(_) => 0,
                }
            };

            if res == 0 {
                self.energy_full_design()
            } else {
                res
            }
        })

    }

    fn energy_full_design(&self) -> u32 {
        *self.energy_full_design.get_or_create(|| {
            match sysfs::get_u32(self.root.join("energy_full_design")) {
                Ok(energy) => energy / 1_000,
                Err(_) => match sysfs::get_u32(self.root.join("charge_full_design")) {
                    Ok(charge) => charge / 1_000 * self.design_voltage(),
                    Err(_) => 0,
                }
            }
        })
    }

    fn energy_rate(&self) -> u32 {
        *self.energy_rate.get_or_create(|| {
            let mut value = match sysfs::get_u32(self.root.join("power_now")) {
                Ok(power) if power > 10_000 => power / 1_000,
                _ => {
                    match sysfs::get_u32(self.root.join("current_now")) {
                        Ok(current_now) => {
                            // If charge_full exists, then current_now is always reported in uA.
                            // In the legacy case, where energy only units exist, and power_now isn't present
                            // current_now is power in uW.
                            // Source: upower
                            let mut current = current_now / 1_000;
                            if self.charge_full() != 0 {
                                current *= self.design_voltage();
                            }
                            current
                        },
                        Err(_) => {
                            0u32
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

            // Sanity check, same as upower does, if power is greater than 100W
            if value > 100_000 {
                value = 0;
            }

            // TODO: Calculate energy_rate manually, if hardware fails.
            // if value < 0.01 {
            //    // Check upower `up_device_supply_calculate_rate` function
            // }

            // Some batteries give out massive rate values when nearly empty
             if value < 10_000 {
                 value = 0;
             }

            value
        })
    }

    // mV
    fn voltage(&self) -> u32 {
        *self.voltage.get_or_create(|| {
            ["voltage_now", "voltage_avg"].iter()
                .filter_map(|filename| {
                    match sysfs::get_u32(self.root.join(filename)) {
                        Ok(voltage) if voltage > 1 => Some(voltage / 1_000),
                        _ => None,
                    }
                })
                .next()
                .unwrap_or(0) // TODO: Check if it is really unreachable
        })
    }

    // 0.0..100.0
    fn percentage(&self) -> f32 {
        *self.percentage.get_or_create(|| {
            let capacity= match sysfs::get_u32(self.root.join("capacity")) {
                Ok(capacity) => capacity,
                _ if self.energy_full() > 0 => 100 * self.energy() / self.energy_full(),
                Err(_) => 0,
            };

            set_bounds(capacity as f32)
        })
    }

    fn state(&self) -> State {
        *self.state.get_or_create(|| {
            sysfs::get_string(self.root.join("status"))
                .and_then(|x| State::from_str(&x))
                .unwrap_or(State::Unknown)
        })
    }

    fn temperature(&self) -> Option<f32> {
        *self.temperature.get_or_create(|| {
            let res = sysfs::get_f32(self.root.join("temp"))
                .and_then(|temp| Ok(temp / 10.0));
            match res {
                Ok(value) => Some(value),
                Err(_) => None,
            }
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

    fn cycle_count(&self) -> Option<u32> {
        *self.cycle_count.get_or_create(|| {
            match sysfs::get_u32(self.root.join("cycle_count")) {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        })
    }

}

#[inline]
fn set_bounds(value: f32) -> f32 {
    if value < 0.0 {
        return 0.0;
    }
    if value > 100.0 {
        return 100.0;
    }

    value
}
