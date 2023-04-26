#![no_std]

use panic_halt as _;
use core::ffi::{c_int, c_char};
use rp2040_hal as hal;
use hal::pac;
use embedded_hal::digital::v2::OutputPin;

extern {
    fn stdio_init_all() -> bool;
    fn puts(s: *const c_char) -> c_int;
    fn sleep_ms(ms: u32);
}

#[no_mangle]
pub extern "C" fn main() {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
    let mut led_pin = pins.gpio25.into_push_pull_output();

    // stdio has to be initialized after initializing pins with Rust HAL
    unsafe { stdio_init_all(); }

    loop {
        unsafe { puts("Hello, world\0".as_ptr() as *const c_char); }

        led_pin.set_high().unwrap();
        unsafe { sleep_ms(500); }

        led_pin.set_low().unwrap();
        unsafe { sleep_ms(500); }
    }
}
