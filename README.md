# Using std Rust on Raspberry Pi Pico (RP2040)
## About
This is a proof-of-concept for using the Rust standard library (`std`) on RP2040.
You can use standard API such as `println!` or `thread::spawn`, as well as peripheral access using embedded-hal.

It leverages ESP-IDF support of `std` and combines Pico SDK, FreeRTOS, and some code adopted from ESP-IDF.
Hence it creates a weird target triple called `thumbv6m-none-espidf-eabi`.

## Prerequisites
Only tested on Windows 11.
- Nightly Rust and Cargo
- `rust-src`
- GNU Arm Embedded Toolchain (which includes GCC and Newlib for `arm-none-eabi`)
- CMake
- Ninja
- ldproxy (can be installed by `cargo install ldproxy`)

## Building and flashing
### Building
Only the following command line is required for build.
It produces an ELF file `pico-std-rust` in `target/thumbv6m-none-espidf-eabi/debug/` directory.
```
cargo build
```

Pico SDK and FreeRTOS source code are automatically downloaded during build.

### Step 3: Flashing binary on Raspberry Pi Pico
If you have a SWD debug probe, you can use `probe-rs-cli` for flashing.
```
probe-rs-cli download target/thumbv6m-none-espidf-eabi/debug/pico-std-rust --chip RP2040
```

Probably, converting ELF to UF2 and drag-and-drop flashing using BOOTSEL will also work.

## TODO
- [ ] Protect internally used peripherals from embedded-hal access
- [ ] Networking support for Pico W

## License
Portions written by me are licensed under MIT or Apache 2.0 license.
However this repo also contains code from ESP-IDF under Apache 2.0 license only.