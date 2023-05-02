# Using std Rust on Raspberry Pi Pico (RP2040)
## About
This is a proof-of-concept for using the Rust standard library (`std`) on RP2040.

It leverages ESP-IDF support of `std` and combines Pico SDK, FreeRTOS, and some code adopted from ESP-IDF.
Hence it creates a weird target triple called `thumbv6m-none-espidf-eabi`.

## Prerequisites
Only tested on Windows 11.
- Nightly Rust and Cargo
- `rust-src`
- GNU Arm Embedded Toolchain (which includes GCC and Newlib for `arm-none-eabi`)
- CMake
- Ninja (probably GNU Make will also work)

## Building and flashing
Two steps are needed for building.
### Step 1: Building Rust part
```
cargo build
```

It produces a static library `target/thumbv6m-none-espidf-eabi/librustcode.a`.

### Step 2: Building C part, including Pico SDK and FreeRTOS
```
mkdir build
cd build/
cmake ..
ninja
cd ..
```

It produces an ELF file `build/pico-std-rust.elf`.
Pico SDK and FreeRTOS source code are automatically downloaded by CMake.

### Step 3: Flashing binary on Raspberry Pi Pico
If you have a SWD debug probe, you can use `cargo flash` for flashing.
```
cargo flash --chip RP2040 --elf .\build\pico-std-rust.elf
```

Probably, converting ELF to UF2 and drag-and-drop flashing using BOOTSEL will also work.

## TODO
- [ ] Automatically invoke CMake from Cargo
- [ ] embedded-hal integration
- [ ] Networking support for Pico W

## License
Portions written by me are licensed under MIT or Apache 2.0 license.
However this repo also contains code from ESP-IDF under Apache 2.0 license only.