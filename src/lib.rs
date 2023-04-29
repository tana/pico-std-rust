use std::{thread, time::Duration};
use core::{ffi::{c_char, c_void}};
// use rp2040_hal as hal;
// use hal::pac;
// use embedded_hal::digital::v2::OutputPin;

extern {
    fn stdio_init_all() -> bool;

    fn vTaskStartScheduler();
    fn xTaskCreate(
        pvTaskCode: extern "C" fn(*mut c_void),
        pcName: *const c_char,
        usStackDepth: u32,
        pvParameters: *mut c_void,
        uxPriority: u32,
        pxCreatedTask: *mut c_void) -> i32;
    
    fn esp_newlib_locks_init();
}

extern "C" fn main_task(_: *mut c_void) {
    loop {
        println!("Hello, world");
        thread::sleep(Duration::from_millis(1000));
    }
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        esp_newlib_locks_init();

        stdio_init_all();

        xTaskCreate(
            main_task, "main_task\0".as_ptr() as *const c_char, 1024,
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