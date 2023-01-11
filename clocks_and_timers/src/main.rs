#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1ms
    tim6.arr.write(|w| w.arr().bits(ms));
    // CEN: Enable the counter
    tim6.cr1.write(|w| w.cen().set_bit());
    // Wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}
    // Clear the update event flag (or next time we enter this function it will think the update event has already
    // happend and skip over the busy waiting part)
    tim6.sr.write(|w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // Power on the TIM6 timer
    rcc.apb1enr.write(|w| w.tim6en().set_bit());

    // OPM select one pulse mode
    // CEN keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // Configure the prescaler to have the counter operate at 1 Khz
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 Mhz / (7999 + 1) = 1KHz
    // The counter (CNT) will increase on every millisecond
    tim6.psc.write(|w| w.psc().bits(7999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}