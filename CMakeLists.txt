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

# Automatically download FreeRTOS+POSIX
FetchContent_Declare(
    freertos_posix
    GIT_REPOSITORY https://github.com/FreeRTOS/Lab-Project-FreeRTOS-POSIX.git
    GIT_TAG b62f30baf5a4ee08656381b31399ff3f6fc85467
)
FetchContent_GetProperties(freertos_posix)
if(NOT freertos_posix_POPULATED)
    FetchContent_Populate(freertos_posix)
endif()

add_library(FreeRTOS STATIC
    ${freertos_kernel_SOURCE_DIR}/croutine.c
    ${freertos_kernel_SOURCE_DIR}/event_groups.c
    ${freertos_kernel_SOURCE_DIR}/list.c
    ${freertos_kernel_SOURCE_DIR}/queue.c
    ${freertos_kernel_SOURCE_DIR}/stream_buffer.c
    ${freertos_kernel_SOURCE_DIR}/tasks.c
    ${freertos_kernel_SOURCE_DIR}/timers.c
    ${freertos_kernel_SOURCE_DIR}/portable/ThirdParty/GCC/RP2040/port.c
    ${freertos_kernel_SOURCE_DIR}/portable/MemMang/heap_4.c
    
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_clock.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_mqueue.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_pthread.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_pthread_barrier.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_pthread_cond.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_pthread_mutex.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_sched.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_semaphore.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_timer.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_unistd.c
    ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/source/FreeRTOS_POSIX_utils.c
)
target_include_directories(FreeRTOS
    PUBLIC ${freertos_kernel_SOURCE_DIR}/include
    PUBLIC ${freertos_kernel_SOURCE_DIR}/portable/ThirdParty/GCC/RP2040/include

    PUBLIC ${freertos_posix_SOURCE_DIR}/include
    PRIVATE ${freertos_posix_SOURCE_DIR}/include/private
    PUBLIC ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/include
    PUBLIC ${freertos_posix_SOURCE_DIR}/FreeRTOS-Plus-POSIX/include/portable

    PUBLIC include/ # For FreeRTOSConfig.h and FreeRTOS_POSIX_portable.h
)
target_link_libraries(FreeRTOS PUBLIC pico_stdlib pico_multicore hardware_exception)

add_library(rustcode STATIC IMPORTED)
add_dependencies(rustcode cargo-build)
set_target_properties(rustcode PROPERTIES IMPORTED_LOCATION_DEBUG ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-espidf-eabi/debug/librustcode.a)

add_executable(pico-std-rust src/freertos_support.c)
target_link_libraries(pico-std-rust PUBLIC rustcode pico_stdlib FreeRTOS)

#pico_add_extra_outputs(pico-std-rust)