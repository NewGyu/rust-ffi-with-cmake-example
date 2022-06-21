use std::ffi::{CStr, CString};
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use wasm_bindgen::prelude::*;

#[cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn print_str(s: &str) {
    if cfg!(no_ffi) {
        println!("Printed by rust: {}",s)
    } else {
        let cstring = CString::new(s).unwrap();
        unsafe { ffi_example_sys::print_str(cstring.as_ptr()) }
    }
}

#[cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn hello() -> String {
    if cfg!(no_ffi) {
        "from rust".to_string()
    } else {
        unsafe { CStr::from_ptr(ffi_example_sys::hello()) }
            .to_str()
            .unwrap()
            .to_string()
    }
}
