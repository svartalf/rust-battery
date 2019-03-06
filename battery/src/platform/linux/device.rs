use std::io;
use std::f32;
use std::convert::AsRef;
use std::str::FromStr;
use std::path::PathBuf;
use std::default::Default;

use lazy_init::Lazy;
use num_traits::identities::Zero;

use crate::{State, Technology};
use crate::units::{Energy, Power, ElectricPotential, ElectricCharge, Ratio, ThermodynamicTemperature};
use crate::units::power::{watt, microwatt};
use crate::platform::traits::{BatteryDevice, Bound};
use super::sysfs;

#[derive(Default)]
pub struct Inner {
    root: PathBuf,

    design_voltage: Lazy<ElectricPotential>,
    energy: Lazy<Energy>,
    energy_full: Lazy<Energy>,
    energy_full_design: Lazy<Energy>,
    energy_rate: Lazy<Power>,
    voltage: Lazy<ElectricPotential>,
    state_of_charge: Lazy<Ratio>,

    temperature: Lazy<Option<ThermodynamicTemperature>>,
    cycle_count: Lazy<Option<u32>>,

    state: Lazy<State>,
    technology: Lazy<Technology>,
    manufacturer: Lazy<Option<String>>,
    model_name: Lazy<Option<String>>,
    serial_number: Lazy<Option<String>>,
}

impl Inner {
    pub fn new(root: PathBuf) -> Inner {
        let device = Inner {
            root,
            ..Default::default()
        };

        device.preload();

        device
    }
}

impl Inner {
    // With current design, `Inner` is not an instant representation of the battery stats
    // because of `Lazy` fields. End user might fetch needed data with a significant time difference
    // which will lead to an inconsistent results.
    // All results should be loaded at the same time; as for now, making a quick hack
    // and preloading all the stuff in once.
    // It seems that even with ignored results (`let _ = self...()`), rust still calls all required methods,
    // since we have side effects (file I/O)
    fn preload(&self) {
        let _ = self.design_voltage();
        let _ = self.energy();
        let _ = self.energy_full();
        let _ = self.energy_full_design();
        let _ = self.energy_rate();
        let _ = self.voltage();
        let _ = self.state_of_charge();
        let _ = self.temperature();
        let _ = self.state();
        let _ = self.technology();
        let _ = self.vendor();
        let _ = self.model();
        let _ = self.serial_number();
        let _ = self.cycle_count();
    }

    fn design_voltage(&self) -> ElectricPotential {
        *self.design_voltage.get_or_create(|| {
            ["voltage_max_design", "voltage_min_design", "voltage_present", "voltage_now"].iter()
                .filter_map(|filename| sysfs::voltage(self.root.join(filename)))
                .next()
                // Same to `upower`, using 10V as an approximation
                .unwrap_or_else(|| volt!(10.0))
        })
    }

    fn energy_now(&self) -> Option<Energy> {
        ["energy_now", "energy_avg"].iter()
            .filter_map(|filename| sysfs::energy(self.root.join(filename)))
            .next()
    }

    fn charge_now(&self) -> Option<ElectricCharge> {
        ["charge_now", "charge_avg"].iter()
            .filter_map(|filename| sysfs::charge(self.root.join(filename)))
            .next()
    }

    fn charge_full(&self) -> ElectricCharge {
        ["charge_full", "charge_full_design"].iter()
            .filter_map(|filename| sysfs::charge(self.root.join(filename)))
            .next()
            .unwrap_or_else(|| microampere_hour!(0.0))
    }

}

impl BatteryDevice for Inner {
    fn state_of_health(&self) -> Ratio {
        let energy_full = self.energy_full();
        if !energy_full.is_zero() {
            (energy_full / self.energy_full_design()).into_bounded()
        } else {
            percent!(100.0)
        }
    }

    fn energy(&self) -> Energy {
        *self.energy.get_or_create(|| {
            self.energy_now()
                .or_else(|| match self.charge_now() {
                    Some(charge) => Some(charge * self.design_voltage()),
                    None => None,
                })
                .unwrap_or_else(|| self.energy_full() * self.state_of_charge())
        })
    }

    fn energy_full(&self) -> Energy {
        *self.energy_full.get_or_create(|| {
            sysfs::energy(self.root.join("energy_full"))
                .or_else(|| {
                    match sysfs::charge(self.root.join("charge_full")) {
                        Some(charge) => Some(charge * self.design_voltage()),
                        None => None
                    }
                })
                .unwrap_or_else(|| self.energy_full_design())
        })
    }

    fn energy_full_design(&self) -> Energy {
        *self.energy_full_design.get_or_create(|| {
            sysfs::energy(self.root.join("energy_full_design"))
                .or_else(|| {
                    match sysfs::charge(self.root.join("charge_full_design")) {
                        Some(charge) => Some(charge * self.design_voltage()),
                        None => None
                    }
                })
                // Seems to be an impossible case
                .unwrap_or_else(|| microwatt_hour!(0.0))
        })
    }

    fn energy_rate(&self) -> Power {
        *self.energy_rate.get_or_create(|| {
            sysfs::power(self.root.join("power_now"))
                .or_else(|| {
                    match sysfs::get_f32(self.root.join("current_now")) {
                        Ok(current_now) => {
                            // If charge_full exists, then current_now is always reported in µA.
                            // In the legacy case, where energy only units exist, and power_now isn't present
                            // current_now is power in µW.
                            // Source: upower
                            if !self.charge_full().is_zero() {
                                // µA then
                                Some(microampere!(current_now) * self.design_voltage())
                            } else {
                                // µW :|
                                Some(microwatt!(current_now))
                            }
                        },
                        _ => None,
                    }
                })
                // Sanity check if power is greater than 100W (upower)
                .map(|power| {
                    if power.get::<watt>() > 100.0 {
                        watt!(0.0)
                    } else {
                        power
                    }
                })
                // Some batteries give out massive rate values when nearly empty (upower)
                .map(|power| {
                    if power.get::<microwatt>() < 10.0 {
                        watt!(0.0)
                    } else {
                        power
                    }
                })
                // ACPI gives out the special 'Ones' (Constant Ones Object) value for rate
                // when it's unable to calculate the true rate. We should set the rate zero,
                // and wait for the BIOS to stabilise.
                // Source: upower
                //
                // It come as an `0xffff` originally, but we are operating with `Power` now,
                // so this `Ones` value is recalculated a little.
                .map(|power| {
                    // TODO: There might be a chance that we had lost a precision during the conversion
                    // from the microwatts into default watts, so this should be fixed
                    if (power.get::<watt>() - 65535.0).abs() < f32::EPSILON {
                        watt!(0.0)
                    } else {
                        power
                    }
                })
                .unwrap_or_else(|| microwatt!(0.0))

                // TODO: Calculate energy_rate manually, if hardware fails.
                // if value < 0.01 {
                //    // Check upower `up_device_supply_calculate_rate` function
                // }
        })
    }

    fn state_of_charge(&self) -> Ratio {
        *self.state_of_charge.get_or_create(|| {
            match sysfs::get_f32(self.root.join("capacity")) {
                Ok(capacity) => percent!(capacity).into_bounded(),
                _ if self.energy_full().is_sign_positive() => self.energy() / self.energy_full(),
                Err(_) => percent!(0.0),
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

    fn voltage(&self) -> ElectricPotential {
        *self.voltage.get_or_create(|| {
            ["voltage_now", "voltage_avg"].iter()  // µV
                .filter_map(|filename| sysfs::voltage(self.root.join(filename)))
                .next()
                .unwrap_or_else(|| microvolt!(0.0))
        })
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        *self.temperature.get_or_create(|| {
            let res = sysfs::get_f32(self.root.join("temp"))
                .and_then(|temp| Ok(temp / 10.0));
            // TODO: Use .transmute() when it is stable
            match res {
                Ok(value) => Some(celsius!(value)),
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
        }).as_ref().map(AsRef::as_ref)
    }

    fn model(&self) -> Option<&str> {
        self.model_name.get_or_create(|| {
            match sysfs::get_string(self.root.join("model_name")) {
                Ok(model) => Some(model),
                Err(_) => None,
            }
        }).as_ref().map(AsRef::as_ref)
    }

    fn serial_number(&self) -> Option<&str> {
        self.serial_number.get_or_create(|| {
            match sysfs::get_string(self.root.join("serial_number")) {
                Ok(serial) => Some(serial),
                Err(_) => None,
            }
        }).as_ref().map(AsRef::as_ref)
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

#[derive(Default)]
pub struct SysFsDevice(Inner);

impl SysFsDevice {
    pub fn new(root: PathBuf) -> SysFsDevice {
        SysFsDevice(Inner::new(root))
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        self.0 = Inner::new(self.0.root.clone());

        Ok(())
    }
}

impl BatteryDevice for SysFsDevice {
    fn state_of_health(&self) -> Ratio {
        self.0.state_of_health()
    }

    fn energy(&self) -> Energy {
        self.0.energy()
    }

    fn energy_full(&self) -> Energy {
        self.0.energy_full()
    }

    fn energy_full_design(&self) -> Energy {
        self.0.energy_full_design()
    }

    fn energy_rate(&self) -> Power {
        self.0.energy_rate()
    }

    fn state_of_charge(&self) -> Ratio {
        self.0.state_of_charge()
    }

    fn state(&self) -> State {
        self.0.state()
    }

    fn voltage(&self) -> ElectricPotential {
        self.0.voltage()
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.0.temperature()
    }

    fn vendor(&self) -> Option<&str> {
        self.0.vendor()
    }

    fn model(&self) -> Option<&str> {
        self.0.model()
    }

    fn serial_number(&self) -> Option<&str> {
        self.0.serial_number()
    }

    fn technology(&self) -> Technology {
        self.0.technology()
    }

    fn cycle_count(&self) -> Option<u32> {
        self.0.cycle_count()
    }
}
