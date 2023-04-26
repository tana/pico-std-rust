#![no_std]

use panic_abort as _;
use core::ffi::{c_int, c_char};

extern {
    fn stdio_init_all() -> bool;
    fn puts(s: *const c_char) -> c_int;
    fn sleep_ms(ms: u32);
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe { stdio_init_all(); }

    loop {
        unsafe {
            puts("Hello, world\0".as_ptr() as *const c_char);
            sleep_ms(1000);
        }
    }
}
