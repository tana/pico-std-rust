#![no_std]

use panic_halt as _;
use core::ffi::{c_int, c_char, c_void};
use rp2040_hal as hal;
use hal::pac;
use embedded_hal::digital::v2::OutputPin;

extern {
    fn stdio_init_all() -> bool;
    fn puts(s: *const c_char) -> c_int;

    fn vTaskStartScheduler();
    fn xTaskCreate(
        pvTaskCode: extern "C" fn(*mut c_void),
        pcName: *const c_char,
        usStackDepth: u32,
        pvParameters: *mut c_void,
        uxPriority: u32,
        pxCreatedTask: *mut c_void) -> i32;
    fn vTaskDelay(xTicksToDelay: u32);
}

extern "C" fn blink_task(_: *mut c_void) {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
    let mut led_pin = pins.gpio25.into_push_pull_output();

    unsafe {
        // stdio has to be initialized after initializing pins with Rust HAL
        stdio_init_all();


        xTaskCreate(
            print_task, "print\0".as_ptr() as *const c_char, 1024,
            core::ptr::null_mut(), 1, core::ptr::null_mut());
    }

    loop {
        led_pin.set_high().unwrap();
        unsafe { vTaskDelay(500); }

        led_pin.set_low().unwrap();
        unsafe { vTaskDelay(500); }
    }
}

extern "C" fn print_task(_: *mut c_void) {
    loop {
        unsafe {
            puts("Hello, world\0".as_ptr() as *const c_char);
            vTaskDelay(1000);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        xTaskCreate(
            blink_task, "blink\0".as_ptr() as *const c_char, 1024,
            core::ptr::null_mut(), 2, core::ptr::null_mut());

        vTaskStartScheduler();  // Start FreeRTOS task scheduler
    }
}

#[no_mangle]
pub extern "C" fn vApplicationMallocFailedHook() {
    panic!("malloc failed")
}

#[no_mangle]
pub extern "C" fn vApplicationStackOverflowHook() {
    panic!("stack overflow")
}