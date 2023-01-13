#![no_main]
#![no_std]

use core::fmt::{self, Write};
use aux11::{entry, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            // Wait until its safe to write to TDR
            while self.usart1.isr.read().txe().bit_is_clear() {}

            // Write a byte 
            self
                .usart1
                .tdr
                .write(|w| w.tdr().bits(byte as u16));
        }

        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // Wait until there's data available
    while usart1.isr.read().rxne().bit_is_clear() {}
    
    // Retrieve the data
    let _byte = usart1.rdr.read().rdr().bits() as u8;

    aux11::bkpt();

    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "The answer is {}", 40 + 2);


    loop {}
}