use std::i32;
use std::fmt;

use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::{CFString, CFStringGetTypeID};
use core_foundation::boolean::{CFBoolean, CFBooleanGetTypeID};
use core_foundation::number::{CFNumber, CFNumberGetTypeID};

use crate::units::{ElectricPotential, ElectricCurrent, ElectricCharge, ThermodynamicTemperature, Time};
use super::{IoObject, Result};
use super::super::traits::DataSource;

static FULLY_CHARGED_KEY: &'static str = "FullyCharged";
static EXTERNAL_CONNECTED_KEY: &'static str = "ExternalConnected";
static IS_CHARGING_KEY: &'static str = "IsCharging";
static VOLTAGE_KEY: &'static str = "Voltage";
static AMPERAGE_KEY: &'static str = "Amperage";
static DESIGN_CAPACITY_KEY: &'static str = "DesignCapacity";
static MAX_CAPACITY_KEY: &'static str = "MaxCapacity";
static CURRENT_CAPACITY_KEY: &'static str = "CurrentCapacity";
static TEMPERATURE_KEY: &'static str = "Temperature";
static CYCLE_COUNT_KEY: &'static str = "CycleCount";
static TIME_REMAINING_KEY: &'static str = "TimeRemaining";
static MANUFACTURER_KEY: &'static str = "Manufacturer";
static DEVICE_NAME_KEY: &'static str = "DeviceName";
static BATTERY_SERIAL_NUMBER_KEY: &'static str = "BatterySerialNumber";

pub struct PowerSource {
    object: IoObject,
    props: CFDictionary<CFString, CFType>,
}

impl PowerSource {
    pub fn get_bool(&self, raw_key: &'static str) -> Option<bool> {
        let key = CFString::from_static_string(raw_key);

        self.props.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFBooleanGetTypeID());
                }

                value_ref.downcast::<CFBoolean>()
            })
            .map(Into::into)
    }

    pub fn get_u32(&self, raw_key: &'static str) -> Option<u32> {
        let key = CFString::from_static_string(raw_key);

        self.props.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFNumberGetTypeID());
                }

                value_ref.downcast::<CFNumber>()
            })
            // TODO: We can lose data here actually,
            // but with currently used keys it seems to be impossible
            .and_then(|number| number.to_i32())
            .map(|value| value as u32)
    }

    pub fn get_i32(&self, raw_key: &'static str) -> Option<i32> {
        let key = CFString::from_static_string(raw_key);

        self.props.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFNumberGetTypeID());
                }

                value_ref.downcast::<CFNumber>()
            })
            .and_then(|number| number.to_i32())
    }

    pub fn get_string(&self, raw_key: &'static str) -> Option<String> {
        let key = CFString::from_static_string(raw_key);

        self.props.find(&key)
            .and_then(|value_ref| {
                unsafe {
                    debug_assert!(value_ref.type_of() == CFStringGetTypeID());
                }

                value_ref.downcast::<CFString>()
            })
            .map(|cf_string| cf_string.to_string())
    }
}

impl DataSource for PowerSource {
    fn refresh(&mut self) -> Result<()> {
        self.props = self.object.properties()?;

        Ok(())
    }

    fn fully_charged(&self) -> bool {
        self.get_bool(FULLY_CHARGED_KEY)
            .expect("IOKit is not providing required data")
    }

    fn external_connected(&self) -> bool {
        self.get_bool(EXTERNAL_CONNECTED_KEY)
            .expect("IOKit is not providing required data")
    }

    fn is_charging(&self) -> bool {
        self.get_bool(IS_CHARGING_KEY)
            .expect("IOKit is not providing required data")
    }

    // mV
    fn voltage(&self) -> ElectricPotential {
        let value = self.get_u32(VOLTAGE_KEY)
            .expect("IOKit is not providing required data");
        millivolt!(value)
    }

    // mA
    fn amperage(&self) -> ElectricCurrent {
        let value = self.get_i32(AMPERAGE_KEY)
            .expect("IOKit is not providing required data");
        milliampere!(value.abs())
    }

    // mAh
    fn design_capacity(&self) -> ElectricCharge {
        let value = self.get_u32(DESIGN_CAPACITY_KEY)
            .expect("IOKit is not providing required data");
        milliampere_hour!(value)
    }

    // mAh
    fn max_capacity(&self) -> ElectricCharge {
        let value = self.get_u32(MAX_CAPACITY_KEY)
            .expect("IOKit is not providing required data");
        milliampere_hour!(value)
    }

    // mAh
    fn current_capacity(&self) -> ElectricCharge {
        let value = self.get_u32(CURRENT_CAPACITY_KEY)
            .expect("IOKit is not providing required data");
        milliampere_hour!(value)
    }

    // milliCelsius :)
    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.get_i32(TEMPERATURE_KEY)
            .map(|value| celsius!(value as f32 / 100.0))
    }

    fn cycle_count(&self) -> Option<u32> {
        self.get_u32(CYCLE_COUNT_KEY)
    }

    fn time_remaining(&self) -> Option<Time> {
        self.get_i32(TIME_REMAINING_KEY)
            .and_then(|val| {
                if val == i32::MAX {
                    None
                } else {
                    Some(minute!(val))
                }
            })
    }

    fn manufacturer(&self) -> Option<String> {
        self.get_string(MANUFACTURER_KEY)
    }

    fn device_name(&self) -> Option<String> {
        self.get_string(DEVICE_NAME_KEY)
    }

    fn serial_number(&self) -> Option<String> {
        self.get_string(BATTERY_SERIAL_NUMBER_KEY)
    }
}

impl From<IoObject> for PowerSource {
    fn from(io_obj: IoObject) -> PowerSource {
        let props = io_obj.properties().expect("Unable to fetch properties for IOKit IOObject");

        PowerSource {
            object: io_obj,
            props,
        }
    }
}

impl fmt::Debug for PowerSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PowerSource")
            .field("io_object", &self.object)
            .field("properties", &self.props.as_CFType())
            .finish()
    }
}
