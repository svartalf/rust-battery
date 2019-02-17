use std::ffi;
use std::ops;
use std::os::windows::ffi::OsStringExt;

#[derive(Debug)]
pub struct WideString(Vec<u16>);

impl Default for WideString {
    fn default() -> Self {
        let buf = vec![0u16; 128];
        Self(buf)
    }
}

impl WideString {
    pub fn with_capacity(capacity: usize) -> WideString {
        let buf = vec![0u16; capacity];
        Self(buf)
    }

    // Note: those a u8, not a u16 received!
    #[inline]
    pub fn truncate(&mut self, bytes: usize) {
        // `-1` is for dropping the traling `\0`
        // which we will always have in our cases
        if let Some(new_size) = (bytes / 2).checked_sub(1) {
            self.0.truncate(new_size);
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<WideString> for String {
    fn from(b: WideString) -> Self {
        let raw = ffi::OsString::from_wide(&b.0);
        raw.to_string_lossy().to_string()
    }
}

impl ops::Deref for WideString {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for WideString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
