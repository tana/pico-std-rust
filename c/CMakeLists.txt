cmake_minimum_required(VERSION 3.19)

include(FetchContent)

set(Rust_CARGO_TARGET "thumbv6m-none-eabi")

# Automatically download Pico SDK
set(PICO_SDK_FETCH_FROM_GIT on)
include(pico_sdk_import.cmake)

project(pico-std-rust)

pico_sdk_init()

# Automatically download FreeRTOS
FetchContent_Declare(
    freertos_kernel
    GIT_REPOSITORY https://github.com/FreeRTOS/FreeRTOS-Kernel.git
    GIT_TAG 570ade4001e50adbf06a074582ea993af562e0e1 # smp branch
)
FetchContent_GetProperties(freertos_kernel)
if(NOT freertos_kernel_POPULATED)
    FetchContent_Populate(freertos_kernel)
endif()

add_library(FreeRTOS INTERFACE
    ${freertos_kernel_SOURCE_DIR}/croutine.c
    ${freertos_kernel_SOURCE_DIR}/event_groups.c
    ${freertos_kernel_SOURCE_DIR}/list.c
    ${freertos_kernel_SOURCE_DIR}/queue.c
    ${freertos_kernel_SOURCE_DIR}/stream_buffer.c
    ${freertos_kernel_SOURCE_DIR}/tasks.c
    ${freertos_kernel_SOURCE_DIR}/timers.c
    ${freertos_kernel_SOURCE_DIR}/portable/ThirdParty/GCC/RP2040/port.c
    ${freertos_kernel_SOURCE_DIR}/portable/MemMang/heap_4.c
)
target_include_directories(FreeRTOS
    INTERFACE ${freertos_kernel_SOURCE_DIR}/include
    INTERFACE ${freertos_kernel_SOURCE_DIR}/portable/ThirdParty/GCC/RP2040/include
    INTERFACE include/ # For FreeRTOSConfig.h
)
target_link_libraries(FreeRTOS INTERFACE pico_stdlib pico_multicore hardware_exception hardware_clocks)
target_compile_definitions(FreeRTOS INTERFACE PICO_CONFIG_RTOS_ADAPTER_HEADER=${freertos_kernel_SOURCE_DIR}/portable/ThirdParty/GCC/RP2040/include/freertos_sdk_config.h)

add_library(rustcode STATIC IMPORTED)
add_dependencies(rustcode cargo-build)
set_target_properties(rustcode PROPERTIES IMPORTED_LOCATION_DEBUG ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-espidf-eabi/debug/librustcode.a)

add_library(pico-std-rust-c STATIC
    src/freertos_support.c
    src/shim.c
    
    esp-idf/src/pthread.c
    esp-idf/src/pthread_cond_var.c
    esp-idf/src/pthread_local_storage.c
    esp-idf/src/pthread_rwlock.c
    # Although ESP-IDF pthread has semaphore implementation, we use pico-sdk implementation instead.
    esp-idf/src/locks.c
    esp-idf/src/stdatomic.c
)
target_include_directories(pico-std-rust-c PUBLIC esp-idf/include)
target_link_libraries(pico-std-rust-c PUBLIC rustcode pico_stdlib FreeRTOS)
target_compile_definitions(pico-std-rust-c PUBLIC
    _POSIX_THREADS _POSIX_BARRIERS _POSIX_READER_WRITER_LOCKS _UNIX98_THREAD_MUTEX_ATTRIBUTES

    CONFIG_PTHREAD_STACK_MIN=256
    CONFIG_PTHREAD_TASK_CORE_DEFAULT=-1
    CONFIG_PTHREAD_TASK_STACK_SIZE_DEFAULT=1024
    CONFIG_PTHREAD_TASK_PRIO_DEFAULT=5
    CONFIG_PTHREAD_TASK_NAME_DEFAULT="task"
)

install()