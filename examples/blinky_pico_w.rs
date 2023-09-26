/// Make the built-in LED (connected to the wifi module) on a Raspberry Pi Pico board blink at 1 Hz

use std::{time::Duration, thread};
use pico_std::cyw43::*;

fn main() {
    let err = unsafe { cyw43_arch_init() };
    if err != 0 {
        panic!("Cannot initialize the wifi module (code = {})", err);
    }

    loop {
        unsafe { cyw43_arch_gpio_put(0, true); }
        thread::sleep(Duration::from_millis(500));

        unsafe { cyw43_arch_gpio_put(0, false); }
        thread::sleep(Duration::from_millis(500));
    }
}
