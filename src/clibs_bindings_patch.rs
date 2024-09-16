
use crate::clibs_bindings::{Adafruit_SSD1306};

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


/*
When function-impl is not supported:
1: Copy the extern declaration from clibs_bindings.rs to clibs_bindings_patch.rs
2: Create/Copy/Paste/Edit at will the implementation
3: Add function to the blocklist_function: in clibs_bindings.yaml
 */
/*
1:
extern "C" {
    #[link_name = "\u{1}_ZN16Adafruit_SSD130613invertDisplayEb"]
    pub fn Adafruit_SSD1306_invertDisplay(this: *mut ::core::ffi::c_void, i: bool);
}
2:
impl Adafruit_SSD1306 {
    #[inline]
    pub unsafe fn invertDisplay(&mut self, i: bool) { Adafruit_SSD1306_invertDisplay(self, i) } // this: *mut ::core::ffi::c_void
}
3:
blocklist_function:
- Adafruit_SSD1306_invertDisplay.*
*/

#[allow(dead_code)]
extern "C" {
    #[link_name = "\u{1}_ZN16Adafruit_SSD130613invertDisplayEb"]
    pub fn Adafruit_SSD1306_invertDisplay(this: *mut Adafruit_SSD1306, i: bool);
}

#[allow(dead_code)]
impl Adafruit_SSD1306 {
    #[inline]
    pub unsafe fn invertDisplay(&mut self, i: bool) { Adafruit_SSD1306_invertDisplay(self, i) } // this: *mut ::core::ffi::c_void
}
