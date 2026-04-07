# Porting notes

## Official demo scope mirrored here

Public documentation for the Waveshare Arduino and ESP-IDF demos says the test app is used to validate onboard device functionality. It documents:

- page 1: SD Card, Flash Size, Battery Voltage, RTC Time, Wi-Fi scan, backlight brightness
- page 2: MP3 playback from TF card root
- speech recognition enabled in the vendor demo

This Rust project mirrors the **diagnostics** part of that scope first.

## Hard parts still open

### QSPI LCD

The display is ST77916 on QSPI, not a simple single-lane SPI TFT. That is the main reason this repo is serial-first right now.

### Wi-Fi scan

This is straightforward in ESP-IDF, but not part of a tiny no_std esp-hal bring-up app.

### Audio / MP3 / speech

The vendor test firmware bundles audio and speech subsystems. Recreating that in Rust is possible, but it is a much larger step than the board diagnostics themselves.

## Recommended implementation order

1. keep this serial diagnostics path working
2. add LCD reset + backlight + a solid panel init path
3. draw a simple diagnostics page
4. add TF card probe
5. add Wi-Fi count
6. add audio
