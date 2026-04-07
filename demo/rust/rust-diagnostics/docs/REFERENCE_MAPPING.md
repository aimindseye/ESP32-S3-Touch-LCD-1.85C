# Reference mapping used for the Rust scaffold

This document records which uploaded vendor files informed the Rust port.

## Top-level init order

### ESP-IDF
- `main/main.c`
  - `Driver_Init()`
  - `Flash_Searching()`
  - `BAT_Init()`
  - `I2C_Init()`
  - `EXIO_Init()`
  - `PCF85063_Init()`
  - `SD_Init()`
  - `LCD_Init()`
  - `Audio_Init()`
  - `MIC_Speech_init()`
  - `LVGL_Init()`

### Arduino
- `examples/LVGL_Arduino/LVGL_Arduino.ino`
  - `Flash_test()`
  - `BAT_Init()`
  - `I2C_Init()`
  - `TCA9554PWR_Init(0x00)`
  - `Backlight_Init()`
  - `PCF85063_Init()`
  - `SD_Init()`
  - `Audio_Init()`
  - `MIC_Init()`
  - `LCD_Init()`
  - `Lvgl_Init()`

## Touch path

### ESP-IDF
- `main/Touch_Driver/CST816.h`
- `main/Touch_Driver/CST816.c`

Key details copied into the Rust scaffold:
- I2C addr `0x15`
- touch IRQ GPIO4
- touch reset through `Set_EXIO(TCA9554_EXIO1, ...)`
- `read_id()` using `0xA7`
- disable auto sleep using register `0xFE`
- coordinate read starting at register `0x02` in the ESP-IDF helper, or `0x01` in the Arduino helper

### Arduino
- `examples/LVGL_Arduino/Touch_CST816.h`
- `examples/LVGL_Arduino/Touch_CST816.cpp`

Copied behavior:
- IRQ-driven reads
- config reads from `0x15`, `0xA7`, `0xA8`, `0xA9`
- write `10` to `0xFE`

## LCD / backlight

### ESP-IDF
- `main/LCD_Driver/ST77916.h`
- `main/LCD_Driver/ST77916.c`

Key details captured:
- 360x360 display
- QSPI display pin map
- LCD reset via `EXIO2`
- backlight on GPIO5
- vendor ST77916 init table exists here and should be the basis of a future full display port

## EXIO expander

### ESP-IDF
- `main/EXIO/TCA9554PWR.h`
- `main/EXIO/TCA9554PWR.c`

Copied behavior:
- expander addr `0x20`
- config register `0x03`
- output register `0x01`
- `TCA9554PWR_Init(0x00)` means all outputs

## RTC

### ESP-IDF
- `main/PCF85063/PCF85063.h`
- `main/PCF85063/PCF85063.c`

Copied behavior:
- RTC addr `0x51`
- time read starts at register `0x04`
- BCD conversion layout

## Battery

### ESP-IDF
- `main/BAT_Driver/BAT_Driver.h`
- `main/BAT_Driver/BAT_Driver.c`

Copied behavior:
- ADC channel for GPIO8 path
- measurement scale uses divider factor 3 and `Measurement_offset = 0.994500`

## Demo fields shown on the onboard page

### ESP-IDF / Arduino
- `main/LVGL_UI/LVGL_Example.c`
- `examples/LVGL_Arduino/LVGL_Example.cpp`

Rust serial output mirrors these fields:
- SD Card
- Flash Size
- Battery Voltage
- RTC Time
- Wireless Scan
- Backlight Brightness

## Still not ported

- ST77916 full QSPI display bring-up
- LVGL UI
- SD/MMC mount + card size
- Wi-Fi scan task
- PCM5101 playback
- ESP-SR wake word / speech command flow
