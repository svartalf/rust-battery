cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub type BatteryIterator = linux::SysFs;
        pub type BatteryDevice = linux::SysFsDevice;
    } else {
        compile_error!("Support for this target OS is not implemented yet!\n \
            You may want to create an issue: https://github.com/svartalf/rust-battery/issues/new");
    }
}

use crate::Battery;

pub fn get() -> impl Iterator<Item=Battery> {
    BatteryIterator::new()
}
