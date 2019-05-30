use std::fs;

use super::super::SysFsDevice;

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

    fs::remove_dir_all(path).unwrap();
}
