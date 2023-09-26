use core::ffi::{c_int, c_uint};

extern "C" {
    pub fn cyw43_arch_init() -> c_int;

    pub fn cyw43_arch_gpio_put(wl_gpio: c_uint, value: bool);
}