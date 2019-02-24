extern crate battery;

use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut manager = battery::Manager::new();
    let mut battery = match manager.iter().next() {
        Some(battery) => battery,
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound))
        }
    };

    loop {
        manager.refresh(&mut battery)?;
        println!("{:?}", battery);

        thread::sleep(Duration::from_secs(1));
    }
}
