extern crate battery;

use std::io;
use std::thread;
use std::time::Duration;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    loop {
        println!("{:?}", battery);
        thread::sleep(Duration::from_secs(1));
        manager.refresh(&mut battery)?;
    }
}
