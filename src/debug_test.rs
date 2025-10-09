#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_rp::gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    defmt::info!("Debug test starting - this should be visible via debug probe");
    
    // GPIO25 is the onboard LED
    let mut led = Output::new(p.PIN_25, Level::Low);
    
    // Simple delay function
    let mut delay_counter = 0u32;
    
    loop {
        defmt::info!("LED on - Debug should work now!");
        led.set_high();
        
        // Simple delay loop
        for _ in 0..1_000_000 {
            delay_counter = delay_counter.wrapping_add(1);
        }
        
        defmt::info!("LED off");  
        led.set_low();
        
        // Simple delay loop  
        for _ in 0..1_000_000 {
            delay_counter = delay_counter.wrapping_add(1);
        }
    }
}
