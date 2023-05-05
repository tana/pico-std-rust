#![no_main]

use std::{thread, time::Duration, sync::{Mutex, Arc}};
use core::{ffi::{c_char, c_void, c_int}};

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
    fn esp_pthread_init() -> c_int;
}

extern "C" fn main_task(_: *mut c_void) {
    let counter = Arc::new(Mutex::new(0));

    {
        let counter = Arc::clone(&counter);

        thread::spawn(move || {
            loop {
                {
                    let mut counter = counter.lock().unwrap();
                    *counter = (*counter + 1) % 100;
                }

                println!("Another thread {}", *counter.lock().unwrap());
                thread::sleep(Duration::from_secs(3));
            }
        });
    }

    loop {
        {
            let mut counter = counter.lock().unwrap();
            *counter = (*counter + 1) % 100;
        }

        println!("Main thread {}", *counter.lock().unwrap());
        thread::sleep(Duration::from_millis(1000));
    }
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        esp_newlib_locks_init();
        esp_pthread_init();

        stdio_init_all();

        xTaskCreate(
            main_task, "main_task\0".as_ptr() as *const c_char, 1024,
            core::ptr::null_mut(), 2, core::ptr::null_mut());

        vTaskStartScheduler();  // Start FreeRTOS task scheduler
    }
}