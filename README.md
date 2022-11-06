# Example of an Arduino library usage in a Rust project

The project tested with Arduino UNO on Fedora 35.  
It demonstrates the usage of [LiquidCrystal_I2C](https://github.com/johnrickman/LiquidCrystal_I2C) 
with rust project to control I2C Text Display from Rust. 
It also shows how to combine it with existing Arduino rust crates.
``arduino_hal`` crate is used to blink the LED.

## Project setup

- Install avr toolchain and avrdude
```sh
sudo dnf install avrdude avr-gcc avr-libc
```
- Install dependencies to compile ``ravedude``
```sh
sudo dnf install systemd-devel pkgconf-pkg-config
```
- Install  ``ravedude``:
```sh
cargo install ravedude
```
- Install ``bindgen`` dependencies:
```
sudo dnf install clang-devel
```
- Install arduino IDE and validate it is working by compiling a simple sketch.
- Install [LiquidCrystal_I2C](https://github.com/johnrickman/LiquidCrystal_I2C) library to Arduino Libraries folder
(manually or using Arduino IDE like you would normally do)
- Edit ```arduino.yaml``` to configure your arduino installation and version of core library
- Ensure to have rustfmt installed
```sh
rustup component add rustfmt
```
- Plug in your Arduino UNO and run the project:
```sh
cargo run
```

If you see the following error
```
> avrdude: ser_open(): can't open device "/dev/ttyACM0": Permission denied
```
then try editing the permission or adding you user to ``dialout`` group (require re-login):
```
sudo chmod a+rw /dev/ttyACM0
sudo usermod -aG dialout $USER
```

## Windows

If you are running on Windows, consider different way of system dependencies installation.

- Instead of install avr tools using dnf, locate them in your Arduino IDE installation and add the folders containing
``avr-gcc`` and ``avrdude`` to the PATH. I had to add the following:
  - ``%LOCALAPPDATA%\Arduino15\packages\arduino\tools\avr-gcc\7.3.0-atmel3.6.1-arduino7\bin\`` 
  - ``%LOCALAPPDATA%\Arduino15\packages\arduino\tools\avrdude\6.3.0-arduino17\bin\``
- Ensure ``avr-gcc`` and ``avrdude`` are found.
- Download and install clang for Windows [here](https://releases.llvm.org/download.html)
- Setup ``LIBCLANG_PATH`` to point to ``<CLANG_INSTALL_DIR>\bin``
- Setup ``AR`` to point to ``%LIBCLANG_PATH%\llvm-ar.exe``
- Don't forget to update arduino.yaml with correct paths.
- ``cargo build`` and ``cargo run`` should work now!
