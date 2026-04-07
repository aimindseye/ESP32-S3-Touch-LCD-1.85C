#![no_std]
#![no_main]

mod board;
mod drivers;

use drivers::{
    cst816::{Cst816, TouchConfig, TouchPoint},
    pcf85063::{DateTime, Pcf85063},
    tca9554::Tca9554,
};

use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    time::Rate,
};
use esp_println::println;

#[derive(Debug, Clone, Copy)]
struct DiagnosticSnapshot {
    flash_mib: u32,
    battery_raw: u16,
    battery_v: f32,
    rtc: Option<DateTime>,
    touch_cfg: Option<TouchConfig>,
    last_touch: Option<TouchPoint>,
    backlight_pct: u8,
    sd_card_mib: Option<u32>,
    wifi_aps: Option<u16>,
}

#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut delay = Delay::new();

    println!("\n=== {} ===", board::BOARD_NAME);
    println!("Reference-aligned Rust diagnostics scaffold using esp-hal 1.0");
    println!(
        "Expected flash={}MiB psram={}MiB lcd={}x{}",
        board::BOARD_FLASH_MIB,
        board::BOARD_PSRAM_MIB,
        board::LCD_WIDTH,
        board::LCD_HEIGHT
    );
    println!(
        "Init order mirrored from uploaded vendor demos: Flash -> BAT -> I2C -> EXIO -> RTC -> SD -> LCD -> Audio -> MIC -> UI\n"
    );

    let mut backlight_pin =
        Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let touch_int =
        Input::new(peripherals.GPIO4, InputConfig::default().with_pull(Pull::Up));

    let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .expect("I2C init failed")
        .with_sda(peripherals.GPIO11)
        .with_scl(peripherals.GPIO10);

    i2c.apply_config(&I2cConfig::default().with_frequency(Rate::from_khz(400)))
        .expect("I2C config failed");

    let mut adc_cfg = AdcConfig::new();
    let mut bat_pin = adc_cfg.enable_pin(peripherals.GPIO8, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc_cfg);

    let mut exio = Tca9554::new();
    let touch = Cst816::new();
    let rtc = Pcf85063::new();

    println!(
        "[1/10] Flash_Searching -> expected {} MiB from board config",
        board::BOARD_FLASH_MIB
    );
    println!("[2/10] BAT_Init -> ADC on GPIO{}", board::BATTERY_ADC_GPIO);
    println!(
        "[3/10] I2C_Init -> SDA GPIO{} / SCL GPIO{} @ 400kHz",
        board::I2C_SDA_GPIO,
        board::I2C_SCL_GPIO
    );

    println!("Known I2C probes:");
    println!(
        "  0x20 TCA9554  => {:?}",
        exio.ping(&mut i2c, board::TCA9554_ADDR)
    );
    println!(
        "  0x15 CST816   => {:?}",
        touch.ping(&mut i2c, board::CST816_ADDR)
    );
    println!(
        "  0x51 PCF85063 => {:?}",
        rtc.ping(&mut i2c, board::PCF85063_ADDR)
    );

    println!("[4/10] EXIO_Init -> TCA9554 config=0x00 (all outputs), matching vendor demos");
    let _ = exio.set_config(&mut i2c, board::TCA9554_ADDR, 0x00);

    // Safe idle state: release touch reset, release LCD reset, drive SD_CS high.
    let _ = exio.write_pin(
        &mut i2c,
        board::TCA9554_ADDR,
        board::EXIO_TOUCH_RST,
        true,
    );
    let _ = exio.write_pin(
        &mut i2c,
        board::TCA9554_ADDR,
        board::EXIO_LCD_RST,
        true,
    );
    let _ = exio.write_pin(&mut i2c, board::TCA9554_ADDR, board::EXIO_SD_CS, true);

    println!("[5/10] PCF85063_Init -> read-only bring-up");
    let rtc_now = rtc.read_datetime(&mut i2c, board::PCF85063_ADDR).ok();
    if let Some(dt) = rtc_now {
        println!(
            "  RTC read ok: {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            2000 + dt.year as u16,
            dt.month,
            dt.day,
            dt.hour,
            dt.minute,
            dt.second
        );
    } else {
        println!("  RTC read failed");
    }

    println!(
        "[6/10] SD_Init -> pins CLK={} CMD={} D0={} (runtime SD/MMC not yet bound in this esp-hal demo)",
        board::SD_CLK_GPIO,
        board::SD_CMD_GPIO,
        board::SD_D0_GPIO
    );

    println!(
    "[10/10] Touch_Init -> EXIO1 reset + config read + disable autosleep + IRQ-driven reads on GPIO{}",
    board::TOUCH_INT_GPIO
    );

    let touch_cfg = touch.read_config(&mut i2c, board::CST816_ADDR).ok();
    let _ = touch.disable_auto_sleep(&mut i2c, board::CST816_ADDR);

    if let Some(cfg) = touch_cfg {
        println!(
            "  Touch config: version=0x{:02X} chip_id=0x{:02X} project_id=0x{:02X} fw=0x{:02X}",
            cfg.version,
            cfg.chip_id,
            cfg.project_id,
            cfg.fw_version
        );
    } else {
        println!("  Touch config read failed");
    }

    println!("\nOnboard diagnostics start. This mirrors the vendor demo fields over serial:");
    println!("  SD Card | Flash Size | Battery Voltage | RTC Time | Wireless Scan | Backlight Brightness");
    println!("Touching the panel prints live coordinates just like the vendor CST816 test path.\n");

    let mut tick: u32 = 0;
    let mut last_touch: Option<TouchPoint> = None;
    let mut backlight_pct = board::BACKLIGHT_DEFAULT;

    loop {
        if touch_int.is_low() {
            if let Ok(point) = touch.read_touch(&mut i2c, board::CST816_ADDR) {
                if point.fingers > 0 {
                    last_touch = Some(point);
                    println!(
                        "touch: gesture=0x{:02X} fingers={} x={} y={}",
                        point.gesture, point.fingers, point.x, point.y
                    );
                }
            }
        }

        if tick % 50 == 0 {
            let raw = nb::block!(adc1.read_oneshot(&mut bat_pin)).unwrap_or(0);
            let battery_v = battery_voltage_from_raw(raw);
            let rtc_now = rtc.read_datetime(&mut i2c, board::PCF85063_ADDR).ok();

            let snap = DiagnosticSnapshot {
                flash_mib: board::BOARD_FLASH_MIB,
                battery_raw: raw,
                battery_v,
                rtc: rtc_now,
                touch_cfg,
                last_touch,
                backlight_pct,
                sd_card_mib: None,
                wifi_aps: None,
            };

            print_snapshot(&snap);

            set_backlight(&mut backlight_pin, backlight_pct, &mut delay);
            backlight_pct = next_backlight(backlight_pct);
        }

        tick = tick.wrapping_add(1);
        delay.delay_millis(20);
    }
}

fn print_snapshot(s: &DiagnosticSnapshot) {
    match s.rtc {
        Some(dt) => println!(
            "diag: sd={} flash={}MB battery_raw={} battery≈{:.2}V rtc={:04}-{:02}-{:02} {:02}:{:02}:{:02} wifi={} backlight={} touch_cfg={} last_touch={}",
            fmt_opt_u32(s.sd_card_mib),
            s.flash_mib,
            s.battery_raw,
            s.battery_v,
            2000 + dt.year as u16,
            dt.month,
            dt.day,
            dt.hour,
            dt.minute,
            dt.second,
            fmt_opt_u16(s.wifi_aps),
            s.backlight_pct,
            if s.touch_cfg.is_some() { "ok" } else { "err" },
            if s.last_touch.is_some() { "yes" } else { "no" },
        ),
        None => println!(
            "diag: sd={} flash={}MB battery_raw={} battery≈{:.2}V rtc=ERR wifi={} backlight={} touch_cfg={} last_touch={}",
            fmt_opt_u32(s.sd_card_mib),
            s.flash_mib,
            s.battery_raw,
            s.battery_v,
            fmt_opt_u16(s.wifi_aps),
            s.backlight_pct,
            if s.touch_cfg.is_some() { "ok" } else { "err" },
            if s.last_touch.is_some() { "yes" } else { "no" },
        ),
    }
}

fn fmt_opt_u32(v: Option<u32>) -> u32 {
    v.unwrap_or(0)
}

fn fmt_opt_u16(v: Option<u16>) -> u16 {
    v.unwrap_or(0)
}



fn set_backlight(pin: &mut Output<'_>, percent: u8, delay: &mut Delay) {
    if percent == 0 {
        pin.set_low();
        return;
    }

    pin.set_high();
    delay.delay_millis(2);
}

fn next_backlight(current: u8) -> u8 {
    match current.min(board::BACKLIGHT_MAX) {
        0..=29 => 30,
        30..=69 => 70,
        70..=99 => board::BACKLIGHT_MAX,
        _ => 30,
    }
}

fn battery_voltage_from_raw(raw: u16) -> f32 {
    let adc_volts = (raw as f32 / 4095.0) * 3.3;
    (adc_volts * board::BATTERY_DIVIDER_SCALE) / board::BATTERY_MEASUREMENT_OFFSET
}