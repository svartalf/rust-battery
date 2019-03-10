use crate::Result;
use crate::platform::traits::BatteryDevice;
use crate::units::{ElectricPotential, ElectricCurrent, ElectricCharge, ThermodynamicTemperature, Time};
use crate::units::energy::watt_hour;
use crate::units::power::milliwatt;
use super::device::IoKitDevice;
use super::traits::DataSource;

/// This data source is not using uom types, because it is easier to create test suites
/// from the `ioreg` tool output that way (which values are in mV, mA, mAh and mWh).
#[derive(Debug, Default)]
struct TestDataSource {
    fully_charged: bool,
    external_connected: bool,
    is_charging: bool,
    voltage: u32,
    amperage: i32,
    design_capacity: u32,
    max_capacity: u32,
    current_capacity: u32,
    temperature: Option<f32>,
    cycle_count: Option<u32>,
}

impl DataSource for TestDataSource {
    fn refresh(&mut self) -> Result<()> {
        Ok(())
    }

    fn fully_charged(&self) -> bool {
        self.fully_charged
    }

    fn external_connected(&self) -> bool {
        self.external_connected
    }

    fn is_charging(&self) -> bool {
        self.is_charging
    }

    fn voltage(&self) -> ElectricPotential {
        millivolt!(self.voltage)
    }

    fn amperage(&self) -> ElectricCurrent {
        milliampere!(self.amperage.abs())
    }

    fn design_capacity(&self) -> ElectricCharge {
        milliampere_hour!(self.design_capacity)
    }

    fn max_capacity(&self) -> ElectricCharge {
        milliampere_hour!(self.max_capacity)
    }

    fn current_capacity(&self) -> ElectricCharge {
        milliampere_hour!(self.current_capacity)
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.temperature.map(|value| {
            celsius!(value)
        })
    }

    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count
    }

    fn time_remaining(&self) -> Option<Time> {
        None
    }

    fn manufacturer(&self) -> Option<&str> {
        None
    }

    fn device_name(&self) -> Option<&str> {
        None
    }

    fn serial_number(&self) -> Option<&str> {
        None
    }
}

// Based on the https://github.com/svartalf/rust-battery/issues/8
#[test]
fn test_energy_calculation() {
    let data = TestDataSource {
        current_capacity: 3938,
        design_capacity: 4315,
        max_capacity: 4119,
        voltage: 12818,
        amperage: -1037,
        ..Default::default()
    };
    let device: IoKitDevice = data.into();

    // TODO: It would be nice to use some approximate equal asserts here,
    // but for now it is enough to check if values are kinda similar to expected
    // and we are not comparing milliwatts to megawatts
    assert_eq!(device.energy_rate().get::<milliwatt>().floor(), 13292.0);
    assert_eq!(device.energy().get::<watt_hour>().floor(), 50.0);
    assert_eq!(device.energy_full().get::<watt_hour>().floor(), 52.0);
    assert_eq!(device.energy_full_design().get::<watt_hour>().floor(), 55.0);
}
