use core::ffi::{c_char, c_void, c_int};

extern {
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

extern "C" fn main_task_wrapper(parameters: *mut c_void) {
    unsafe { std::mem::transmute::<*mut c_void, fn()>(parameters)(); }
}

pub fn start_main_task(main_task_fn: fn()) {
    unsafe {
        esp_newlib_locks_init();
        esp_pthread_init();

        xTaskCreate(
            main_task_wrapper, "main_task\0".as_ptr() as *const c_char, 1024,
            main_task_fn as *mut c_void, 2, core::ptr::null_mut());

        vTaskStartScheduler();  // Start FreeRTOS task scheduler
    }
}