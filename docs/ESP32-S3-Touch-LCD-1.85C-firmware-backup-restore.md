# ESP32-S3-Touch-LCD-1.85C Firmware Backup and Restore Runbook

This runbook explains how to:

1. back up the current firmware from a Waveshare **ESP32-S3-Touch-LCD-1.85C** board,
2. restore the original/demo firmware from Waveshare's Google Drive demo package, and
3. recover from common flashing issues.

It is written around the command-line `esptool` workflow because it is cross-platform and reliable.

## What you need before you start

### Hardware

- Waveshare **ESP32-S3-Touch-LCD-1.85C** or speaker-box variant
- USB-C data cable
- A computer running macOS, Linux, or Windows

### Files to download

- Waveshare demo package / demo firmware:
  - https://drive.google.com/file/d/1MT_-zGMCd-G3eeFi8ajbEoS3G_IHbDSq/view?usp=sharing
- Waveshare product wiki:
  - https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.85C
- Waveshare Mac driver archive:
  - https://files.waveshare.com/wiki/common/CH34XSER_MAC.7z
- CH34X macOS driver installation PDF:
  - https://files.waveshare.com/upload/1/1a/CH34X_DRV_INSTALL_INSTRUCTIONS.pdf

### Tools to install

#### Option A: Recommended cross-platform CLI path

1. **Python 3.10+**
2. **esptool**
3. On macOS, if the board is not recognized or flashing fails, install the **Waveshare/WCH CH34X Mac driver** first
4. A tool that can extract `.7z` archives if you are on macOS or Linux and do not already have one

Install `esptool`:

```bash
python3 -m pip install --upgrade pip
python3 -m pip install esptool
```

Check it:

```bash
python3 --version
python3 -m esptool version
```

#### Option B: Windows GUI path

If you prefer a GUI on Windows, Waveshare links Espressif's **Flash Download Tool** from the product wiki. For backup, the CLI route is still easier and more flexible.

## Board notes that matter for backup/restore

- Waveshare documents this board as having **16MB Flash** and **8MB PSRAM**.
- The onboard I2C pins are fixed to **GPIO10 (SCL)** and **GPIO11 (SDA)**.
- The Waveshare wiki says the demo package includes **test firmware** for checking whether onboard devices work properly.
- On macOS, Waveshare explicitly says to install the **MAC driver** if program flashing fails or if the board is not recognized.

## macOS driver setup before backup or restore

If your Mac does not show a serial port for the board, or if flashing fails even though the cable and board are fine, install the Waveshare/WCH CH34X macOS driver first.

### Download links

- Driver archive: https://files.waveshare.com/wiki/common/CH34XSER_MAC.7z
- Installation guide PDF: https://files.waveshare.com/upload/1/1a/CH34X_DRV_INSTALL_INSTRUCTIONS.pdf

### What the Waveshare/WCH PDF says to do

1. Download `CH34XSER_MAC.7z`.
2. Extract the archive to a local folder.
3. Open the driver package and proceed through the installer.

### Extra step for macOS 11.0 and above

After the package install finishes:

1. open **LaunchPad**,
2. find **CH34xVCPDriver**,
3. open it,
4. click **Install**.

### Extra step for macOS 10.9 to 10.15

After installation:

1. click **Restart** and reboot the Mac,
2. reconnect the board,
3. open **System Report** -> **Hardware** -> **USB**,
4. confirm a USB serial device appears with **Vendor ID `0x1a86`**, and
5. in Terminal run:

```bash
ls /dev/tty*
```

You should see a device such as:

```text
tty.wchusbserialx
```

where `x` is the assigned device number.

### High Sierra security approval note

The PDF notes that **macOS High Sierra 10.13** may require explicit approval for third-party kernel extensions.

If the driver installs but still does not work, open:

- **System Preferences** -> **Security & Privacy** -> **General**

and make sure the system allows software from identified developers.

### macOS uninstall notes from the PDF

#### macOS 10.9 to 10.15

```bash
sudo rm -rf /Library/Extensions/CH34xVCPDriver.kext
sudo rm -rf /var/db/receipts/*CH34xVCPDriver*.*
```

#### macOS 11.0 and above

1. Move the **CH34xVCPDriver** application to **Trash**.
2. Restart the computer before reinstalling the driver.

## Find the serial port

### macOS / Linux

```bash
ls /dev/cu.usbmodem* /dev/tty.usbmodem* /dev/ttyUSB* /dev/ttyACM* /dev/tty.wchusbserial*
```

Typical macOS examples:

```bash
/dev/cu.usbmodem8401
/dev/tty.wchusbserial1410
```

### Windows

Look in **Device Manager** for the COM port, or check inside Arduino IDE / Espressif tools.

## Important prep before flashing or backup

Before running `esptool`:

- close **Arduino IDE Serial Monitor**,
- close any serial terminal app,
- unplug/replug the board if the port looks stuck.

If the board will not enter download mode normally:

1. hold **BOOT**,
2. press **RESET**,
3. release **RESET**,
4. release **BOOT**.

Waveshare also notes that if reconnect/flash fails after using demos, pressing **RESET** for more than 1 second and waiting for the PC to re-recognize the device can help.

## 1) Back up the current firmware

### Recommended full backup command

Replace the serial port with your actual port.

```bash
python3 -m esptool --chip esp32s3 --port /dev/cu.usbmodem8401 --baud 921600 \
  read-flash 0 ALL esp32-s3-touch-lcd-1.85c-backup.bin
```

What this does:

- starts reading at flash offset `0`
- reads the **entire flash chip** using `ALL`
- saves it to `esp32-s3-touch-lcd-1.85c-backup.bin`

### Optional: explicit 16MB backup

If you prefer to be explicit instead of using `ALL`:

```bash
python3 -m esptool --chip esp32s3 --port /dev/cu.usbmodem8401 --baud 921600 \
  read-flash 0 16M esp32-s3-touch-lcd-1.85c-backup.bin
```

### Optional: record flash information

```bash
python3 -m esptool --chip esp32s3 --port /dev/cu.usbmodem8401 flash-id
```

### Optional: create a checksum for your backup

#### macOS / Linux

```bash
shasum -a 256 esp32-s3-touch-lcd-1.85c-backup.bin
```

#### Windows PowerShell

```powershell
Get-FileHash .\esp32-s3-touch-lcd-1.85c-backup.bin -Algorithm SHA256
```

## 2) Restore the Waveshare demo firmware

This runbook assumes you downloaded the single demo/test `.bin` image from the Waveshare demo package and want to restore the board to that state.

### Step A: rename or note the file path

Example file name:

```text
ESP32-S3-Touch-LCD-1.85C.bin
```

### Step B: erase flash

```bash
python3 -m esptool --chip esp32s3 --port /dev/cu.usbmodem8401 erase-flash
```

### Step C: flash the demo image at `0x0`

```bash
python3 -m esptool --chip esp32s3 --port /dev/cu.usbmodem8401 --baud 921600 \
  write-flash 0x0 ESP32-S3-Touch-LCD-1.85C.bin
```

### Step D: power-cycle the board

After the flash completes:

1. unplug USB,
2. wait 5 to 10 seconds,
3. reconnect USB,
4. press **RESET** once if needed.

## 3) Restore your own backup later

If you want to put your own saved image back on the board:

```bash
python3 -m esptool --chip esp32s3 --port /dev/cu.usbmodem8401 erase-flash

python3 -m esptool --chip esp32s3 --port /dev/cu.usbmodem8401 --baud 921600 \
  write-flash 0x0 esp32-s3-touch-lcd-1.85c-backup.bin
```

## 4) Common problems

### `command not found: esptool.py`

Use the Python module form instead:

```bash
python3 -m esptool version
```

If needed:

```bash
python3 -m pip install esptool
```

### `Resource busy` / port busy

Something else still has the serial port open.

#### macOS / Linux

See what is holding the port:

```bash
lsof /dev/cu.usbmodem8401
```

Close Arduino Serial Monitor or kill the listed PID.

### Board not detected on macOS

Work through these in order:

1. confirm the USB-C cable is a **data** cable,
2. install the Waveshare/WCH **CH34X Mac driver**,
3. reboot if the installer or PDF says to do so for your macOS version,
4. check **System Report** -> **USB** for **Vendor ID `0x1a86`**, and
5. check Terminal output with:

```bash
ls /dev/tty* /dev/cu.*
```

### Board not detected after a failed demo or bad flash

Try this sequence:

1. unplug the board,
2. reconnect it,
3. hold **BOOT**,
4. tap **RESET**,
5. release **RESET**,
6. release **BOOT**,
7. retry `erase-flash` and `write-flash`.

### Display shows bars/flashing after bad demo code

If touch still works or the board still enumerates over USB, the board may only need a full erase and reflash of the merged demo firmware image.

## 5) Recommended file layout

```text
firmware/
├── ESP32-S3-Touch-LCD-1.85C.bin
├── esp32-s3-touch-lcd-1.85c-backup.bin
└── checksums.txt
```

## 6) Suggested recovery workflow

1. Install the Mac driver first if you are on macOS and the board is not recognized.
2. Download the Waveshare demo package.
3. Back up the board before experimenting.
4. Save a SHA256 checksum of your backup.
5. Test custom firmware.
6. If recovery is needed, erase flash and write the Waveshare demo image at `0x0`.
7. If you later want your own board state back, erase flash and write your saved backup image at `0x0`.

## Source links

- Waveshare product wiki: https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.85C
- Waveshare demo package: https://drive.google.com/file/d/1MT_-zGMCd-G3eeFi8ajbEoS3G_IHbDSq/view?usp=sharing
- Waveshare Mac driver archive: https://files.waveshare.com/wiki/common/CH34XSER_MAC.7z
- CH34X macOS driver installation PDF: https://files.waveshare.com/upload/1/1a/CH34X_DRV_INSTALL_INSTRUCTIONS.pdf
- esptool installation docs: https://docs.espressif.com/projects/esptool/en/latest/esp32/installation.html
- esptool basic commands docs: https://docs.espressif.com/projects/esptool/en/latest/esp32s3/esptool/basic-commands.html
