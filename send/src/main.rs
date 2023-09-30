#![no_std]   
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600); // connect to serial

    ufmt::uwriteln!(serial, "Hello there!\r").unwrap();

    println!("this is a std lib thing, and wont work here :(");

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
        ufmt::uwriteln!(serial, "f(4) = {}", f(4));
    }   
}


fn f(x: i64) -> i64 {
    i64::pow(x, 2)
}