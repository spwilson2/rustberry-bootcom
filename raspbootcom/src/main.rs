//include!(concat!(env!("OUT_DIR"), "/librasbootcom."));
extern crate libc;

use std::ffi;
use libc::{c_int,c_char};
use std::env;


extern {
    fn run(argc: c_int, argv :*const *const c_char) -> c_int;
}

fn main() {
    unsafe {
        let argv:Vec<ffi::CString> = env::args().into_iter().map(|arg| { ffi::CString::new(arg).unwrap() } ).collect();
        let args:Vec<*const c_char> = argv.into_iter().map(|arg| { arg.into_raw() as *const c_char} ).collect();
        run(args.len() as c_int, args.as_ptr() as *const *const c_char);
    }
}
