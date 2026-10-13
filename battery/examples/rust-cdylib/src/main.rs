use battery::{Manager, battery_last_error_length, battery_last_error_message};
use std::io;

fn last_error() -> io::Error {
    let length = battery_last_error_length();
    let message = if length > 0 {
        let mut buffer = vec![0u8; length as usize];
        let written = battery_last_error_message(&mut buffer);
        if written > 0 {
            String::from_utf8_lossy(&buffer[..written as usize]).into_owned()
        } else {
            "unknown battery error".to_owned()
        }
    } else {
        "unknown battery error".to_owned()
    };
    io::Error::new(io::ErrorKind::Other, message)
}

fn main() -> io::Result<()> {
    let manager = Manager::new_ffi().ok_or_else(last_error)?;
    let batteries = manager.batteries_ffi().ok_or_else(last_error)?;

    for battery in batteries {
        let battery = battery.map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

        let vendor = battery.vendor_ffi();
        let model = battery.model_ffi();

        println!(
            "{} {}: {:?}, {:?}, {:.1}%",
            vendor.as_ref().map(|value| value.as_str()).unwrap_or("unknown vendor"),
            model.as_ref().map(|value| value.as_str()).unwrap_or("unknown model"),
            battery.technology(),
            battery.state(),
            battery.state_of_charge_ffi(),
        );
    }

    Ok(())
}
