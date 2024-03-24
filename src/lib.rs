
extern crate encoding_rs as encode;

use self::encode::SHIFT_JIS;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::*;
use std::vec::Vec;

pub trait AsEncode {
    fn as_str(&self) -> String;
}
//エンコーディング変換
pub trait ToEncode {
    fn to_shiftjis(&self) -> CString;
    fn to_cstring(&self) -> CString;
    //fn to_tchar(&self) ->;
}
impl AsEncode for [i8]{
    fn as_str(&self)->String{
        unsafe{
            let mut cstr = std::ffi::CStr::from_ptr(self.as_ptr());
            let mut string = cstr.to_str().unwrap().to_string();
            return string;
        }
    }
}
impl AsEncode for CString {
    fn as_str(&self) -> String {
        unsafe {
            let mut cstr = std::ffi::CStr::from_ptr(self.as_ptr());
            let mut string = cstr.to_str().unwrap().to_string();
            return string;
        }
    }
}
impl ToEncode for &str {
    // &strをshiftjisのエンコーディングとして変換し、CStringを返す
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(self);
        let mut string = CString::new(res).unwrap();
        return string;
    }
    // &strをデフォルトのUTF-8のエンコーディングとして変換し、CStringを返す
    fn to_cstring(&self) -> CString {
        return CString::new(self.as_bytes()).unwrap();
    }
}
impl ToEncode for String {
    // Stringをshiftjisのエンコーディングとして変換し、CStringを返す
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(&self);
        let mut string = CString::new(res).unwrap();
        return string;
    }
    // StringをデフォルトのUTF-8のエンコーディングとして変換し、CStringを返す
    fn to_cstring(&self) -> CString {
        return CString::new(self.clone().into_bytes()).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){

    }
}
