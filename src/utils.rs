use std::ffi::CString;

pub fn lossy_cstring<S: AsRef<str>>(string: S) -> CString {
    match CString::new(string.as_ref()) {
        Ok(cstr) => cstr,
        Err(_) => CString::new(string.as_ref().replace('\0', "")).expect("string has no nulls"),
    }
}
