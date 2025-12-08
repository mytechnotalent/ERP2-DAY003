![image](https://github.com/mytechnotalent/ERP2-DAY003/blob/main/DAY003.png?raw=true)

## FREE Reverse Engineering Self-Study Course [HERE](https://github.com/mytechnotalent/Reverse-Engineering-Tutorial)
VIDEO PROMO [HERE](https://www.youtube.com/watch?v=aD7X9sXirF8)

<br>

# ⭐ DAY003: Traffic Light Sim

<br>

**Difficulty**: Beginner  
**Date**: Day 3 of 365  
**Components**: 3 LEDs (Red, Yellow, Green), 3 Resistors  
**Concepts**: GPIO, Digital Output, State Machine, Async Programming

<br>

# 🔋 Schematic
![image](https://github.com/mytechnotalent/Embedded-Hacking/blob/main/EHP2_bb.png?raw=true)

<br>

# 📋 Project Overview
This is the third project in the **365 Pico2 RP2350 Project Ideas** series. Building on Day 2's sequential LED blinking, we now simulate a real traffic light with proper state transitions. This project introduces you to state machine design patterns and timed state transitions with Embassy Rust.

## What You'll Learn
- Implementing state machine patterns in embedded systems
- Creating realistic traffic light timing sequences
- Managing state transitions with proper timing
- Building reusable controller abstractions
- Understanding state-based control logic
- Using Embassy's async/await for timed state changes

<br>

# 🧩 Hardware
## Raspberry Pi Pico 2 w/ Header [BUY](https://www.pishop.us/product/raspberry-pi-pico-2-with-header)
## USB A-Male to USB Micro-B Cable [BUY](https://www.pishop.us/product/usb-a-male-to-usb-micro-b-cable-6-inches)
## Raspberry Pi Pico Debug Probe [BUY](https://www.pishop.us/product/raspberry-pi-debug-probe)
## Complete Component Kit for Raspberry Pi [BUY](https://www.pishop.us/product/complete-component-kit-for-raspberry-pi)
## 10pc 25v 1000uF Capacitor [BUY](https://www.amazon.com/Cionyce-Capacitor-Electrolytic-CapacitorsMicrowave/dp/B0B63CCQ2N?th=1)
### 10% PiShop DISCOUNT CODE - KVPE_HS320548_10PC

<br>

# 🔌 Hardware Requirements

## Components Needed
| Quantity | Component                    | Notes                      |
| -------- | ---------------------------- | -------------------------- |
| 1        | Raspberry Pi Pico 2 (RP2350) |                            |
| 1        | Red LED                      | Stop signal                |
| 1        | Yellow LED                   | Caution signal             |
| 1        | Green LED                    | Go signal                  |
| 3        | 100Ω Resistors               | Current-limiting resistors |
| 1        | Breadboard                   | For prototyping            |
| 8        | Jumper Wires                 | Male-to-Male               |

## Wiring Diagram
```
┌─────────────────────────┐
│  Raspberry Pi Pico 2    │
│                         │
│  ┌────────┐             │
│  │  GP16  ├─────┐       │  RED LED
│  └────────┘     │       │
│  ┌────────┐     │       │
│  │  GP17  ├─────┼───┐   │  YELLOW LED
│  └────────┘     │   │   │
│  ┌────────┐     │   │   │
│  │  GP18  ├─────┼───┼───┐  GREEN LED
│  └────────┘     │   │   │
│                 │   │   │
│  ┌────────┐     │   │   │
│  │  GND   ├──┐  │   │   │
│  └────────┘  │  │   │   │
└──────────────┼──┼───┼───┘
               │  │   │   │
               │  └[100Ω]─[RED]───┐
               │      │           │
               │  └[100Ω]─[YELLOW]┤
               │      │           │
               │  └[100Ω]─[GREEN]─┤
               │                  │
               └──────────────────┘
                     GND

Connection Steps:
1. GP16 (Pin 21) → 100Ω Resistor → RED LED Anode (+)
2. GP17 (Pin 22) → 100Ω Resistor → YELLOW LED Anode (+)
3. GP18 (Pin 24) → 100Ω Resistor → GREEN LED Anode (+)
4. All LED Cathodes (-) → GND
```

## Pin Information
- **GP16**: Red LED (Stop signal)
- **GP17**: Yellow LED (Caution signal)
- **GP18**: Green LED (Go signal)
- **GND**: Ground connection (any GND pin works)

<br>

# 🛠️ Software Requirements

## Prerequisites
1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **ARM Cortex-M Target**
   ```bash
   rustup target add thumbv8m.main-none-eabihf
   ```
3. **probe-rs** (for flashing and debugging)
   ```bash
   cargo install probe-rs-tools --locked
   ```
4. **flip-link** (stack overflow protection)
   ```bash
   cargo install flip-link
   ```

## Dependencies
All dependencies are specified in `Cargo.toml`:
- **embassy-executor**: Async task executor for embedded systems (git version)
- **embassy-time**: Time and timer abstractions (git version)
- **embassy-rp**: Hardware Abstraction Layer (HAL) for RP2350 with `rp235xa` chip feature (git version for full RP2350 support)
- **cortex-m**: Low-level Cortex-M utilities
- **panic-halt**: Panic handler for embedded systems

> **Important Note**: We're using git versions of the Embassy framework because the crates.io releases don't yet have full RP2350 support. The RP2350 uses ARMv8-M architecture with different MPU registers than earlier chips. We specifically enable the `rp235xa` feature for Pico 2 (RP2350-A revision) and `critical-section-impl` for proper interrupt handling.

<br>

# 📝 Code Structure

## Project Files
```
DAY003/
├── Cargo.toml           # Project dependencies and configuration
├── build.rs             # Build script for linker configuration
├── memory.x             # Memory layout for RP2350
├── Makefile             # Build and test automation
├── .cargo/
│   └── config.toml      # Target and runner configuration
├── src/
│   ├── main.rs          # Main application code
│   ├── lib.rs           # Library module exports
│   ├── config.rs        # Configuration constants
│   └── traffic_light.rs # Traffic light controller
└── README.md            # This file
```

## Key Code Section

### 1. Main Function (`main.rs`)
```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut red = Output::new(p.PIN_16, Level::Low);
    let mut yellow = Output::new(p.PIN_17, Level::Low);
    let mut green = Output::new(p.PIN_18, Level::Low);
    let mut controller = TrafficLightController::new();
    loop {
        set_led(&mut red, controller.is_red());
        set_led(&mut yellow, controller.is_yellow());
        set_led(&mut green, controller.is_green());
        Timer::after_millis(controller.current_duration()).await;
        controller.advance();
    }
}
```

<br>

# 🚀 Building, Flashing and Testing

## Step 1: Connect the Pico 2
1. Hold the **BOOTSEL** button on the Pico 2
2. Connect the Pico to your computer via USB
3. Release the BOOTSEL button
4. The Pico appears as a USB drive (RPI-RP2)

## Step 2: Build the Project
```bash
cd DAY003
cargo build --release
```
This compiles the code for the RP2350 target.

## Step 3: Flash and Run
```bash
cargo run --release
```
This will:
1. Compile the code
2. Flash it to the Pico 2 using probe-rs
3. Start the traffic light simulation

### Expected Output
The traffic light follows a realistic sequence:
- **RED** stays on for 3 seconds (Stop)
- **GREEN** stays on for 3 seconds (Go)
- **YELLOW** stays on for 1 second (Caution)
- Cycle repeats continuously

## Step 4: Test
```bash
make test
```
This will run all of the unittests to ensure our project is functioning correctly and that recent changes haven't introduced regressions.

<br>

# 🔧 Troubleshooting

## Issue: `probe-rs` not found
**Solution**: Install probe-rs tools
```bash
cargo install probe-rs-tools --locked
```

## Issue: Can't find device
**Solution**: 
1. Ensure BOOTSEL was pressed during connection
2. Try a different USB cable (some are power-only)
3. Check USB permissions on Linux:
   ```bash
   sudo usermod -a -G plugdev $USER
   ```

## Issue: Build errors about missing target
**Solution**: Add the ARM target
```bash
rustup target add thumbv8m.main-none-eabihf
```

## Issue: LEDs don't light up
**Solutions**:
1. Check the wiring (resistors and polarity)
2. Verify you're using the correct GPIO pins (16, 17, 18)
3. Check if the LEDs are functional (test with a battery)
4. Ensure proper ground connection

## Issue: Wrong LED colors
**Solutions**:
1. Verify Red is on GP16, Yellow on GP17, Green on GP18
2. Check LED orientation (longer leg is anode/+)
3. Trace each wire connection

## Issue: Linker errors
**Solution**: Install flip-link
```bash
cargo install flip-link
```

<br>

# 📚 Understanding the Code

## State Machine Pattern
The traffic light uses a simple state machine with three states:
```rust
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}
```
Each state has a defined duration and transition to the next state.

## State Transitions
The sequence follows real traffic light behavior:
```rust
TrafficLightState::Red => TrafficLightState::Green,
TrafficLightState::Green => TrafficLightState::Yellow,
TrafficLightState::Yellow => TrafficLightState::Red,
```
This creates the pattern: Red → Green → Yellow → Red (repeat).

## GPIO Control
Each GPIO pin is configured as an output:
```rust
let mut red = Output::new(p.PIN_16, Level::Low);
```
- `Output::new()`: Configures the pin as a digital output
- `Level::Low`: Initial state is OFF (0V)

## Timing
Each state has its own duration:
```rust
pub const RED_DURATION_MS: u64 = 3000;    // 3 seconds
pub const YELLOW_DURATION_MS: u64 = 1000; // 1 second
pub const GREEN_DURATION_MS: u64 = 3000;  // 3 seconds
```

<br>

# 🎯 Experiments and Modifications

## 1. Change Light Durations
Modify the duration values in `config.rs`:
```rust
// Longer red light (5 seconds)
pub const RED_DURATION_MS: u64 = 5000;

// Shorter yellow light (500ms)
pub const YELLOW_DURATION_MS: u64 = 500;
```

## 2. Add a Blinking Yellow Phase
Create a warning mode with blinking yellow:
```rust
// Add new state variant
FlashingYellow,

// Implement 500ms on/off cycle
```

## 3. Pedestrian Crossing Mode
Add a button to trigger pedestrian crossing:
```rust
// Read button input
// Extend red duration when button pressed
```

## 4. Night Mode
Implement a flashing yellow night mode:
```rust
// After 10 cycles, switch to flashing yellow
// Flash at 500ms intervals
```

<br>

# 🧪 Challenges
1. **Countdown Display**: Add an LED countdown for each phase
2. **Pedestrian Signal**: Add walk/don't walk LEDs
3. **Emergency Mode**: Implement flashing red for emergency
4. **Dual Intersection**: Control two traffic lights in coordination

<br>

# 🔗 Resources

## Official Documentation
- [Embassy Book](https://embassy.dev/book/)
- [embassy-rp Documentation](https://docs.embassy.dev/embassy-rp/)
- [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
- [Pico 2 Documentation](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html)

## Rust Embedded
- [Embedded Rust Book](https://rust-embedded.github.io/book/)
- [probe-rs Documentation](https://probe.rs/)

## Community
- [Embassy GitHub](https://github.com/embassy-rs/embassy)
- [Rust Embedded Matrix Chat](https://matrix.to/#/#rust-embedded:matrix.org)

<br>

# ✅ Completion Checklist
- [ ] Hardware assembled correctly (3 LEDs with resistors)
- [ ] Rust toolchain installed
- [ ] probe-rs and flip-link installed
- [ ] Project builds without errors
- [ ] Traffic light cycles correctly (Red → Green → Yellow)
- [ ] Timing is correct (3s Red, 3s Green, 1s Yellow)
- [ ] Experimented with different durations
- [ ] Understood the state machine pattern
- [ ] Ready to move to DAY004

<br>

**Congratulations!** 🎉 You've completed the third Embassy Rust project on the Pico 2. You now understand how to implement state machines for embedded control systems. These skills will be essential for many future projects including reaction games, menu systems, and complex control logic.

<br>

# ⭐ DAY002: Blink LEDs in Sequence [HERE](https://github.com/mytechnotalent/ERP2-DAY002)

<br>

# 📄 License
MIT License - Copyright (c) 2025
