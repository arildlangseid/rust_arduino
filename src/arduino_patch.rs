
use crate::arduino::{Adafruit_SSD1306};

#[allow(dead_code)]
extern "C" {
    #[link_name = "\u{1}_ZN16Adafruit_SSD13069drawPixelEiij"]
    pub fn Adafruit_SSD1306_drawPixel(this: *mut Adafruit_SSD1306, x: i16, y: i16, color: u16);
}

#[allow(dead_code)]
impl Adafruit_SSD1306 {
    #[inline]
    pub unsafe fn drawPixel(&mut self, x: i16, y: i16, color: u16) { Adafruit_SSD1306_drawPixel(self, x, y, color) }
}
