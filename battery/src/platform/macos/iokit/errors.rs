use std::io;
use std::fmt;
use std::result;
use std::error::Error;

use mach::kern_return::{kern_return_t, KERN_SUCCESS};

pub type Result<T> = result::Result<T, KernError>;

#[derive(Debug)]
pub struct KernError(kern_return_t);

impl fmt::Display for KernError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for KernError {}

impl From<kern_return_t> for KernError {
    fn from(ret: kern_return_t) -> KernError {
        debug_assert!(ret != KERN_SUCCESS);

        KernError(ret)
    }
}

#[macro_export]
macro_rules! r#kern_try {
    ($expr:expr) => (match $expr {
        mach::kern_return::KERN_SUCCESS => (),
        err_code => {
            return Result::Err($crate::platform::macos::iokit::errors::KernError::from(err_code));
        }
    });
    ($expr:expr,) => (r#try!($expr));
}

impl From<KernError> for io::Error {
    fn from(e: KernError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, e)
    }
}
