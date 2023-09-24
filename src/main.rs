use std::{thread, time::Duration, sync::{Mutex, Arc}};
use rp2040_hal as hal;
use embedded_hal::digital::v2::OutputPin;

mod startup;
mod peripherals;

extern "C" {
    fn stdio_init_all() -> bool;
}

fn main() {
    let mut peripherals = peripherals::Peripherals::take().unwrap();
    let sio = peripherals::Sio::take().unwrap();
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