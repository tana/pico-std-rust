/// Concurrently run two threads.
/// The main thread writes messages to the stdout (default is UART0) every second.
/// The another thread writes messages every 3 seconds.
/// Both threads share a counter.

use std::{thread, time::Duration, sync::{Mutex, Arc}};
use pico_std as _;   // pico_std has to always be imported, even if no items of it is used.

fn main() {
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