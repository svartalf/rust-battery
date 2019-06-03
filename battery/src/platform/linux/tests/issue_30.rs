use std::fs;

use super::super::SysFsDevice;

// https://github.com/svartalf/rust-battery/issues/30
//
// Both missing `energy_full_design` and `charge_full_design`
// should not lead to the infinite recursion and as a consequence,
// to the stack overflow
#[test]
fn test_issue_30() {
    let root = sysfs_test_suite!(
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

    // Since both needed files are missing, it will be an error here
    assert!(device.is_err());

    fs::remove_dir_all(path).unwrap();
}
