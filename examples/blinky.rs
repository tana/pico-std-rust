/// Make the built-in LED (connected to GPIO 25) on a Raspberry Pi Pico board blink at 1 Hz

use std::{time::Duration, thread};
use rp2040_hal as hal;
use embedded_hal::digital::v2::OutputPin;
use pico_std::peripherals;

fn main() {
    let mut peripherals = peripherals::Peripherals::take().unwrap();
    let sio = peripherals::Sio::take().unwrap();
    let pins = hal::gpio::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS
    );

    let mut pin = pins.gpio25.into_push_pull_output();

    loop {
        pin.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));

        pin.set_low().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}
