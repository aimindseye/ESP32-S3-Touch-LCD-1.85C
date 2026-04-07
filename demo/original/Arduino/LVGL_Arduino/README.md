# LVGL Arduino Bring-Up for Waveshare ESP32-S3 Touch LCD 1.85 (V1)

Minimal modern Arduino bring-up for the **Waveshare ESP32-S3 Touch LCD 1.85 V1** using:

- **ESP32 Arduino core 3.2.x**
- **LVGL 9.5.x**
- **SD_MMC**
- **RTC (PCF85063)**
- **Touch + backlight**
- **PCM5101-based audio playback from SD**

This effort replaces the older monolithic Waveshare Arduino demo flow with a cleaner, staged bring-up that works with current Arduino libraries and a round-display-safe UI.

---

## Current Status

Working on **V1 hardware**:

- Display
- Touch
- Backlight control
- Battery readout
- RTC label / RTC refresh
- SD card mount and root listing
- MP3 playback from SD
- LVGL UI with separate **Home** and **Player** tabs

Notes:

- **V1 uses PCM5101 audio**
- **V2 uses ES8311 audio**
- The newer Waveshare `05_audio_out_tf` example is for **V2** and will fail on **V1**

---

## Hardware Version

This README is for:

- **Waveshare ESP32-S3 Touch LCD 1.85 V1**

Known working assumptions for V1:

- Audio path uses **PCM5101**
- Playback pins:
  - `GPIO48` -> `I2S_BCK`
  - `GPIO38` -> `I2S_LRCK`
  - `GPIO47` -> `I2S_DIN`
- SD_MMC pins:
  - `GPIO14` -> `CLK`
  - `GPIO17` -> `CMD`
  - `GPIO16` -> `D0`

---

## Why This Exists

The original Waveshare `LVGL_Arduino` demo in the older package is tied to older library versions and older LVGL APIs.

Typical problems with the original demo on a modern Arduino setup include:

- LVGL 8 vs LVGL 9 API mismatch
- legacy driver structs no longer present in LVGL 9
- speech-recognition partition/model assumptions
- monolithic demo folder layout
- audio path mismatch between V1 and V2 hardware

This project keeps the useful board support files from Waveshare while rebuilding the app/UI flow around current libraries.

---

## Features

### Home tab
- heap display
- battery voltage
- RTC time
- SD usage
- backlight slider
- touch test button

### Player tab
- cleaner audio page for the round screen
- track title and playback state
- playback time / duration
- volume slider
- icon-based transport controls:
  - previous
  - play/pause
  - next
  - stop
- embedded-cover detection placeholder:
  - detects whether an MP3 contains embedded art
  - currently shows a styled cover card state
  - does **not** yet render the actual embedded image

---

## Project Layout

Suggested sketch folder contents:

```text
LVGL_Arduino/
├── LVGL_Arduino.ino
├── LVGL_Driver.h
├── LVGL_Driver.cpp
├── Display_ST77916.h
├── Display_ST77916.cpp
├── esp_lcd_st77916.h
├── esp_lcd_st77916.c
├── Touch_CST816.h
├── Touch_CST816.cpp
├── TCA9554PWR.h
├── TCA9554PWR.cpp
├── I2C_Driver.h
├── I2C_Driver.cpp
├── BAT_Driver.h
├── BAT_Driver.cpp
├── RTC_PCF85063.h
├── RTC_PCF85063.cpp
├── SD_Card.h
└── SD_Card.cpp
```

Do **not** mix in unused legacy files unless you are intentionally re-enabling them.

Examples of files that should stay out unless explicitly needed:

- `MIC_MSM.*`
- old `LVGL_Example.*`
- old `LVGL_Music.*`
- V2-only ES8311 files when targeting V1

---

## Arduino IDE Setup

### Board
Use the Waveshare board selection that worked in this effort, or an equivalent ESP32-S3 target if needed.

### Port
Use the active USB serial port, for example:

```text
/dev/cu.usbmodem8401
```

### Partition scheme
Use a **non-SR** partition scheme.

Working choice:

```text
16M Flash (3MB APP/9.9MB FATFS)
```

Do **not** use the ESP-SR partition scheme unless you are specifically working on speech-recognition examples.

### Upload speed
If flashing is unreliable, reduce upload speed:

- `460800`
- or `115200`

---

## Library Setup

Install or keep available in Arduino:

- `lvgl` **9.5.x**
- `ESP32-audioI2S`
- board-provided `SD_MMC`, `Wire`, and core ESP32 libraries

### LVGL config
A custom `lv_conf.h` is required and should live next to the `lvgl` library folder in the Arduino libraries path.

Expected location:

```text
~/Documents/Arduino/libraries/lv_conf.h
```

Recommended minimum settings:

```c
#if 1

#define LV_COLOR_DEPTH 16

#define LV_USE_DRAW_SW 1
#define LV_USE_DRAW_SW_ASM LV_DRAW_SW_ASM_NONE
#define LV_USE_NATIVE_HELIUM_ASM 0
#define LV_USE_DRAW_ARM2D_SYNC 0

#endif
```

This avoids architecture-specific draw backends that are not appropriate for ESP32-S3.

---

## Known Good Bring-Up Order

The project was stabilized in this order:

1. Display + LVGL
2. Touch
3. Backlight
4. Battery readout
5. RTC
6. SD_MMC
7. Standalone PCM5101 audio test
8. Integrated LVGL player page

This order is recommended if rebuilding from scratch.

---

## Audio Notes

### V1 audio
V1 playback is working using:

- `ESP32-audioI2S`
- PCM5101 path
- I2S pins:
  - `BCLK = 48`
  - `LRC  = 38`
  - `DOUT = 47`

### Tested playback
Confirmed playback from SD for MP3 files in the card root.

### Serial output example
A successful audio startup looks like:

```text
Booting V1 PCM5101 audio test...
SD card type: SDHC
SD total: 7585 MB
SD used : 13 MB
SD: test file found: /solarflex-bollywood-indian-hindi-song-509913.mp3 (4741632 bytes)
SD root listing:
  [DIR] System Volume Information (0 bytes)
  Technical Summary for Waveshare Sup.txt (902 bytes)
  solarflex-bollywood-indian-hindi-song-509913.mp3 (4741632 bytes)
  bounce-bay-records-bollywood-1-437912.mp3 (5236224 bytes)
  freemusicforvideo-bollywood-indian-hindi-song-music-504893.mp3 (4435968 bytes)
Playing: /solarflex-bollywood-indian-hindi-song-509913.mp3
```

### Important
Do **not** use the V2 `ES8311` example on V1 hardware.  
That path will fail during codec initialization.

---

## UI Notes

The display is round, so rectangular full-screen layouts can look clipped.

This project uses:

- a centered safe-content region
- a separate **Home** tab
- a separate **Player** tab

That avoids overcrowding and edge clipping.

---

## Serial Monitor

To view runtime logs:

1. Open **Tools -> Serial Monitor**
2. Set baud rate to `115200`
3. Press reset on the board if needed

The upload/build pane is **not** the same as Serial Monitor.

---

## Troubleshooting

### Build succeeds but upload fails with no serial data
Try:

- close Serial Monitor
- reduce upload speed
- hold **BOOT**
- tap **RST**
- release **BOOT**
- retry upload

### `srmodels.bin` upload error
You are probably using an ESP-SR partition scheme with a non-speech sketch.  
Switch to a normal 16M flash partition scheme.

### LVGL compile errors mentioning old types
You are likely compiling old LVGL 8-era code against LVGL 9.  
Use the rewritten LVGL driver/UI path instead of the old Waveshare LVGL example files.

### ES8311 init fails
You are likely running a V2 audio example on **V1** hardware.  
Use the PCM5101 playback path instead.

### SD works on screen but no serial listing appears
Make sure you are looking at the **Serial Monitor**, not the upload console.

---

## Limitations

Current limitations:

- embedded MP3 cover art is only **detected**, not rendered
- no on-screen file browser yet
- no persistent settings storage yet
- no pause-state persistence across reboot
- no advanced playlist UI yet

---

## Recommended Next Steps

High-value next steps:

1. Render actual embedded album art from MP3 if present
2. Add a simple SD browser / playlist view
3. Add play queue and current-track highlight
4. Persist last volume / last track
5. Add battery icon and RTC date formatting
6. Add cleaner playback progress UI
7. Add speaker mute / soft stop behavior
8. Split the project into:
   - board support
   - LVGL app UI
   - audio/player module

---

## Summary

This Arduino bring-up establishes a practical modern baseline for the **Waveshare ESP32-S3 Touch LCD 1.85 V1** using current libraries.

It successfully replaces the fragile older demo path with a working stack that now supports:

- LVGL 9
- touch
- backlight
- RTC
- SD
- PCM5101 audio playback
- round-screen-safe tabbed UI

This is a good foundation for continuing toward a more polished embedded UI or a fuller device application.