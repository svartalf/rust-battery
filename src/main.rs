extern crate battery;

use std::time::Duration;

use humantime::format_duration;

fn format(val: Option<Duration>) -> String {
    let d = match val {
        Some(duration) => duration,
        None => Duration::from_secs(0),
    };

    format_duration(d).to_string()
}

fn from_millis(value: u32) -> f32 {
    value as f32 / 1_000.0
}

fn main() {
    for (idx, bat) in battery::get().enumerate() {
        println!("Device:\t\t\t{}", idx);
        println!("vendor:\t\t\t{}", bat.vendor().unwrap_or("N/A"));
        println!("model:\t\t\t{}", bat.model().unwrap_or("N/A"));
        println!("S/N:\t\t\t{}", bat.serial_number().unwrap_or("N/A"));
        println!("battery");
        println!("  state:\t\t{}", bat.state());
        println!("  energy:\t\t{:.2} Wh", from_millis(bat.energy()));
        println!("  energy-full:\t\t{:.2} Wh", from_millis(bat.energy_full()));
        println!("  energy-full-design:\t{:.2} Wh", from_millis(bat.energy_full_design()));
        println!("  energy-rate:\t\t{:.2} Wh", from_millis(bat.energy_rate()));
        println!("  voltage:\t\t{:.2} V", from_millis(bat.voltage()));
        match bat.state() {
            battery::State::Discharging => {
                println!("  time-to-empty\t\t{}", format(bat.time_to_empty()));
            },
            battery::State::Charging => {
                println!("  time-to-full\t\t{}", format(bat.time_to_full()));
            },
            _ => {},
        }
        println!("  percentage:\t\t{:.2}%", bat.percentage());
        print!("  temperature:\t\t");
        match bat.temperature() {
            Some(value) => println!("{:.2} °C", value),
            None => println!("N/A"),
        }
        println!("  capacity:\t\t{}%", bat.capacity());
        println!("  technology:\t\t{}", bat.technology());
    }
}
