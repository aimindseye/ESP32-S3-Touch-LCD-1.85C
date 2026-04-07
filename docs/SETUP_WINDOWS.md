# 🪟 Windows Setup Guide: Rust on ESP32-S3-Touch-LCD-1.85C

Setting up the Waveshare 1.85C on Windows requires installing the Xtensa Rust toolchain and ensuring Windows recognizes the CH34X USB driver.

## Phase 1: USB Drivers & Basic Rust

**1. USB Drivers**
Windows 10/11 is usually excellent at automatically fetching CH34X drivers. Plug your board in. If you check **Device Manager** under "Ports (COM & LPT)" and see a "USB-SERIAL CH340" (or CH343) on a COM port, you are good to go. If it shows up with a yellow warning triangle, download the Windows driver from the [Waveshare Wiki](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.85C).

**2. Install Standard Rust**
Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs/).

**3. C++ Build Tools**
You must have the Visual Studio C++ Build Tools installed for Rust to compile properly on Windows. You can get them from the [Microsoft Visual Studio site](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

## Phase 2: The Rust Xtensa Toolchain

**1. Install the Espressif Toolchain (`espup`)**
Open PowerShell or Windows Terminal:

```powershell
cargo install espup
espup install
```

*Note: This creates an export file (`export-esp.ps1`) in your user directory.*

**2. Install the Flasher**

```powershell
cargo install cargo-espflash
```

## Phase 3: VS Code Configuration

1. Install these extensions in VS Code:
   * **`rust-analyzer`**
   * **`Even Better TOML`**
   * **`crates`**
2. **Starting VS Code:** Because the Xtensa compiler lives outside standard Rust paths, `rust-analyzer` will show errors if launched normally. **Always launch VS Code from PowerShell where the Espressif environment variables are loaded:**
   ```powershell
   . $HOME\export-esp.ps1
   code .
   ```

## Phase 4: Flashing the Board

1. Navigate to your app directory (e.g., `cd diagnostics/hw-diagnostic`).
2. Run `cargo run`.
3. Select the `COM` port that corresponds to your board.

---

### Hardware Rescue: "No serial data received"

If the board is running code that crashes in a tight loop, it drops the USB connection and `espflash` will fail to connect.
**The Fix (Manual Download Mode):**

1. Press and **HOLD** the physical `BOOT` button on the back of the board.
2. While holding `BOOT`, press the `RESET` button once.
3. Release the `BOOT` button.
4. Run `cargo run` again. The board is now locked in a safe listening state.
