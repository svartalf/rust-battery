cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub type BatteryIterator = linux::SysFs;
        pub type BatteryDevice = linux::SysFsDevice;
    } else if #[cfg(target_os = "macos")] {
        mod darwin;

        pub type BatteryIterator = darwin::IoReg;
        pub type BatteryDevice = darwin::IoRegDevice;
    } else {
        compile_error!("Support for this target OS is not implemented yet!\n \
            You may want to create an issue: https://github.com/svartalf/rust-battery/issues/new");
    }
}

use crate::Battery;

/// Returns an iterator with batteries states.
///
/// # Example
///
/// ```edition2018
/// # use battery::get;
/// #
/// # fn main() {
/// for (idx, bat) in get().enumerate() {
///     println!("Battery {} state: {}", idx, bat.state());
/// }
/// # }
/// ```
pub fn get() -> impl Iterator<Item=Battery> {
    BatteryIterator::new()
}
