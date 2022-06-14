use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    pub fn gethostname(name: *mut c_char, len: usize) -> i32;
}

fn main() {
    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr() as *mut c_char;

    unsafe {
        gethostname(ptr, len);
        println!("{:?}", CStr::from_ptr(ptr));
    }
}
