#![no_main]

use std::{thread, time::Duration, sync::{Mutex, Arc}};
use core::{ffi::{c_char, c_void, c_int}};
use rp2040_hal as hal;
use hal::{pac as pac, Sio};
use embedded_hal::digital::v2::OutputPin;

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
    let mut peripherals = pac::Peripherals::take().unwrap();
    let sio = Sio::new(peripherals.SIO);
    let pins = hal::gpio::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS
    );

    // Somehow, stdio has to be initialized after Peripherals::take().
    unsafe { stdio_init_all(); }

    let mut pin = pins.gpio2.into_push_pull_output();

    // Pin toggling thread
    thread::spawn(move || {
        loop {
            pin.set_high().unwrap();
            thread::sleep(Duration::from_millis(500));

            pin.set_low().unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Test of synchronization mechanisms and stdio

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

        xTaskCreate(
            main_task, "main_task\0".as_ptr() as *const c_char, 1024,
            core::ptr::null_mut(), 2, core::ptr::null_mut());

        vTaskStartScheduler();  // Start FreeRTOS task scheduler
    }
}