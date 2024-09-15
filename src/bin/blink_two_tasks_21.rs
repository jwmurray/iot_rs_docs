//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

type LedType = Mutex<ThreadModeRawMutex, Option<Output<'static>>>; // static lifetime is needed for the mutex
static LED: LedType = Mutex::new(None); // Create a static mutex to protect the LED

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let led = Output::new(p.PIN_21, Level::Low);

    // inner scope is so that once the mutex is written to, the MutexGuard is dropped, thus the
    // Mutex is released.  Without this code, the LED would be locked forever.
    {
        *(LED.lock().await) = Some(led); // Simply exerise the mutex lock to set the LED
    } // MutexGuard is dropped here

    let dt = 100 * 1_000_000;
    let k = 1.003;

    // Pass the LED mutex to the spawned tasks.  Each can now now request a lock on the LED.
    unwrap!(spawner.spawn(toggle_led(&LED, Duration::from_nanos(dt), "Fast thread")));
    unwrap!(spawner.spawn(toggle_led(
        &LED,
        Duration::from_nanos((dt as f64 * k) as u64),
        "Slow thread"
    )));
}

#[embassy_executor::task(pool_size = 2)]
async fn toggle_led(led: &'static LedType, delay: Duration, name: &'static str) {
    let mut ticker = Ticker::every(delay);
    info!("Starting {} with delay {:?}", name, delay);
    loop {
        {
            let mut led_unlocked = led.lock().await;
            if let Some(pin_ref) = led_unlocked.as_mut() {
                pin_ref.toggle();
            }
        }
        ticker.next().await;
    }
}
