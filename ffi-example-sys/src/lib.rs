#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern "C" {
    pub fn print_str(str_: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn hello() -> *const ::std::os::raw::c_char;
}
