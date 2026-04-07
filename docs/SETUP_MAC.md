# 🍎 Mac Setup Guide: Rust on ESP32-S3-Touch-LCD-1.85C

Setting up the Waveshare 1.85C on macOS requires a specific USB driver before your machine will even recognize the board, followed by the Xtensa Rust toolchain.

## Phase 1: The Crucial CH34X USB Driver

The Waveshare 1.85C uses a CH34X USB-to-UART bridge. **Without this driver, your Mac will not see the board when you plug it in.**

1. **Download the Driver:** Download the official Mac driver from the Waveshare Wiki: [CH34XSER_MAC.7z](https://files.waveshare.com/wiki/common/CH34XSER_MAC.7z)
2. **Extract the Archive:** You will need a utility like [The Unarchiver](https://theunarchiver.com/) or Keka to extract `.7z` files on Mac. Inside, you will find the installer and an instructional PDF.
3. **Run the Installer:** Double-click the `.pkg` installer file and follow the standard installation wizard.
4. **Approve the Kernel/System Extension (CRITICAL):**
   * macOS heavily restricts third-party drivers. During or immediately after installation, open your Mac's **System Settings** -> **Privacy & Security**.
   * Scroll down to the "Security" section.
   * You should see a message saying system software from developer "QinHeng Electronics" (or "WCH") was blocked from loading. Click **Allow**.
5. **Restart Your Mac:** You must restart your computer for the driver to fully mount.

Once restarted, plug in your board. If you run `ls /dev/cu.*` in your terminal, you should now see a device like `/dev/cu.wchusbserial...` or `/dev/cu.usbserial...`.

## Phase 2: The Rust Xtensa Toolchain

**1. Install Standard Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

**2. Install the Espressif Toolchain (`espup`)**
Because the ESP32-S3 is an Xtensa chip, standard Rust cannot compile for it.

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
2. **Starting VS Code:** Because the Xtensa compiler lives outside standard Rust paths, `rust-analyzer` will fail if it can't find it. **Always launch VS Code from a terminal where the Espressif environment variables are loaded:**
   ```bash
   . $HOME/export-esp.sh
   code .
   ```

## Phase 4: Flashing and the Mac "Blank Monitor" Quirk

1. Navigate to your app (e.g., `cd diagnostics/hw-diagnostic`).
2. Run `cargo run`.
3. Select your newly mapped `/dev/cu.wchusbserial...` port.

**⚠️ The Mac USB Disconnect Quirk:**
When the ESP32-S3 reboots after a successful flash, macOS physically drops and reconnects the USB port. By the time the OS mounts the port and the serial monitor opens, the board has already printed its startup logs.

* **The Fix:** Once the terminal says it is listening to the port, physically press the **RESET** button on the Waveshare board once. It will reboot while the monitor is actively listening.

---

### Hardware Rescue: Download Mode

If `espflash` fails with "No serial data received," the board is trapped in a crash loop blocking the flasher.

1. Press and **HOLD** the `BOOT` button.
2. Press the `RESET` button once.
3. Release the `BOOT` button.
4. Run `cargo run` again.
