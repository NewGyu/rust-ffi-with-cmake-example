use std::ffi::{CStr, CString};
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use wasm_bindgen::prelude::*;

#[cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn print_by_c(s: &str) {
    let cstring = CString::new(s).unwrap();
    unsafe { ffi_example_sys::print_str(cstring.as_ptr()) }
}

#[cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn hello_by_c() -> String {
    unsafe { CStr::from_ptr(ffi_example_sys::hello()) }
        .to_str()
        .unwrap()
        .to_string()
}
