# Configuration Demo - How Easy Timer Changes Really Are! 

This demonstrates just how simple it is to change the curing duration without touching any complex code.

## Before and After Comparison

### ❌ Old Way (Complex)
```rust
// Had to find this line buried in main.rs:
Timer::after(Duration::from_secs(10)).await;

// And also update the log message:  
info!("Relay CLOSED - UV LEDs ON - Curing for 10 seconds");

// And possibly other timing-related constants scattered around...
```

### ✅ New Way (Super Simple!)

Just edit **one line** in `src/config.rs`:

```rust
// Change this single line from:
pub const CURING_DURATION_SECONDS: u64 = 10;

// To whatever you want:
pub const CURING_DURATION_SECONDS: u64 = 30;  // 30-second cure
```

That's it! Run `cargo build --release` and flash - everything else updates automatically!

## Real Examples

### Quick Test (2 seconds)
```rust
pub const CURING_DURATION_SECONDS: u64 = 2;
```
Perfect for testing your hardware setup without waiting.

### Standard Resin (10 seconds) 
```rust  
pub const CURING_DURATION_SECONDS: u64 = 10;
```
Default setting - good for most standard resins.

### Tough Resin (45 seconds)
```rust
pub const CURING_DURATION_SECONDS: u64 = 45;  
```
For ABS-like or tough resins that need longer exposure.

### Full Post-Processing (2 minutes)
```rust
pub const CURING_DURATION_SECONDS: u64 = 120;
```
Complete post-processing cure for final strength.

## What Updates Automatically

When you change `CURING_DURATION_SECONDS`, these all update automatically:
- ✅ Actual curing timer 
- ✅ Startup message: "System ready - press button to start **X**-second curing cycle"
- ✅ Active curing message: "UV LEDs ON - Curing for **X** seconds"
- ✅ Compile-time validation (prevents crazy values like 0 or 1000+ seconds)

## Bonus: Use Presets

Instead of editing the duration number, you can uncomment preset sections:

```rust
/* QUICK TEST MODE - Fast cycles for testing hardware */
pub const CURING_DURATION_SECONDS: u64 = 2;
pub const COMPLETION_BEEPS: u32 = 1;
```

Or:

```rust  
/* DEEP CURE MODE - For thick or tough resins */
pub const CURING_DURATION_SECONDS: u64 = 30;
pub const COMPLETION_BEEPS: u32 = 5;  // More beeps for longer cure
```

## Safety Features Built-In

The configuration system has compile-time safety checks:
- ❌ Won't compile if duration is 0 seconds
- ❌ Won't compile if duration is over 10 minutes (600 seconds)  
- ❌ Won't compile if invalid beep counts
- ❌ Won't compile if debounce timing is unreasonable

This prevents accidentally setting dangerous or unusable values!

---

**That's the power of good configuration design** - change one number, everything else just works! 🎯