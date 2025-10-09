# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Raspberry Pi Pico embedded Rust project using Embassy for UV resin curing. The system controls UV LEDs via a relay for a timed 5-minute curing cycle, triggered by a button press with buzzer notification when complete.

### Hardware Setup
- **Target**: Raspberry Pi Pico (RP2040) - **MUST be revision B2 or newer**
- **Debugger**: Raspberry Pi Pico with picoprobe firmware (connected via USB) - **MUST be revision B2 or newer**
- **Button**: Connected to GPIO6 (input with pull-up)
- **Relay**: Connected to GPIO10 (output, normally open → closed to activate 12V UV LEDs)
- **Buzzer**: Connected to GPIO7 (output for completion notification)
- **UV LEDs**: External 12V power supply, controlled via relay

⚠️ **IMPORTANT**: Older Raspberry Pi Pico revisions (B0, B1) have hardware bugs that prevent reliable SWD debugging. Both the picoprobe and target Pico must be revision B2 or newer for debug connections to work properly.

### Operation
1. Press button on GPIO6 to start curing cycle
2. Relay on GPIO10 closes, activating UV LEDs for 5 minutes
3. After 5 minutes, relay opens (LEDs off) and buzzer on GPIO7 sounds three short rings

## Development Commands

### Building
```bash
# Build for Raspberry Pi Pico (RP2040)
cargo build --release

# Build with size optimizations (important for embedded)
cargo build --release
```

### Flashing and Running
```bash
# Flash to Pico via picoprobe (recommended)
cargo flash --chip RP2040 --release

# Flash and attach to RTT logs (may hang waiting for RTT)
cargo run --release

# Reset the target device
probe-rs reset --chip RP2040
```

### Debugging
```bash
# Start GDB debugging session
cargo embed --release

# View logs/RTT output
probe-rs log --chip RP2040

# Get device info
probe-rs info
```

### Testing
```bash
# Run unit tests (host target)
cargo test --lib

# Run tests with output
cargo test --lib -- --nocapture

# Run specific test module
cargo test --lib <module_name>

# Note: Integration tests require hardware or simulation
```

### Code Quality
```bash
# Check code for RP2040 target
cargo check --target thumbv6m-none-eabi

# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --check

# Run clippy for embedded target
cargo clippy --target thumbv6m-none-eabi

# Fix clippy warnings automatically where possible
cargo clippy --target thumbv6m-none-eabi --fix
```

### Documentation
```bash
# Generate documentation for embedded target
cargo doc --target thumbv6m-none-eabi --open

# Generate docs without opening
cargo doc --target thumbv6m-none-eabi
```

### Dependency Management
```bash
# Add embassy dependencies
cargo add embassy-executor --features nightly
cargo add embassy-time --features nightly
cargo add embassy-rp --features nightly

# Add development dependencies
cargo add --dev <crate_name>

# Update dependencies
cargo update
```

## Architecture Notes

### Current Structure
- **Embedded binary**: Single binary crate targeting RP2040 microcontroller
- **Embassy framework**: Async embedded framework for Rust
- **Target**: `thumbv6m-none-eabi` (Cortex-M0+ architecture)
- **Edition 2021**: Standard for embedded projects (2024 not yet stable for embedded)

### Key Components
- **GPIO Management**: Button input (GPIO6), relay output (GPIO10), buzzer output (GPIO7)
- **Timer System**: 5-minute curing cycle using Embassy's async timers
- **Interrupt Handling**: Button press detection with debouncing
- **State Machine**: Idle → Curing → Complete → Idle cycle

### Module Organization
```
src/
├── main.rs          # Main application and Embassy executor
├── hardware.rs      # GPIO pin definitions and hardware abstractions
├── curing.rs        # Curing cycle state machine and logic
├── buzzer.rs        # Buzzer patterns and control
└── lib.rs           # Common types and utilities
```

### Embassy Task Structure
- **Main Task**: Handles button input and coordinates curing cycles
- **Curing Task**: Manages 5-minute timer and relay control
- **Buzzer Task**: Handles completion notification patterns

## Development Guidelines

### Required Dependencies
```toml
[dependencies]
embassy-executor = { version = "0.5", features = ["arch-cortex-m", "executor-thread", "integrated-timers"] }
embassy-time = { version = "0.3", features = ["defmt", "defmt-timestamp-uptime"] }
embassy-rp = { version = "0.1", features = ["defmt", "unstable-pac", "time-driver", "critical-section-impl"] }
defmt = "0.3"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }
cortex-m = { version = "0.7.6", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7.0"
```

### Cargo Configuration
Ensure `.cargo/config.toml` is set up for RP2040:
```toml
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
runner = "probe-rs run --chip RP2040"
rustflags = [
  "-C", "linker=flip-link",
  "-C", "link-arg=--nmagic",
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=-Tdefmt.x",
]
```

### GPIO Pin Assignments
- **GPIO6**: Button input (with internal pull-up)
- **GPIO7**: Buzzer output (active high)
- **GPIO10**: Relay control output (active high for closed relay)

### Error Handling
- Use `defmt` for logging instead of `println!`
- Embassy's `Result` types for async operations
- Hardware failures should trigger safe shutdown (relay off)
- Button debouncing to prevent multiple trigger events

### Testing Strategy
- Unit tests for state machine logic (host target)
- Hardware-in-the-loop testing with actual Pico
- Verify timing accuracy with oscilloscope/logic analyzer
- Test button debouncing with rapid presses

### Safety Considerations
- Always turn off relay on panic or unexpected reset
- Implement watchdog timer for fail-safe operation
- Maximum curing time limit (fail-safe if timer fails)
- Button debouncing to prevent accidental multiple starts
- Visual/audio feedback for all state transitions

### Development Setup
1. **Verify hardware versions**: Both picoprobe and target must be Pico revision B2 or newer
2. Install probe-rs: `cargo install probe-rs --features cli`
3. Connect picoprobe to target Pico via SWD and UART pins:
   - picoprobe Pin 4 (GPIO2) → target Pin 31 (SWCLK)
   - picoprobe Pin 5 (GPIO3) → target Pin 32 (SWDIO)
   - picoprobe Pin 6 (GPIO4) → target Pin 30 (RUN/RESET)
   - picoprobe Pin 7 (GPIO5) → target Pin 1 (GPIO0/UART0_TX)
   - picoprobe Pin 9 (GPIO6) → target Pin 2 (GPIO1/UART0_RX)
   - picoprobe VSYS → target VSYS (power)
   - picoprobe GND → target GND
4. Verify connection: `probe-rs info`
5. Flash and run: `cargo run --release`

### Troubleshooting Debug Connection
- **"Target device did not respond"**: Usually indicates older Pico revision (pre-B2)
- **"Cannot read IDR"**: Hardware version issue or loose SWD connections
- **"Probe not found"**: Check udev rules and USB permissions
