#include "arduino_helper_lib.h"

#include "Arduino.h"

void arduino_init() {
    init();
}

void arduino_attachUSB() {
#if defined(USBCON)
    USBDevice.attach();
#endif
}

void arduino_serialEventRun() {
    if (serialEventRun) serialEventRun();
}

bool check_serial()
{
/*
    int32_t result = a + b;
    printf("[C] source: Argument a is:{ %i }, Argument b is:{ %i } \n", a, b);
    printf("[C] source: returning the result { %i } to Rust\n", result);
*/
//    init();

//pinMode(LED_BUILTIN, OUTPUT);
/*
    while (!Serial) {
      digitalWrite(LED_BUILTIN, HIGH);  // turn the LED on (HIGH is the voltage level)
      delay(100);
      digitalWrite(LED_BUILTIN, LOW);   // turn the LED off by making the voltage LOW
      delay(100);
    };
    Serial.println("HelloWorld");
*/
    return true;
}