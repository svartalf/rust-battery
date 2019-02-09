extern crate battery;

use battery::State;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    for (idx, battery) in battery::get().enumerate() {
        let battery = battery?;

        print!("BAT{}: {}, {:.2}%", idx, battery.state(), battery.current() / battery.full() * 100.0);

        match battery.state() {
            State::Discharging => {
                if *battery.charge_rate() == 0.0 {
                    print!(", discharging at zero rate - will never fully discharge");
                } else {
                    // TODO: Fix time representation
                    print!(", {:.2} hour remaining", battery.current() / battery.charge_rate());
                }
            },
            State::Charging => {
                if *battery.charge_rate() == 0.0 {
                    print!(", charging at zero rate - will never fully charge");
                } else {
                    // TODO: Fix time representation
                    print!(", {:.2} hour until charged", (battery.full() - battery.current()) / battery.charge_rate());
                }
            }
            _ => {}
        }

        println!("  [Voltage: {:.2}V (design: {:.2}V)]", battery.voltage(), battery.design_voltage());
    }

    Ok(())
}