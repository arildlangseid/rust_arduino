#![no_std]
#![no_main]
//#![feature(core_ffi_c)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(dead_code)]
mod arduino;
mod arduino_patch;

//use arduino::{LiquidCrystal_I2C, LiquidCrystal_I2C_write};

use arduino_hal::prelude::*;
use panic_halt as _;
use crate::arduino::Adafruit_SSD1306;
use crate::arduino::TwoWire;

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
    //let mut display = Adafruit_SSD1306::new
//    let mut display = Adafruit_SSD1306::new5(128,64,TwoWire::new(),0,0,0);


    display.begin(0x02, 0x3c, true, true);
    display.display();
//    display.clearDisplay();
//    display.drawPixel(30, 10, 1);
//    display.display();
//    display.clearDisplay();

//    display.invertDisplay();
/*
class Adafruit_SSD1306 : public Adafruit_GFX {
public:
    bool begin(uint8_t switchvcc = SSD1306_SWITCHCAPVCC, uint8_t i2caddr = 0,
                   bool reset = true, bool periphBegin = true);
    void display(void);
    void clearDisplay(void);
    void invertDisplay(bool i);
    void dim(bool dim);
    void drawPixel(int16_t x, int16_t y, uint16_t color);
    virtual void drawFastHLine(int16_t x, int16_t y, int16_t w, uint16_t color);
    virtual void drawFastVLine(int16_t x, int16_t y, int16_t h, uint16_t color);
    void startscrollright(uint8_t start, uint8_t stop);
    void startscrollleft(uint8_t start, uint8_t stop);
    void startscrolldiagright(uint8_t start, uint8_t stop);
    void startscrolldiagleft(uint8_t start, uint8_t stop);
    void stopscroll(void);
    void ssd1306_command(uint8_t c);
    bool getPixel(int16_t x, int16_t y);
    uint8_t *getBuffer(void);



void Adafruit_SSD1306::invertDisplay(bool i) {
  TRANSACTION_START
  ssd1306_command1(i ? SSD1306_INVERTDISPLAY : SSD1306_NORMALDISPLAY);
  TRANSACTION_END
}


    @brief  Dim the display.
    @param  dim
            true to enable lower brightness mode, false for full brightness.
    @return None (void).
    @note   This has an immediate effect on the display, no need to call the
            display() function -- buffer contents are not changed.

void Adafruit_SSD1306::dim(bool dim) {
  // the range of contrast to too small to be really useful
  // it is useful to dim the display
  TRANSACTION_START
  ssd1306_command1(SSD1306_SETCONTRAST);
  ssd1306_command1(dim ? 0 : contrast);
  TRANSACTION_END
}
*/
    ufmt::uwriteln!(&mut serial, "starting on {}\r", 0x27).void_unwrap();

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
        arduino_hal::delay_ms(1000);
    }
}
