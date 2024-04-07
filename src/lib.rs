extern crate encoding_rs as encode;
extern crate libc;

use self::encode::SHIFT_JIS;
use std::ffi::CStr;
use std::ffi::CString;
/*
pub trait AsEncode {
    fn as_str(&self) -> String;
}
*/
pub trait ToEncode {
    fn to_shiftjis(&self) -> CString;
    fn to_cstring(&self) -> CString;
}
/*
impl AsEncode for [i8] {
    fn as_str(&self) -> String {
        unsafe {
            let cstr = std::ffi::CStr::from_ptr(self.as_ptr());
            cstr.to_string_lossy().to_string()
        }
    }
}
*/
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
impl ToEncode for *mut i8 {
    fn to_cstring(&self) -> CString {
        unsafe {
            let ptr = CStr::from_ptr(*self);
            CString::new(ptr.to_str().expect("Failed to convert *mut i8 to str"))
                .expect("Failed to convert str to CString")
        }
    }
    fn to_shiftjis(&self) -> CString {
        CString::new("").unwrap()
    }
}
impl ToEncode for *const i8 {
    fn to_cstring(&self) -> CString {
        unsafe {
            let ptr = CStr::from_ptr(*self);
            CString::new(ptr.to_str().expect("Failed to convert *mut i8 to str"))
                .expect("Failed to convert str to CString")
        }
    }
    fn to_shiftjis(&self) -> CString {
        CString::new("").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::c_char;
    use libc::c_int;
    use libc::fclose;
    use libc::fgetc;
    use libc::fopen;
    use libc::EOF;
    use libc::FILE;
    #[test]
    fn test() {
        unsafe {
            let mut s = String::from("");
        
            let mut fp: *mut FILE =
                fopen("test.txt".to_cstring().as_ptr(), "r".to_cstring().as_ptr());
            if fp.is_null() {
                panic!("Failed to open file");
            }
            loop {
                let ch: c_int = fgetc(fp);
                print!("{}", ch as u8 as char);
                if ch == EOF {
                    break;
                }
            }
            fclose(fp);
        }
    }
}
