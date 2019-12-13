use std::fs;

use approx::assert_abs_diff_eq;

use super::super::SysFsDevice;
use crate::platform::traits::BatteryDevice;
use crate::{State, Technology};

// https://github.com/svartalf/rust-battery/issues/28
//
// This test is not actually covers the `ENODEV` case,
// but it would be nice to have some test cases,
// especially when people are so gracious to provide
// the test data.
#[test]
fn test_issue_28() {
    let root = sysfs_test_suite!(
        "charge_full_design" => 3600000,
        "serial_number" => "41167",
        "technology" => "Li-ion",
        "charge_now" => 725000,
        "present" => 1,
        "manufacturer" => "Hewlett-Packard",
        "type" => "Battery",
        "charge_full" => 3424000,
        "capacity" => 21,
        "cycle_count" => 0,
        "voltage_now" => 10663000,
        "status" => "Discharging",
        "alarm" => 340000,
        "model_name" => "PABAS0241231",
        "voltage_min_design" => 11400000,
        "capacity_level" => "Normal"
    );

    let path = root.into_path();
    let device = SysFsDevice::try_from(path.clone());

    assert!(device.is_ok());
    let device = device.unwrap();

    assert_eq!(device.state(), State::Discharging);
    assert_eq!(device.technology(), Technology::LithiumIon);
    assert!(device.temperature().is_none());
    assert_eq!(device.cycle_count(), None);
    assert_eq!(device.vendor(), Some("Hewlett-Packard"));
    assert_eq!(device.model(), Some("PABAS0241231"));
    assert_eq!(device.serial_number(), Some("41167"));
    assert_abs_diff_eq!(device.state_of_health().value, 0.9511111);
    assert_abs_diff_eq!(device.state_of_charge().value, 0.21);
    assert_abs_diff_eq!(device.energy().value, 29753.998);
    assert_abs_diff_eq!(device.energy_full().value, 140520.95);
    assert_abs_diff_eq!(device.energy_full_design().value, 147744.0);
    assert_abs_diff_eq!(device.energy_rate().value, 0.0);
    assert_abs_diff_eq!(device.voltage().value, 10.663);

    fs::remove_dir_all(path).unwrap();
}
