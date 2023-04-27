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
    # add_subdirectory(${freertos_kernel_SOURCE_DIR})
    # add_subdirectory(${freertos_kernel_SOURCE_DIR}/portable/ThirdParty/GCC/RP2040)
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
)
target_include_directories(FreeRTOS
    PUBLIC ${freertos_kernel_SOURCE_DIR}/include
    PUBLIC ${freertos_kernel_SOURCE_DIR}/portable/ThirdParty/GCC/RP2040/include
    PUBLIC include/ # For FreeRTOSConfig.h
)
target_link_libraries(FreeRTOS PUBLIC pico_stdlib pico_multicore hardware_exception)

add_library(rustcode STATIC IMPORTED)
add_dependencies(rustcode cargo-build)
set_target_properties(rustcode PROPERTIES IMPORTED_LOCATION_DEBUG ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-eabi/debug/librustcode.a)

add_executable(pico-std-rust)
target_include_directories(pico-std-rust PUBLIC include/)
target_link_libraries(pico-std-rust PUBLIC rustcode pico_stdlib FreeRTOS)

#pico_add_extra_outputs(pico-std-rust)