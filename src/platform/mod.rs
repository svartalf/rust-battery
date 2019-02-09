use std::io;
use std::iter;

use crate::Battery;

#[cfg(target_os = "linux")]
mod linux;

/// Creates an iterator which yields information about available batteries.
///
/// ```edition2018
/// # use std::error::Error;
/// # use battery;
/// #
/// # fn main() -> Result<(), Box<Error>> {
/// for (idx, bat) in battery::get().enumerate() {
///     let bat = bat?;
///     println!("BAT{}: {}, {:.2}%", idx, bat.state(), bat.current() / bat.full() * 100.0);
/// }
/// # Ok(())
/// # }
pub fn get() -> impl iter::Iterator<Item=io::Result<Battery>> {
    #[cfg(target_os = "linux")]
    self::linux::SysFs::new()
}