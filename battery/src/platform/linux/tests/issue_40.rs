use std::fs;

use approx::assert_abs_diff_eq;

use crate::{State, Technology};
use crate::platform::traits::BatteryDevice;
use super::super::SysFsDevice;

// https://github.com/svartalf/rust-battery/issues/40
//
// Both missing `energy_full_design` and `charge_full_design`
// should be handled correctly.
//
// See https://github.com/starship/starship/issues/613#issuecomment-548873632
#[test]
fn test_issue_40() {
    let root = sysfs_test_suite!(
        "capacity" => 83,
        "charge_counter" => 2584,
        "current_now" => 898,
        "health" => "Good",
        "present" => 1,
        "status" => "Discharging",
        "technology" => "Li-ion",
        "temp" => 258,
        "type" => "Battery",
        "voltage_now" => 11829000
    );

    let path = root.into_path();
    let device = SysFsDevice::try_from(path.clone());

    assert!(device.is_ok());
    let device = device.unwrap();

    assert_eq!(device.state(), State::Discharging);
    assert_eq!(device.technology(), Technology::LithiumIon);
    assert_eq!(device.cycle_count(), None);
    assert!(device.vendor().is_none());
    assert!(device.model().is_none());
    assert!(device.serial_number().is_none());
    assert!(device.temperature().is_some());
    assert_abs_diff_eq!(device.temperature().unwrap().value, 298.94998); // Kelvins

    assert_abs_diff_eq!(device.state_of_health().value, 1.0);
    assert_abs_diff_eq!(device.state_of_charge().value, 0.83);
    assert_abs_diff_eq!(device.energy().value, 0.0);
    assert_abs_diff_eq!(device.energy_full().value, 0.0);
    assert_abs_diff_eq!(device.energy_full_design().value, 0.0);
    assert_abs_diff_eq!(device.energy_rate().value, 0.00089799997);
    assert_abs_diff_eq!(device.voltage().value, 11.8289995);


//    &device.source = InstantData {
//    state_of_health: 1.0,
//    state_of_charge: 0.83,
//    energy: 0.0 m^2 kg^1 s^-2,
//    energy_full: 0.0 m^2 kg^1 s^-2,
//    energy_full_design: 0.0 m^2 kg^1 s^-2,
//    energy_rate: 0.00089799997 m^2 kg^1 s^-3,
//    voltage: 11.8289995 m^2 kg^1 s^-3 A^-1,
//    state: Discharging,
//    temperature: Some(
//        298.94998 K^1,
//    ),
//    cycle_count: None,
//}


    fs::remove_dir_all(path).unwrap();
}
