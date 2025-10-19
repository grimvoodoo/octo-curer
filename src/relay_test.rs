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
    info!("Simple Relay Test Starting!");

    // GPIO pin setup
    let mut button = Input::new(p.PIN_6, Pull::Up);
    let mut relay = Output::new(p.PIN_10, Level::High);  // SRD-05VDC-SL-C relay (HIGH = open)
    let mut status_led = Output::new(p.PIN_25, Level::Low);

    info!("System ready - press button for simple relay test");
    
    loop {
        // Wait for button press
        button.wait_for_falling_edge().await;
        info!("Button pressed! Testing relay...");
        
        // Debounce
        Timer::after_millis(50).await;
        
        // Close relay and turn on LED
        status_led.set_high();
        relay.set_low();   // LOW closes the relay
        info!("Relay should be CLOSED now");
        
        // Wait 3 seconds (shorter for testing)
        Timer::after(Duration::from_secs(3)).await;
        
        // Open relay and turn off LED  
        relay.set_high();  // HIGH opens the relay
        status_led.set_low();
        info!("Relay should be OPEN now");
        
        // Wait before next test
        Timer::after(Duration::from_secs(2)).await;
        info!("Ready for next test");
    }
}