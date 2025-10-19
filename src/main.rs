// These attributes tell Rust we're writing embedded code without the standard library
#![no_std]   // Don't use the standard library (not available on microcontrollers)
#![no_main]  // We'll define our own main function instead of using Rust's default

// Import necessary modules and functions
// 'use' statements are like 'import' in Python or '#include' in C++
use defmt::*;  // Import logging/debugging functions (like println! but for embedded)
use embassy_executor::Spawner;  // Embassy's async task spawner
use embassy_rp::gpio::{Flex, Input, Level, Output, Pin, Pull};  // GPIO pin types and functions
use embassy_time::{Duration, Timer};  // Time-related functions for delays
use {defmt_rtt as _, panic_probe as _};  // Debugging tools for development

// Import our configuration module - all timing settings are in config.rs
mod config;
use config::*;

// This attribute marks our main function for Embassy's async executor
// Embassy is an async framework for embedded Rust - it handles timing and concurrency
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the RP2040 hardware with default settings
    // 'let' creates a new variable, 'p' contains all the GPIO pins
    let p = embassy_rp::init(Default::default());
    
    // 'info!' is like println! but optimized for embedded systems
    info!("UV Resin Curing Timer Starting!");

    /* GPIO PIN SETUP SECTION */
    // Create GPIO pins for our hardware connections
    // 'mut' means the variable can be modified (mutable)
    
    // Button input with internal pull-up resistor (pressed = LOW, released = HIGH)
    let mut button = Input::new(p.PIN_6, Pull::Up);
    
    // Buzzer output pin (starts LOW = off)
    let mut buzzer = Output::new(p.PIN_7, Level::Low);
    
    // Onboard LED for status indication (starts LOW = off)
    let mut status_led = Output::new(p.PIN_25, Level::Low);

    info!("System ready - press button to start {}-second curing cycle", CURING_DURATION_SECONDS);
    
    /* RELAY CONTROL PIN SETUP */
    // FlexPin can switch between input/output modes - crucial for relay reset
    // The SRD-05VDC-SL-C relay module needs this special handling
    let mut flex_pin = Flex::new(p.PIN_10.degrade());
    
    /* STARTUP RELAY RESET - CRITICAL FOR PREVENTING INITIAL ACTIVATION */
    // When Pico powers on, GPIO pins can be in undefined states
    // This ensures the relay is definitely OFF at startup
    info!("Performing startup relay reset to ensure LEDs are OFF...");
    flex_pin.set_as_input();       // First set to high-impedance (guaranteed OFF)
    Timer::after_millis(RELAY_SETTLE_TIME_MS).await;  // Wait for relay to settle
    flex_pin.set_as_output();      // Then set as output for control
    flex_pin.set_high();           // HIGH = relay open (UV LEDs off)
    info!("Relay reset complete - LEDs confirmed OFF");
    
    /* MAIN PROGRAM LOOP */
    // In Rust, 'loop' creates an infinite loop - like 'while True:' in Python
    loop {
        /* STEP 1: WAIT FOR USER INPUT */
        // 'await' keyword pauses execution until the button is pressed
        // This is non-blocking - the CPU can do other things while waiting
        button.wait_for_falling_edge().await;  // Wait for button press (HIGH to LOW)
        info!("Button pressed! Starting curing cycle...");
        
        /* STEP 2: DEBOUNCE THE BUTTON */
        // Physical buttons can "bounce" - send multiple signals when pressed once
        // This delay prevents multiple triggers from a single press (configurable in config.rs)
        Timer::after_millis(BUTTON_DEBOUNCE_MS).await;
        
        /* STEP 3: ACTIVATE UV LEDS */
        // Set the relay pin to output mode and pull it LOW
        // SRD-05VDC-SL-C relay: LOW = closed = UV LEDs ON
        flex_pin.set_as_output();     // Ensure pin is in output mode
        flex_pin.set_low();           // Close relay (activate UV LEDs)
        status_led.set_high();        // Turn on internal LED for visual feedback
        info!("Relay CLOSED - UV LEDs ON - Curing for {} seconds", CURING_DURATION_SECONDS);
        
        /* STEP 4: CURING TIMER */
        // Wait for the configured duration while UV LEDs cure the resin
        // Duration is configurable in config.rs - change CURING_DURATION_SECONDS
        Timer::after(Duration::from_secs(CURING_DURATION_SECONDS)).await;
        
        /* STEP 5: TURN OFF UV LEDS (CRITICAL SECTION) */
        // This is the key discovery: setting pin to INPUT mode (high-impedance)
        // completely "kills" the pin, forcing the relay to open reliably
        flex_pin.set_as_input();      // High-impedance = no voltage = relay opens
        status_led.set_low();         // Turn off internal LED
        Timer::after_millis(RELAY_SETTLE_TIME_MS).await;  // Allow relay time to settle
        
        info!("Curing complete! UV LEDs OFF - Sounding completion buzzer...");
        
        /* STEP 6: COMPLETION NOTIFICATION */
        // Loop for configured number of beeps (configurable in config.rs)
        for i in 1..=COMPLETION_BEEPS {
            info!("Buzzer beep {}/{}", i, COMPLETION_BEEPS);    // Log which beep we're on
            buzzer.set_high();            // Turn buzzer ON
            Timer::after_millis(BEEP_DURATION_MS).await;  // Configurable beep duration
            buzzer.set_low();             // Turn buzzer OFF
            Timer::after_millis(BEEP_PAUSE_MS).await;     // Configurable pause between beeps
        }
        
        info!("Curing cycle complete! Ready for next cycle.");
        
        /* STEP 7: PREPARE FOR NEXT CYCLE */
        // Brief pause before accepting the next button press
        // Prevents accidental immediate re-triggering (configurable in config.rs)
        Timer::after_millis(CYCLE_COOLDOWN_MS).await;
        
    } // End of loop - jumps back to the beginning to wait for next button press
} // End of main function
