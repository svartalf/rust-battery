use std::io;
use std::time::Duration;

use crate::platform::traits::BatteryDevice;
use super::device::IoKitDevice;
use super::traits::DataSource;

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
    fn new() -> io::Result<Self> where Self: Sized {
        Ok(Default::default())
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

    fn voltage(&self) -> u32 {
        self.voltage
    }

    fn amperage(&self) -> i32 {
        self.amperage
    }

    fn design_capacity(&self) -> u32 {
        self.design_capacity
    }

    fn max_capacity(&self) -> u32 {
        self.max_capacity
    }

    fn current_capacity(&self) -> u32 {
        self.current_capacity
    }

    fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count
    }

    fn time_remaining(&self) -> Option<Duration> {
        None
    }

    fn manufacturer(&self) -> Option<String> {
        None
    }

    fn device_name(&self) -> Option<String> {
        None
    }

    fn serial_number(&self) -> Option<String> {
        None
    }
}

// Based on the https://github.com/svartalf/rust-battery/pull/10
#[test]
fn test_energy_multiplication_overflow() {
    let data = TestDataSource {
        current_capacity: 6232,
        max_capacity: 6324,
        voltage: 12701,
        ..Default::default()
    };
    let device: IoKitDevice = data.into();

    assert!(device.percentage() >= 0.0);
    assert!(device.percentage() <= 100.0);
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

    assert_eq!(device.energy_rate(), 13292);
    assert_eq!(device.energy(), 50477);
    assert_eq!(device.energy_full(), 52797);
    assert_eq!(device.energy_full_design(), 55309);
}
