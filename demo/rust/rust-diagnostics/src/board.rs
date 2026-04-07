pub const BOARD_NAME: &str = "Waveshare ESP32-S3-Touch-LCD-1.85C / 1.85C-BOX";

// Reference pin map taken from the uploaded Arduino and ESP-IDF demo trees.
pub const I2C_SCL_GPIO: u8 = 10;
pub const I2C_SDA_GPIO: u8 = 11;
pub const TOUCH_INT_GPIO: u8 = 4;

pub const BATTERY_ADC_GPIO: u8 = 8;

pub const SD_CLK_GPIO: u8 = 14;
pub const SD_CMD_GPIO: u8 = 17;
pub const SD_D0_GPIO: u8 = 16;


pub const TCA9554_ADDR: u8 = 0x20;
pub const CST816_ADDR: u8 = 0x15;
pub const PCF85063_ADDR: u8 = 0x51;

// TCA9554 bit positions used by the uploaded reference code.
pub const EXIO_TOUCH_RST: u8 = 0; // EXIO1
pub const EXIO_LCD_RST: u8 = 1;   // EXIO2
pub const EXIO_SD_CS: u8 = 2;     // EXIO3

// From BAT_Driver.h / BAT_Driver.c in the uploaded ESP-IDF demo.
pub const BATTERY_DIVIDER_SCALE: f32 = 3.0;
pub const BATTERY_MEASUREMENT_OFFSET: f32 = 0.9945;

pub const BOARD_FLASH_MIB: u32 = 16;
pub const BOARD_PSRAM_MIB: u32 = 8;
pub const LCD_WIDTH: u16 = 360;
pub const LCD_HEIGHT: u16 = 360;
pub const BACKLIGHT_MAX: u8 = 100;
pub const BACKLIGHT_DEFAULT: u8 = 70;
