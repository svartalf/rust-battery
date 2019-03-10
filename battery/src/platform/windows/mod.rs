// https://docs.microsoft.com/en-us/windows/desktop/power/power-management-portal

mod ffi;
mod device;
mod iterator;
mod manager;

pub use self::device::PowerDevice;
pub use self::iterator::PowerIterator;
pub use self::manager::PowerManager;
