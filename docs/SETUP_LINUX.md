# 🐧 Linux Setup Guide: Rust on ESP32-S3-Touch-LCD-1.85C

Linux developers generally have the smoothest experience with ESP32 development, as the CH34X USB drivers are baked directly into the modern Linux kernel.

## Phase 1: Udev Rules & Dependencies

**1. Install System Dependencies**
Ensure you have the required build packages (Ubuntu/Debian example):

```bash
sudo apt update
sudo apt install build-essential curl gcc python3 libudev-dev
```

**2. Configure Udev Rules**
By default, Linux requires `sudo` to access serial ports. To allow standard users to flash the board, add yourself to the `dialout` group:

```bash
sudo usermod -a -G dialout $USER
```

*(You may need to log out and log back in for this to take effect).*

## Phase 2: The Rust Xtensa Toolchain

**1. Install Standard Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

**2. Install the Espressif Toolchain (`espup`)**
Standard Rust cannot compile for the Xtensa-based ESP32-S3.

```bash
cargo install espup
espup install
```

*Note: This creates an export file (`export-esp.sh`) in your home directory.*

**3. Install the Flasher**

```bash
cargo install cargo-espflash
```

## Phase 3: VS Code Configuration

1. Install these extensions in VS Code:
   * **`rust-analyzer`**
   * **`Even Better TOML`**
   * **`crates`**
2. **Starting VS Code:** `rust-analyzer` needs the Xtensa paths. **Always launch VS Code from a terminal where the Espressif environment variables are loaded:**
   ```bash
   . $HOME/export-esp.sh
   code .
   ```

## Phase 4: Flashing the Board

1. Plug in the Waveshare 1.85C. Verify it is recognized by running `dmesg | tail` or `ls /dev/ttyUSB*` (it usually shows up as `/dev/ttyUSB0` or `/dev/ttyACM0`).
2. Navigate to your app directory (e.g., `cd diagnostics/hw-diagnostic`).
3. Run `cargo run`.
4. Select the corresponding port.

---

### Hardware Rescue: "No serial data received"

If the board is trapped in a hardware crash loop, `espflash` cannot initialize the serial connection.
**The Fix (Manual Download Mode):**

1. Press and **HOLD** the physical `BOOT` button on the back of the board.
2. While holding `BOOT`, press the `RESET` button once.
3. Release the `BOOT` button.
4. Run `cargo run` again.
