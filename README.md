## ESP32-S3-Touch-LCD-1.85C (Rust Ecosystem)

A community-driven repository for developing bare-metal Rust (`esp-hal`) applications on the [Waveshare ESP32-S3-Touch-LCD-1.85C](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.85C) circular display module.

The official documentation for this board relies heavily on C++ and the ESP-IDF framework. This repository exists to provide robust, asynchronous, and memory-safe **Rust** implementations for all onboard hardware.

## ⚠️ CAUTION & DISCLAIMER ⚠️

**USE AT YOUR OWN RISK.**
This repository contains low-level hardware initialization code, firmware flashing instructions, and deep-reset sequences

* Incorrectly driving certain pins (like driving an interrupt line `HIGH` as an output) can lock up internal chips.
* Sending malformed I2C commands to the Power Management/Expander ICs can result in a "brown-out" state or temporary bricking of the display.
* **The maintainers are not responsible for bricked devices, damaged screens, or fried silicon.** Always keep a backup of the original factory `.bin` firmware to restore your device via `esptool` if you get stuck.

---

## 🚀 Quick Start: Environment Setup

Because the ESP32-S3 uses the Xtensa architecture, standard Rust requires a specialized toolchain (`espup`). Furthermore, this board requires specific CH34X USB drivers depending on your operating system.

**Choose your OS to set up VS Code and the Rust Toolchain:**

* 🍎 [macOS Setup Guide](docs/SETUP_MAC.md) *(Requires specific kernel extension approvals)*
* 🪟 [Windows Setup Guide](docs/SETUP_WINDOWS.md)
* 🐧 [Linux Setup Guide](docs/SETUP_LINUX.md)

---

## Backup & Restore

- [Backup & Restore firmware](docs/ESP32-S3-Touch-LCD-1.85C-firmware-backup-restore.md)


## 📂 Repository Structure

```text
ESP32-S3-Touch-LCD-1.85C/
├── docs/                   # OS-specific setup and flashing guides
├── diagnostics/            # Core hardware validation and telemetry suites
│   ├── hw-diagnostic/      # Validates I2C, Touch IRQ, Expander, and IMU
│   └── interactive-display/# SPI display init, screen clearing, and touch drawing
└── apps/                   # Advanced product-focused applications
    ├── home-assistant-node/# Native MQTT client / IoT smart display
    ├── flatsphere/         # 3D math and texture mapping visualizer
    ├── mp3-player/         # I2S audio decoding and playback
    └── voice-assistant/    # I2S microphone streaming for wake-words
```

---

## 💡 The "Aha!" Hardware Discoveries

If you are writing custom firmware for this board, **read this first**. 

### 1. GPIO 4 is an INTERRUPT, not a Power Enable!

Many community examples treat `GPIO 4` as the LDO power-enable pin, driving it `OUTPUT HIGH`. **Do not do this on the 1.85C.**

* `GPIO 4` is hardwired to the `TP_INT` (Touch Interrupt) line of the CST816 touch controller.
* If you drive it high, you gag the touch controller. It will freeze, the I2C bus will hang, and it will appear "dead."
* **The Fix:** Set `GPIO 4` as an `INPUT` with a `PULLUP`. Read it to know when a touch event occurs. The peripheral power rail is handled natively or via the Expander.

### 2. The Single I2C Bus Architecture

Standard documentation implies the Touch controller is on a secondary I2C bus (GPIO 1 & 2). It is not.

* The internal TCA9554 Expander (`0x20`), the CST816 Touch controller (`0x15`), and the QMI8658 IMU (`0x6B`) all share the **Main I2C0 Bus** on `SDA: 11` and `SCL: 10`.

### 3. The Expander Deep Reset

If your screen is showing flashing lines, it is trapped in a floating reset state. You must use the I2C Expander (`0x20`) to simultaneously pull `LCD_RST` (P0) and `TP_RST` (P1) LOW for 100ms, then pull them HIGH to boot the hardware.
