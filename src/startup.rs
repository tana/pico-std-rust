use core::ffi::{c_char, c_void, c_int};
use rp2040_hal as hal;
use hal::pac;

extern "C" {
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

    fn __real_main(argc: c_int, argv: *const *const c_void) -> c_int;
}

extern "C" fn main_task_wrapper(_parameters: *mut c_void) {
    unsafe {
        __real_main(0, core::ptr::null_mut());
    }
}

// This function is called by the CRT startup routine in place of C main function,
// because of "-Wl,--wrap=main" compiler flag.
#[no_mangle]
extern "C" fn __wrap_main() {
    let (_sys_peripherals, _sys_sio) = super::peripherals::init(
        pac::Peripherals::take().unwrap()
    );

    unsafe {
        esp_newlib_locks_init();
        esp_pthread_init();

        xTaskCreate(
            main_task_wrapper, "main_task\0".as_ptr() as *const c_char, 1024,
            core::ptr::null_mut() as *mut c_void, 2, core::ptr::null_mut());

        vTaskStartScheduler();  // Start FreeRTOS task scheduler
    }
}