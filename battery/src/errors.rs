//! Errors handling

use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// Battery routines error.
///
/// Since all operations are basically I/O of some kind,
/// this is a thin wrapper around `::std::io::Error` with option
/// to store custom description for debugging purposes.
#[derive(Debug)]
pub struct Error {
    source: io::Error,
    description: Option<Cow<'static, str>>,
}

impl Error {
    #[cfg(any(not(feature = "co3"), feature = "export"))]
    #[allow(unused)]
    pub(crate) fn new<T>(e: io::Error, description: T) -> Error
    where
        T: Into<Cow<'static, str>>,
    {
        Error {
            source: e,
            description: Some(description.into()),
        }
    }

    #[cfg(any(not(feature = "co3"), feature = "export"))]
    #[allow(unused)]
    pub(crate) fn not_found<T>(description: T) -> Error
    where
        T: Into<Cow<'static, str>>,
    {
        Error {
            source: io::Error::from(io::ErrorKind::NotFound),
            description: Some(description.into()),
        }
    }

    #[cfg(any(not(feature = "co3"), feature = "export"))]
    #[allow(unused)]
    pub(crate) fn invalid_data<T>(description: T) -> Error
    where
        T: Into<Cow<'static, str>>,
    {
        Error {
            source: io::Error::from(io::ErrorKind::InvalidData),
            description: Some(description.into()),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.description {
            Some(desc) => write!(f, "{}", desc),
            None => self.source.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            source: e,
            description: None,
        }
    }
}

#[cfg(feature = "export")]
use std::cell::RefCell;

#[cfg(feature = "export")]
thread_local! {
    static LAST_FFI_ERROR: RefCell<Option<Box<dyn StdError>>> = RefCell::new(None);
}

#[cfg(feature = "export")]
pub(crate) fn set_ffi_error<E: StdError + 'static>(error: E) {
    LAST_FFI_ERROR.with(|previous| *previous.borrow_mut() = Some(Box::new(error)));
}

#[cfg(feature = "export")]
pub(crate) fn clear_ffi_error() {
    LAST_FFI_ERROR.with(|previous| *previous.borrow_mut() = None);
}

#[cfg(feature = "export")]
pub(crate) fn into_ffi_option<T>(result: Result<T>) -> Option<T> {
    match result {
        Ok(value) => {
            clear_ffi_error();
            Some(value)
        }
        Err(error) => {
            set_ffi_error(error);
            None
        }
    }
}

#[cfg(feature = "export")]
pub fn battery_have_last_error() -> core::ffi::c_int {
    LAST_FFI_ERROR.with(|previous| i32::from(previous.borrow().is_some()))
}

#[cfg(feature = "export")]
pub(crate) fn battery_last_error_length() -> core::ffi::c_int {
    LAST_FFI_ERROR.with(|previous| match previous.borrow().as_ref() {
        Some(error) => {
            let length = error.to_string().len();
            if length >= core::ffi::c_int::MAX as usize {
                -1
            } else {
                (length + 1) as core::ffi::c_int
            }
        }
        None => 0,
    })
}

/// Copies the message and consumes it only after a successful write.
#[cfg(feature = "export")]
pub(crate) fn battery_last_error_message(buffer: &mut [u8]) -> core::ffi::c_int {
    LAST_FFI_ERROR.with(|previous| {
        let mut previous = previous.borrow_mut();
        let error = match previous.as_ref() {
            Some(error) => error,
            None => return 0,
        };
        let message = error.to_string();
        if message.len() >= buffer.len() || message.len() > core::ffi::c_int::MAX as usize {
            return -1;
        }

        buffer[..message.len()].copy_from_slice(message.as_bytes());
        buffer[message.len()] = 0;
        *previous = None;
        message.len() as core::ffi::c_int
    })
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
#[cfg(any(not(feature = "co3"), feature = "export"))]
mod nix_impl {
    use std::io;

    use super::Error;

    impl From<nix::Error> for Error {
        fn from(e: nix::Error) -> Self {
            match e {
                nix::Error::Sys(errno) => Error {
                    source: io::Error::from_raw_os_error(errno as i32),
                    description: Some(errno.desc().into()),
                },
                nix::Error::InvalidPath => Error {
                    source: io::Error::new(io::ErrorKind::InvalidInput, e),
                    description: Some("Invalid path".into()),
                },
                nix::Error::InvalidUtf8 => Error {
                    source: io::Error::new(io::ErrorKind::InvalidData, e),
                    description: Some("Invalid UTF-8 string".into()),
                },
                nix::Error::UnsupportedOperation => Error {
                    source: io::Error::new(io::ErrorKind::Other, e),
                    description: Some("Unsupported operation".into()),
                },
            }
        }
    }
}
