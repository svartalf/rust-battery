// https://docs.microsoft.com/en-us/windows/desktop/power/power-management-portal

mod device;
mod ffi;
mod iterator;
mod manager;

pub use self::device::PowerDevice;
pub use self::iterator::PowerIterator;
pub use self::manager::PowerManager;
