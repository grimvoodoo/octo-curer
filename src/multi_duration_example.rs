// Advanced Example: Multiple Duration Support
// 
// This example shows how to implement button-selectable curing durations.
// Users can cycle through different preset times before starting curing.
//
// NOTE: This is an EXAMPLE FILE for reference - not used by default.
//       To use this functionality, integrate the concepts into main.rs

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Flex, Input, Level, Output, Pin, Pull};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// Duration presets in seconds
const DURATION_PRESETS: [u64; 5] = [5, 10, 30, 60, 120]; // 5s, 10s, 30s, 1min, 2min
const PRESET_NAMES: [&str; 5] = ["Quick", "Standard", "Deep", "Full", "Extended"];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Multi-Duration UV Curing Controller Starting!");

    // Hardware setup (same as main.rs)
    let mut button = Input::new(p.PIN_6, Pull::Up);
    let mut buzzer = Output::new(p.PIN_7, Level::Low);
    let mut status_led = Output::new(p.PIN_25, Level::Low);
    let mut flex_pin = Flex::new(p.PIN_10.degrade());
    flex_pin.set_as_output();
    flex_pin.set_high();

    let mut selected_duration_index = 1; // Start with "Standard" (10 seconds)
    
    info!("Multi-duration mode ready! Press button to cycle durations, hold to start curing");
    info!("Current: {} ({} seconds)", 
          PRESET_NAMES[selected_duration_index], 
          DURATION_PRESETS[selected_duration_index]);

    loop {
        // Wait for button press
        button.wait_for_falling_edge().await;
        Timer::after_millis(50).await; // Debounce

        // Check if button is held down (long press = start curing)
        let mut hold_time = 0u32;
        while !button.is_high() {
            Timer::after_millis(50).await;
            hold_time += 50;
            
            // Visual feedback for long press
            if hold_time >= 500 && hold_time < 1000 {
                status_led.set_high(); // LED on during hold
            }
            
            // If held for 1 second, start curing with current duration
            if hold_time >= 1000 {
                let curing_duration = DURATION_PRESETS[selected_duration_index];
                let preset_name = PRESET_NAMES[selected_duration_index];
                
                info!("LONG PRESS DETECTED - Starting {} cure ({} seconds)", preset_name, curing_duration);
                
                // Start curing cycle
                flex_pin.set_as_output();
                flex_pin.set_low();
                status_led.set_high();
                info!("UV LEDs ON - {} cure in progress...", preset_name);
                
                // Curing timer with selected duration
                Timer::after(Duration::from_secs(curing_duration)).await;
                
                // Turn off UV LEDs
                flex_pin.set_as_input();
                status_led.set_low();
                Timer::after_millis(500).await;
                
                info!("Curing complete! {} seconds {} cure finished", curing_duration, preset_name);
                
                // Success beeps (more beeps for longer cures)
                let beep_count = match curing_duration {
                    5 => 1,
                    10 => 2, 
                    30 => 3,
                    60 => 4,
                    _ => 5,
                };
                
                for i in 1..=beep_count {
                    info!("Completion beep {}/{}", i, beep_count);
                    buzzer.set_high();
                    Timer::after_millis(200).await;
                    buzzer.set_low();
                    Timer::after_millis(300).await;
                }
                
                Timer::after_millis(1000).await;
                break; // Exit hold detection loop
            }
        }
        
        // If button released before 1 second = short press = cycle duration
        if hold_time < 1000 {
            status_led.set_low(); // Turn off LED
            selected_duration_index = (selected_duration_index + 1) % DURATION_PRESETS.len();
            
            info!("Duration changed: {} ({} seconds)", 
                  PRESET_NAMES[selected_duration_index], 
                  DURATION_PRESETS[selected_duration_index]);
            
            // Audio feedback for duration change
            buzzer.set_high();
            Timer::after_millis(100).await;
            buzzer.set_low();
            Timer::after_millis(100).await;
            
            // Quick LED blinks to show selected duration
            for _ in 0..=selected_duration_index {
                status_led.set_high();
                Timer::after_millis(150).await;
                status_led.set_low();
                Timer::after_millis(150).await;
            }
        }
    }
}

/*
USAGE INSTRUCTIONS FOR MULTI-DURATION MODE:

1. Press button quickly (< 1 second): Cycle through duration presets
   - LED will blink N times to show preset number (1-5 blinks)
   - Buzzer gives short beep for audio feedback
   
2. Hold button (> 1 second): Start curing with current preset
   - LED turns on solid during hold to show "ready to start"
   - Release after 1+ seconds to begin curing
   
3. During curing:
   - LED stays on solid
   - Automatic shutoff after preset time
   - Multiple beeps when complete (more beeps = longer cure)

PRESETS:
1. Quick (5s)    - 1 LED blink, 1 completion beep
2. Standard (10s) - 2 LED blinks, 2 completion beeps  
3. Deep (30s)    - 3 LED blinks, 3 completion beeps
4. Full (60s)    - 4 LED blinks, 4 completion beeps
5. Extended (120s) - 5 LED blinks, 5 completion beeps

TO IMPLEMENT:
Copy the relevant parts of this code into main.rs, or replace main.rs 
with this file (rename it to main.rs).
*/