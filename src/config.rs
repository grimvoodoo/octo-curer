// Configuration Module for UV Resin Curing Controller
// 
// This file contains all user-configurable settings in one place.
// To change curing time or other settings, just modify the values here
// and rebuild the project - no need to edit the main program logic!

/* ===========================================
   🔧 USER CONFIGURABLE SETTINGS 
   =========================================== */

/// Main curing duration in seconds
/// 
/// Common resin curing times:
/// - Quick test: 5 seconds  
/// - Standard cure: 10 seconds
/// - Deep cure: 30 seconds
/// - Full cure: 60 seconds
/// - Extended cure: 120 seconds (2 minutes)
/// - Long cure: 300 seconds (5 minutes)
pub const CURING_DURATION_SECONDS: u64 = 300;

/// Button debounce delay in milliseconds
/// 
/// Prevents multiple triggers from a single button press
/// Increase if you experience double-triggering
pub const BUTTON_DEBOUNCE_MS: u64 = 50;

/// Relay settling time in milliseconds
/// 
/// Time to wait after turning off relay to ensure it fully opens
/// Increase if UV LEDs don't turn off reliably
pub const RELAY_SETTLE_TIME_MS: u64 = 500;

/// Completion buzzer beep settings
/// 
/// How many beeps to sound when curing is complete
pub const COMPLETION_BEEPS: u32 = 3;

/// Duration of each beep in milliseconds
pub const BEEP_DURATION_MS: u64 = 200;

/// Pause between beeps in milliseconds  
pub const BEEP_PAUSE_MS: u64 = 300;

/// Delay before accepting next button press
/// 
/// Prevents accidental immediate re-triggering after completion
pub const CYCLE_COOLDOWN_MS: u64 = 1000;

/* ===========================================
   🎯 PRESET CONFIGURATIONS
   =========================================== */

// Uncomment ONE of these sections to quickly switch between common configurations:

/* QUICK TEST MODE - Fast cycles for testing hardware
pub const CURING_DURATION_SECONDS: u64 = 2;
pub const COMPLETION_BEEPS: u32 = 1;
*/

/* STANDARD MODE - Typical resin curing (DEFAULT)
pub const CURING_DURATION_SECONDS: u64 = 10;
pub const COMPLETION_BEEPS: u32 = 3;
*/

/* DEEP CURE MODE - For thick or tough resins  
pub const CURING_DURATION_SECONDS: u64 = 30;
pub const COMPLETION_BEEPS: u32 = 5;
*/

/* PRODUCTION MODE - Long cure with minimal audio
pub const CURING_DURATION_SECONDS: u64 = 60;
pub const COMPLETION_BEEPS: u32 = 1;
pub const BEEP_DURATION_MS: u64 = 100;
*/

/* ===========================================
   📋 CONFIGURATION VALIDATION
   =========================================== */

// Compile-time checks to prevent invalid configurations
const _: () = {
    assert!(CURING_DURATION_SECONDS > 0, "Curing duration must be greater than 0 seconds");
    assert!(CURING_DURATION_SECONDS <= 600, "Curing duration should be 10 minutes or less for safety");
    assert!(COMPLETION_BEEPS > 0, "Must have at least 1 completion beep");
    assert!(COMPLETION_BEEPS <= 10, "Too many beeps could be annoying");
    assert!(BUTTON_DEBOUNCE_MS >= 10, "Debounce time too short, may cause double-triggers");
    assert!(BUTTON_DEBOUNCE_MS <= 500, "Debounce time too long, will feel unresponsive");
};

/* ===========================================
   💡 USAGE INSTRUCTIONS
   =========================================== */

/*
HOW TO CHANGE CURING TIME:

Method 1 - Quick Change:
1. Edit CURING_DURATION_SECONDS at the top of this file
2. Run: cargo build --release
3. Flash the new firmware to your Pico

Method 2 - Use Presets:
1. Comment out the current preset section
2. Uncomment your desired preset (QUICK TEST, DEEP CURE, etc.)
3. Build and flash

Method 3 - Multiple Duration Support:
For advanced users who want button-selectable durations, see the 
multi_duration_example.rs file for implementation ideas.

EXAMPLES:
- Change line 15 to: pub const CURING_DURATION_SECONDS: u64 = 30;  // 30-second cure
- Change line 15 to: pub const CURING_DURATION_SECONDS: u64 = 120; // 2-minute cure

SAFETY NOTES:
- Always test new timings with small samples first
- UV exposure can be harmful - ensure proper ventilation and eye protection
- Longer curing times generate more heat - monitor temperature
*/