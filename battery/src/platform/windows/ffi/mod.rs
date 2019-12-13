#![allow(clippy::cast_ptr_alignment)]

use std::default::Default;
use std::io;
use std::iter;
use std::mem;
use std::ops::DerefMut;

use winapi::ctypes;
use winapi::shared::{basetsd, devguid, minwindef, ntdef, windef, winerror};
use winapi::um::{errhandlingapi, fileapi, handleapi, ioapiset, minwinbase, setupapi, winbase, winnt};

mod ioctl;
mod wide_string;
mod wrappers;

pub(crate) use self::ioctl::BatteryQueryInformation;
use self::wide_string::WideString;
use self::wrappers::*;

#[inline]
fn get_last_error() -> io::Error {
    let error_type = unsafe { errhandlingapi::GetLastError() };
    io::Error::from_raw_os_error(error_type as i32)
}

#[derive(Debug)]
pub struct DeviceIterator {
    device: setupapi::HDEVINFO,
    current: minwindef::DWORD,
}

impl DeviceIterator {
    pub fn new() -> io::Result<DeviceIterator> {
        let hdev = unsafe {
            setupapi::SetupDiGetClassDevsW(
                &devguid::GUID_DEVCLASS_BATTERY,
                ntdef::NULL as winnt::PCWSTR,
                ntdef::NULL as windef::HWND,
                setupapi::DIGCF_PRESENT | setupapi::DIGCF_DEVICEINTERFACE,
            )
        };
        if hdev == handleapi::INVALID_HANDLE_VALUE {
            Err(get_last_error())
        } else {
            Ok(DeviceIterator {
                device: hdev,
                current: 0,
            })
        }
    }

    fn get_interface_data(&self) -> io::Result<setupapi::SP_DEVICE_INTERFACE_DATA> {
        let mut data = setupapi::SP_DEVICE_INTERFACE_DATA::default();
        data.cbSize = mem::size_of::<setupapi::SP_DEVICE_INTERFACE_DATA>() as u32;
        let result = unsafe {
            setupapi::SetupDiEnumDeviceInterfaces(
                self.device,
                ntdef::NULL as *mut setupapi::SP_DEVINFO_DATA,
                &devguid::GUID_DEVCLASS_BATTERY,
                self.current,
                &mut data,
            )
        };

        // TODO: Add trace
        if result == 0 { Err(get_last_error()) } else { Ok(data) }
    }

    fn get_interface_detail(&self, data: &mut setupapi::SP_DEVICE_INTERFACE_DATA) -> io::Result<InterfaceDetailData> {
        let mut buf_size: minwindef::DWORD = 0;
        unsafe {
            setupapi::SetupDiGetDeviceInterfaceDetailW(
                self.device,
                data,
                ntdef::NULL as setupapi::PSP_DEVICE_INTERFACE_DETAIL_DATA_W,
                0,
                &mut buf_size,
                0 as setupapi::PSP_DEVINFO_DATA,
            )
        };
        let result = unsafe { errhandlingapi::GetLastError() };
        if result != winerror::ERROR_INSUFFICIENT_BUFFER {
            return Err(io::Error::from_raw_os_error(result as i32));
        }

        let mut pdidd = unsafe {
            winbase::LocalAlloc(minwinbase::LPTR, buf_size as basetsd::SIZE_T)
                as setupapi::PSP_DEVICE_INTERFACE_DETAIL_DATA_W
        };
        unsafe {
            (*pdidd).cbSize = mem::size_of::<setupapi::SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as u32;
        }
        unsafe {
            setupapi::SetupDiGetDeviceInterfaceDetailW(
                self.device,
                data,
                pdidd,
                buf_size,
                &mut buf_size,
                0 as setupapi::PSP_DEVINFO_DATA,
            )
        };
        let result = unsafe { errhandlingapi::GetLastError() };
        if result != 0 {
            return Err(io::Error::from_raw_os_error(result as i32));
        }

        Ok(pdidd.into())
    }

    fn get_handle(&self, pdidd: &InterfaceDetailData) -> io::Result<Handle> {
        let device_path = unsafe { (***pdidd).DevicePath.as_ptr() };
        let file = unsafe {
            fileapi::CreateFileW(
                device_path,
                winnt::GENERIC_READ | winnt::GENERIC_WRITE,
                winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE,
                ntdef::NULL as minwinbase::LPSECURITY_ATTRIBUTES,
                fileapi::OPEN_EXISTING,
                winnt::FILE_ATTRIBUTE_NORMAL,
                ntdef::NULL,
            )
        };
        if file == handleapi::INVALID_HANDLE_VALUE {
            Err(get_last_error())
        } else {
            Ok(file.into())
        }
    }

    fn get_tag(&self, handle: &mut Handle) -> io::Result<ioctl::BatteryQueryInformation> {
        let mut query = ioctl::BatteryQueryInformation::default();
        let mut wait_timeout: minwindef::DWORD = 0;
        let mut bytes_returned: minwindef::DWORD = 0;

        let res = unsafe {
            ioapiset::DeviceIoControl(
                **handle as *mut _ as *mut ctypes::c_void,
                ioctl::IOCTL_BATTERY_QUERY_TAG,
                &mut wait_timeout as *mut _ as minwindef::LPVOID,
                mem::size_of::<minwindef::DWORD>() as minwindef::DWORD,
                &mut query.BatteryTag as *mut _ as minwindef::LPVOID,
                mem::size_of::<ntdef::ULONG>() as minwindef::DWORD,
                &mut bytes_returned as *mut _,
                ntdef::NULL as minwinbase::LPOVERLAPPED,
            )
        };

        if res == 0 || query.BatteryTag == 0 {
            return Err(get_last_error());
        }

        Ok(query)
    }

    pub fn prepare_handle(&self) -> io::Result<Handle> {
        let mut interface_data = self.get_interface_data()?;
        let interface_detail_data = self.get_interface_detail(&mut interface_data)?;

        self.get_handle(&interface_detail_data)
    }
}

impl iter::Iterator for DeviceIterator {
    type Item = DeviceHandle;

    fn next(&mut self) -> Option<Self::Item> {
        let mut handle = match self.prepare_handle() {
            Ok(h) => h,
            Err(_) => return None,
        };

        let tag = match self.get_tag(&mut handle) {
            Ok(tag) => tag,
            Err(_) => return None,
        };

        self.current += 1;

        Some(DeviceHandle {
            //            interface_details: interface_detail_data,
            handle: handle,
            tag: tag,
        })
    }
}

impl Drop for DeviceIterator {
    fn drop(&mut self) {
        let res = unsafe { setupapi::SetupDiDestroyDeviceInfoList(self.device) };
        debug_assert_eq!(res, 1, "Unable to destroy DeviceInfoList");
    }
}

// Our inner representation of the battery device.
#[derive(Debug)]
pub struct DeviceHandle {
    //    interface_details: InterfaceDetailData,
    pub handle: Handle,
    // TODO: Carry only `.BatteryTag` field ?
    pub tag: ioctl::BatteryQueryInformation,
}

impl DeviceHandle {
    pub fn information(&mut self) -> io::Result<ioctl::BatteryInformation> {
        let mut query = ioctl::BatteryQueryInformation::default();
        query.BatteryTag = self.tag.BatteryTag;
        let mut out = ioctl::BatteryInformation::default();
        let mut bytes_returned: minwindef::DWORD = 0;

        let res = unsafe {
            ioapiset::DeviceIoControl(
                *self.handle,
                ioctl::IOCTL_BATTERY_QUERY_INFORMATION,
                query.deref_mut() as *mut _ as minwindef::LPVOID,
                // Since wrapper is a newtype struct, `mem::size_of` will be the same as with
                // underline structure. Yet, this might lead to bug if wrapper structure will change.
                // TODO: Get memory size of the underline struct directly
                mem::size_of::<ioctl::BatteryQueryInformation>() as minwindef::DWORD,
                &mut out as *mut _ as minwindef::LPVOID,
                mem::size_of::<ioctl::BatteryInformation>() as minwindef::DWORD,
                &mut bytes_returned as *mut _,
                ntdef::NULL as minwinbase::LPOVERLAPPED,
            )
        };

        if res == 0 { Err(get_last_error()) } else { Ok(out) }
    }

    pub fn status(&mut self) -> io::Result<ioctl::BatteryStatus> {
        let mut query = ioctl::BatteryWaitStatus::default();
        query.BatteryTag = self.tag.BatteryTag;
        let mut out = ioctl::BatteryStatus::default();
        let mut bytes_returned: minwindef::DWORD = 0;

        let res = unsafe {
            ioapiset::DeviceIoControl(
                *self.handle,
                ioctl::IOCTL_BATTERY_QUERY_STATUS,
                query.deref_mut() as *mut _ as minwindef::LPVOID,
                // Since wrapper is a newtype struct, `mem::size_of` will be the same as with
                // underline structure. Yet, this might lead to bug if wrapper structure will change.
                // TODO: Get memory size of the underline struct directly
                mem::size_of::<ioctl::BatteryWaitStatus>() as minwindef::DWORD,
                &mut out as *mut _ as minwindef::LPVOID,
                mem::size_of::<ioctl::BatteryStatus>() as minwindef::DWORD,
                &mut bytes_returned as *mut _,
                ntdef::NULL as minwinbase::LPOVERLAPPED,
            )
        };

        if res == 0 { Err(get_last_error()) } else { Ok(out) }
    }

    // 10ths of a degree Kelvin (or decikelvin)
    pub fn temperature(&mut self) -> io::Result<ntdef::ULONG> {
        let mut query = ioctl::BatteryQueryInformation::default();
        query.BatteryTag = self.tag.BatteryTag;
        query.InformationLevel = ioctl::info_level::BatteryTemperature;
        let mut out: ntdef::ULONG = 0;
        let mut bytes_returned: minwindef::DWORD = 0;

        let res = unsafe {
            ioapiset::DeviceIoControl(
                *self.handle,
                ioctl::IOCTL_BATTERY_QUERY_INFORMATION,
                query.deref_mut() as *mut _ as minwindef::LPVOID,
                // Since wrapper is a newtype struct, `mem::size_of` will be the same as with
                // underline structure. Yet, this might lead to bug if wrapper structure will change.
                // TODO: Get memory size of the underline struct directly
                mem::size_of::<ioctl::BatteryQueryInformation>() as minwindef::DWORD,
                &mut out as *mut _ as minwindef::LPVOID,
                mem::size_of::<ntdef::ULONG>() as minwindef::DWORD,
                &mut bytes_returned as *mut _,
                ntdef::NULL as minwinbase::LPOVERLAPPED,
            )
        };

        if res == 0 { Err(get_last_error()) } else { Ok(out) }
    }

    pub fn device_name(&mut self) -> io::Result<String> {
        self.query_string(ioctl::info_level::BatteryDeviceName)
    }

    pub fn manufacture_name(&mut self) -> io::Result<String> {
        self.query_string(ioctl::info_level::BatteryManufactureName)
    }

    pub fn serial_number(&mut self) -> io::Result<String> {
        self.query_string(ioctl::info_level::BatterySerialNumber)
    }

    fn query_string(&mut self, level: ioctl::info_level::BATTERY_QUERY_INFORMATION_LEVEL) -> io::Result<String> {
        let mut query = ioctl::BatteryQueryInformation::default();
        query.BatteryTag = self.tag.BatteryTag;
        query.InformationLevel = level;
        let mut out = WideString::default();
        let mut bytes_returned: minwindef::DWORD = 0;

        let res = unsafe {
            ioapiset::DeviceIoControl(
                *self.handle,
                ioctl::IOCTL_BATTERY_QUERY_INFORMATION,
                query.deref_mut() as *mut _ as minwindef::LPVOID,
                // Since wrapper is a newtype struct, `mem::size_of` will be the same as with
                // underline structure. Yet, this might lead to bug if wrapper structure will change.
                // TODO: Get memory size of the underline struct directly
                mem::size_of::<ioctl::BatteryQueryInformation>() as minwindef::DWORD,
                out.as_mut_ptr() as *mut _ as minwindef::LPVOID,
                (out.len() * 2) as minwindef::DWORD,
                &mut bytes_returned as *mut _,
                ntdef::NULL as minwinbase::LPOVERLAPPED,
            )
        };

        out.truncate(bytes_returned as usize);

        if res == 0 {
            Err(get_last_error())
        } else {
            Ok(out.into())
        }
    }
}
