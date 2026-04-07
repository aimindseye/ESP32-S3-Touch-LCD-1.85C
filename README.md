# ESP32-S3-Touch-LCD-1.85C 

A repository for developing bare-metal Rust (`esp-hal`) applications on the [Waveshare ESP32-S3-Touch-LCD-1.85C](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.85C) circular display module.

The official documentation for this board relies heavily on C++ and the ESP-IDF framework. This repository exists to provide robust, asynchronous, and memory-safe **Rust** implementations for onboard hardware, while also preserving a practical **Arduino reference/demo path** for board bring-up and comparison.

---

## ⚠️ CAUTION & DISCLAIMER ⚠️

**USE AT YOUR OWN RISK.**  
This repository contains low-level hardware initialization code, firmware flashing instructions, reset sequences, and direct access to board peripherals.

- Incorrectly driving certain pins can lock up internal chips or peripherals.
- Sending malformed I2C commands to the expander / power-management path can leave the display or touch controller in a bad state.
- Misconfiguring reset or boot pins can make flashing appear broken until the device is manually recovered.
- **The maintainers are not responsible for bricked devices, damaged screens, or fried silicon.**

Always keep a backup of the original factory firmware so you can recover with `esptool` if needed.

---

## Repository Goals

This repository has two complementary goals:

1. Build a **proper Rust ecosystem** for the ESP32-S3 Touch LCD 1.85C
2. Keep a **working Arduino reference/demo path** for validation, diagnostics, and hardware comparison

That means this repo is not Arduino-only and not Rust-only. It is structured so hardware discoveries and board bring-up work can feed the Rust effort.

---

## 🚀 Quick Start: Environment Setup

Because the ESP32-S3 uses the Xtensa architecture, Rust requires an ESP-specific toolchain such as `espup`.

**Choose your OS to set up the Rust toolchain and development environment:**

- 🍎 [macOS Setup Guide](docs/SETUP_MAC.md)
- 🪟 [Windows Setup Guide](docs/SETUP_WINDOWS.md)
- 🐧 [Linux Setup Guide](docs/SETUP_LINUX.md)

---

## Backup & Restore

- [Backup & Restore firmware](docs/ESP32-S3-Touch-LCD-1.85C-firmware-backup-restore.md)

This is strongly recommended before experimenting with custom firmware, Rust ports, or low-level display / touch initialization.

---

## Demos & Active Work Areas

### Arduino Demo / Reference Path

- [LVGL Arduino demo](demo/original/Arduino/LVGL_Arduino/)

This folder is the practical Arduino bring-up and validation path for the board. It is useful for:

- confirming display init
- validating touch and backlight
- checking RTC
- verifying SD card access
- testing audio playback on V1 hardware

It is also a useful reference path when comparing Arduino behavior against Rust implementations.

### Rust Work Areas

- [Rust diagnostics](rust/rust-diagnostics/)
- [Rust full port](rust/rust-full-port/)

These folders are the main Rust-side work areas.

**`rust/rust-diagnostics/`** is intended for hardware validation, targeted tests, and subsystem-level experiments.

**`rust/rust-full-port/`** is intended for broader board support and more complete Rust application / firmware work.

### Documentation

- [docs/](docs/)

Repository setup notes, flashing guidance, and related platform documents live here.

---

## 📂 Repository Structure

```text
ESP32-S3-Touch-LCD-1.85C/
├── demo/
│   └── original/
│       └── Arduino/
│           └── LVGL_Arduino/      # Arduino LVGL / RTC / SD / audio reference demo
├── rust/
│   ├── rust-diagnostics/          # Rust hardware validation and board diagnostics
│   └── rust-full-port/            # Broader Rust port / application work
├── docs/                          # Setup guides, flashing notes, recovery docs
└── README.md
```

---

## What the Arduino Demo Currently Represents

The Arduino demo folder is not the final target architecture for this repository, but it is an important working baseline.

It is especially useful for:

- comparing expected hardware behavior against Rust code
- validating board assumptions before porting to Rust
- confirming whether an issue is board-related or Rust-port-related
- documenting V1 vs V2 hardware differences

This is particularly important on this board because the official examples and community examples frequently mix:

- old vs new LVGL APIs
- V1 vs V2 audio paths
- speech-recognition assumptions
- different board package expectations

---

## Current Hardware Notes Worth Carrying Forward

If you are writing custom firmware for this board, read this first.

### 1. Hardware Revision Matters

This board family has meaningful **V1 vs V2** differences.

A major example is audio:

- **V1** uses **PCM5101**
- **V2** uses **ES8311**

Do not assume a demo for one hardware revision will work on the other.

### 2. GPIO 4 is Touch Interrupt, Not a Generic Power Enable

Many examples on the internet incorrectly treat `GPIO 4` as a power-enable line.

On this board, it is tied to the touch interrupt path and should not be casually driven high as a generic output.

### 3. Shared I2C / Peripheral Coordination Matters

Touch, RTC, and expander interactions are sensitive to init order and reset timing.

If a peripheral appears dead, it may be a sequencing issue rather than a permanently bad device.

### 4. Expander-Controlled Reset Behavior Is Important

If the display or touch controller enters a bad state, recovery may require correct reset sequencing through the expander rather than just reflashing firmware.

---

## Why the Rust Focus Matters

The long-term value of this repository is the Rust side:

- safer peripheral access
- better separation of board support vs app logic
- more maintainable async / embedded workflows
- less dependence on fragile vendor example code

The Arduino content exists to accelerate discovery and reduce risk, but the primary ecosystem goal remains a robust Rust implementation for this board.

---

## Suggested Workflow

A practical development flow for this repository is:

1. validate hardware behavior in the Arduino demo path
2. document the actual pin / init / reset behavior
3. reproduce the same subsystem in Rust
4. keep diagnostics and full-port work separated
5. preserve backup / restore guidance as part of normal workflow

---

## Recommended Near-Term Priorities

- keep improving the Rust diagnostics path
- use the Arduino demo only as a hardware reference, not the final architecture
- document V1 vs V2 differences wherever they matter
- continue translating validated hardware behavior into Rust-first implementations
- avoid mixing unrelated experiments into the same demo folder

---

## Summary

This repository is the working home for the **ESP32-S3 Touch LCD 1.85C Rust ecosystem**, with:

- a preserved Arduino reference demo for board validation
- a diagnostics-oriented Rust workspace
- a fuller Rust port workspace
- documentation and recovery notes to support low-level experimentation

The Arduino demo helps confirm what the hardware actually does.  
The Rust work is where the long-term value of the repository is being built.