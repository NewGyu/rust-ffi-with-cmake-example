mod lib;
//mod sequence;
//use sequence::Sequence;

fn main() {
    lib::print_str("Hello FFI!!");
    println!("Hi, {}", lib::hello());
    /*
        let slice = &[1, 13, 5];
        assert_eq!(
            unsafe { ffi_example_sys::sum(slice.as_ptr(), slice.len()) },
            19
        );

        let size = 101;
        let ptr = unsafe { ffi_example_sys::get_sequential_array(size) };
        let slice = unsafe { std::slice::from_raw_parts(ptr, size) };
        assert_eq!(slice.iter().fold(0, |sum, a| sum + a), 5050);
        drop(slice);
        unsafe {
            ffi_example_sys::free_array(ptr);
        }

        let ptr = unsafe { ffi_example_sys::get_sequential_array(size) };
        let cvec =
            unsafe { c_vec::CVec::new_with_dtor(ptr, size, |ptr| ffi_example_sys::free_array(ptr)) };
        assert_eq!(cvec.iter().fold(0, |sum, a| sum + a), 5050);

        let seq = Sequence::new(size);
        assert_eq!(seq.iter().fold(0, |sum, a| sum + a), 5050);
    */
}
