#[macro_export]
macro_rules! r#kern_try {
    ($expr:expr) => (match $expr {
        mach::kern_return::KERN_SUCCESS => (),
        err_code => {
            return ::std::result::Result::Err(::std::io::Error::from_raw_os_error(err_code).into())
        }
    });
    ($expr:expr,) => (r#kern_try!($expr));
}
