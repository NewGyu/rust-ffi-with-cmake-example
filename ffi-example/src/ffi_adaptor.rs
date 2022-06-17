use std::ffi::{CStr, CString};

pub fn print_str(s: &str) {
    let cstring = CString::new(s).unwrap();
    unsafe { ffi_example_sys::print_str(cstring.as_ptr()) }
}

pub fn hello() -> String {
    unsafe { CStr::from_ptr(ffi_example_sys::hello()) }
        .to_str()
        .unwrap()
        .to_string()
}
