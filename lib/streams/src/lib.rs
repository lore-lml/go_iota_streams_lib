mod api;

use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn hello_from_rust(str: *const c_char){
    let c_str = unsafe{ CStr::from_ptr(str) };
    println!("Hello {} from Rust", c_str.to_str().unwrap());
}