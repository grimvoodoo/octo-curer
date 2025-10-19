#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Manual Relay Test - Press button to try different approaches");

    let mut button = Input::new(p.PIN_6, Pull::Up);
    let mut relay = Output::new(p.PIN_10, Level::High);  // HIGH = open
    let mut status_led = Output::new(p.PIN_25, Level::Low);

    let mut test_number = 1;

    loop {
        info!("Ready for test {}. Press button...", test_number);
        button.wait_for_falling_edge().await;
        Timer::after_millis(100).await; // debounce
        
        status_led.set_high();
        
        match test_number {
            1 => {
                info!("Test 1: Brief pulse (100ms)");
                relay.set_low();
                Timer::after_millis(100).await;
                relay.set_high();
            },
            2 => {
                info!("Test 2: Medium pulse (500ms)");
                relay.set_low();
                Timer::after_millis(500).await;
                relay.set_high();
            },
            3 => {
                info!("Test 3: Long pulse (1000ms)");
                relay.set_low();
                Timer::after_millis(1000).await;
                relay.set_high();
            },
            4 => {
                info!("Test 4: Multiple quick pulses");
                for _ in 0..5 {
                    relay.set_low();
                    Timer::after_millis(50).await;
                    relay.set_high();
                    Timer::after_millis(50).await;
                }
            },
            5 => {
                info!("Test 5: Very slow toggle");
                relay.set_low();
                Timer::after(Duration::from_secs(2)).await;
                relay.set_high();
                Timer::after(Duration::from_secs(2)).await;
            },
            _ => {
                test_number = 0; // Reset to test 1 next
                info!("All tests done, restarting...");
            }
        }
        
        status_led.set_low();
        test_number += 1;
        Timer::after_millis(1000).await;
    }
}