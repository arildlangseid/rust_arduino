#![no_std]
#![no_main]
//#![feature(core_ffi_c)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]
mod clibs_bindings;
mod clibs_bindings_patch;

//use arduino::{LiquidCrystal_I2C, LiquidCrystal_I2C_write};

use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
use panic_halt as _;
use crate::clibs_bindings::Adafruit_SSD1306;
use crate::clibs_bindings::TwoWire;

extern "C" {
    fn init();
}

#[arduino_hal::entry]
unsafe fn main() -> ! {
    init();

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();

//    #define OLED_RESET     -1 // Reset pin # (or -1 if sharing Arduino reset pin)
//    Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);
    let mut two_wire = TwoWire::new();
    let ptr: *mut TwoWire = &mut two_wire;
    let mut display = Adafruit_SSD1306::new(128,64,ptr,-1, 400000, 100000);


    display.begin(0x02, 0x3c, true, true);
//    delay_ms(1000);
//    display.invertDisplay(false);
//    display.display();
//    display.clearDisplay();
//    display.drawPixel(30, 10, 1);
//    delay_ms(1000);
//    display.display();
//    display.clearDisplay();

//    display.invertDisplay();

    ufmt::uwriteln!(&mut serial, "starting on {}\r", 0x27).void_unwrap();

//    delay_ms(1000);
//    display.invertDisplay(true);

//    let mut lcd = LiquidCrystal_I2C::new(0x27, 16, 2);

    /*
        let ferris = &[
            0b01010u8, 0b01010u8, 0b00000u8, 0b00100u8, 0b10101u8, 0b10101u8, 0b11111u8, 0b10101u8,
        ];

        lcd.begin(16, 2, 0);
        lcd.init();
        lcd.backlight();

        lcd.clear();
        lcd.printstr("Good morning\0".as_ptr().cast());
        lcd.setCursor(0, 1);
        lcd.printstr("from Rust!!\0".as_ptr().cast());

        lcd.createChar(0, ferris.as_ptr() as *mut _);
        lcd.setCursor(12, 1);

        LiquidCrystal_I2C_write((&mut lcd as *mut LiquidCrystal_I2C).cast(), 0);
    */

    loop {
        led.toggle();
        delay_ms(1000);
        display.invertDisplay(true);
        display.display();
        delay_ms(1000);
        display.invertDisplay(false);
        display.display();

    }
}
