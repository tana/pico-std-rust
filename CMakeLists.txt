cmake_minimum_required(VERSION 3.19)

include(FetchContent)

set(Rust_CARGO_TARGET "thumbv6m-none-eabi")

# Automatically download Pico SDK
set(PICO_SDK_FETCH_FROM_GIT on)
include(pico_sdk_import.cmake)

# Automatically download Corrosion which allows using Cargo from CMake
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3.5
)
FetchContent_MakeAvailable(Corrosion)

project(pico-std-rust)

pico_sdk_init()

corrosion_import_crate(
    MANIFEST_PATH Cargo.toml
    NO_STD
)

add_executable(pico-std-rust)
target_link_libraries(pico-std-rust pico_stdlib rustcode)

#pico_add_extra_outputs(pico-std-rust)