/*
 in terminal:
 build project
 in folder: /target/avr-atmega32u4/debug/build/avr-led-rust-3d7928f726616c33/out/
 readelf -sW 85ec323f8a4bcfb9-USBCore.o | grep FUNC

*/
#[repr(C, packed)]
pub struct USBDevice {
    pub dummy: i8
    /*
        pub _base: Adafruit_GFX,
        /**< Initialized during construction when using SPI. See
        < SPI.cpp, SPI.h*/
        pub spi: *mut SPIClass,
        /**< Initialized during construction when using I2C. See
        < Wire.cpp, Wire.h*/
        pub wire: *mut TwoWire,
        /**< Buffer data used for display buffer. Allocated when
        < begin method is called.*/
        pub buffer: *mut u8,
        ///< I2C address initialized when begin method is called.
        pub i2caddr: i8,
        ///< VCC selection, set by begin method.
        pub vccstate: i8,
        ///< not used
        pub page_end: i8,
        /**< (Master Out Slave In) set when using SPI set during
        < construction.*/
        pub mosiPin: i8,
        ///< (Clock Pin) set when using SPI set during construction.
        pub clkPin: i8,
        ///< (Data Pin) set when using SPI set during construction.
        pub dcPin: i8,
        ///< (Chip Select Pin) set when using SPI set during construction.
        pub csPin: i8,
        ///< Display reset pin assignment. Set during construction.
        pub rstPin: i8,
        pub mosiPort: *mut PortReg,
        pub clkPort: *mut PortReg,
        pub dcPort: *mut PortReg,
        pub csPort: *mut PortReg,
        pub mosiPinMask: PortMask,
        pub clkPinMask: PortMask,
        pub dcPinMask: PortMask,
        pub csPinMask: PortMask,
        ///< Wire speed for SSD1306 transfers
        pub wireClk: u32,
        ///< Wire speed following SSD1306 transfers
        pub restoreClk: u32,
        ///< normal contrast setting for this device
        pub contrast: u8,
        pub spiSettings: SPISettings,
    */
}
extern "C" {
    #[link_name = "\u{1}_ZN10USBDevice_C2Ev"]
    pub fn USBDevice_USBDevice(
        this: *mut USBDevice
    );
}
#[allow(dead_code)]
extern "C" {
    #[link_name = "\u{1}_ZN10USBDevice_6attachEv"]
    pub fn USBDevice_attach(this: *mut USBDevice);
}

#[allow(dead_code)]
impl USBDevice {
    #[inline]
    pub unsafe fn new(
    ) -> Self {
        let mut __bindgen_tmp = ::core::mem::MaybeUninit::uninit();
        USBDevice_USBDevice(
            __bindgen_tmp.as_mut_ptr()
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn attach(&mut self) { USBDevice_attach(self) } // this: *mut ::core::ffi::c_void
}

extern "C" {
    #[link_name = "\u{1}init"]
    pub fn init();
}

extern "C" {
    #[link_name = "\u{1}_Z14serialEventRunv"]
    pub fn serialEventRun();
}
/*
#[allow(dead_code)]
extern "C" {
    #[link_name = "\u{1}_ZN10USBDevice_6attachEv"]
    pub fn USBDevice_attach(this: *mut USBDevice);
}
extern "C" {
    fn serialEventRun();
}

//serialEventRun()
//_Z14serialEventRunv
*/