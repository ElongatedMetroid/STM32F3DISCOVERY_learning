#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    // let mut half_period = 500_u16;
    // The compiler is smart and recognized that half_period didn't change and instead, in the two
    // places where `delay.delay_ms(half_period);` is called we see `mov.w r1, #500`. So changing
    // half_period when debuging ((gdb) set half_period = ...)
    // let v_half_period = Volatile::new(&mut half_period);

    loop {
        // Turn light on for 150ms, Turn a new light on every 100ms
        for i in 0..leds.len() {
            // Turn on an led
            leds[i].on().ok();
            // Wait 100ms
            delay.delay_ms(100u16);
            // Turn on the next led (if we are at the last led, turn on the first one)
            leds[if i + 1 > leds.len() - 1 { 0 } else { i + 1 } ].on().ok();
            // Wait 50ms
            delay.delay_ms(50u16);
            // Turn off the first led we turned on
            leds[i].off().ok();
        }
    }
}
