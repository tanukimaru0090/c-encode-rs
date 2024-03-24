
extern crate encoding_rs as encode;
extern crate libc;

use self::encode::SHIFT_JIS;
use std::ffi::CString;

pub trait AsEncode {
    fn as_str(&self) -> String;
}

pub trait ToEncode {
    fn to_shiftjis(&self) -> CString;
    fn to_cstring(&self) -> CString;
}

impl AsEncode for [i8] {
    fn as_str(&self) -> String {
        unsafe {
            let cstr = std::ffi::CStr::from_ptr(self.as_ptr());
            cstr.to_string_lossy().to_string()
        }
    }
}

impl ToEncode for &str {
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(self);
        CString::new(res).unwrap_or_else(|_| CString::new("").unwrap())
    }

    fn to_cstring(&self) -> CString {
        CString::new(*self).expect("Failed to convert &str to CString")
    }
}

impl ToEncode for String {
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(self);
        CString::new(res).unwrap_or_else(|_| CString::new("").unwrap())
    }

    fn to_cstring(&self) -> CString {
        CString::new(self.as_str()).expect("Failed to convert String to CString")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::printf;

    #[test]
    fn test() {
        unsafe {
            let mes = "hello world!".to_cstring();
            printf(b"%s\n\0" as *const u8 as *const i8, mes.as_ptr());
            // Japanese UTF8
            {
                let mes = "ハロー ワールド!".to_cstring();
                printf(b"%s\n\0" as *const u8 as *const i8, mes.as_ptr());
            }
        }
    }
}
