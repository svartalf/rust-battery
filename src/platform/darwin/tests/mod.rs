use std::io;
use std::time::Duration;

use plist;

use crate::{State, Technology};
use crate::types::Device;
use super::IoRegDevice;

static EXAMPLE: &'static str = include_str!("./example.plist");

#[test]
fn test_parse() {
    let cursor = io::Cursor::new(EXAMPLE);
    let devices: Vec<IoRegDevice> = plist::from_reader_xml(cursor).unwrap();
    let device = devices.iter().next().unwrap();

    assert_eq!(device.capacity(), 91.37062937062937);
    assert_eq!(device.energy(), 21.02975);
    assert_eq!(device.energy_full(), 49.814125);
    assert_eq!(device.energy_full_design(), 54.51875);
    assert_eq!(device.energy_rate(), 0.2745);
    assert_eq!(device.percentage(), 42.21643961426604);
    assert_eq!(device.state(), State::Discharging);
    assert_eq!(device.voltage(), 7.625);
    assert_eq!(device.temperature(), 29.74);
    assert_eq!(device.vendor(), None);
    assert_eq!(device.model(), Some("bq20z451"));
    assert_eq!(device.serial_number(), Some("D8654840J4TF90KBK"));
    assert_eq!(device.technology(), Technology::Unknown);
    assert_eq!(device.time_to_full(), None);
    assert_eq!(device.time_to_empty(), Some(Duration::from_secs(4597)));
}
