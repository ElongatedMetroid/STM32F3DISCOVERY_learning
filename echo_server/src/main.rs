#![no_main]
#![no_std]

use core::fmt::{self, Write};

use aux11::{entry, usart1, iprintln};
use heapless::Vec;

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

impl SerialPort {
    pub fn read_byte(&self) -> u8 {
        while self.usart1.isr.read().rxne().bit_is_clear() {}
        self.usart1.rdr.read().rdr().bits() as u8
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            // Wait until its safe to write to TDR
            while self.usart1.isr.read().txe().bit_is_clear() {}

            // Write a byte
            self.usart1.tdr.write(|w| w.tdr().bits(byte as u16));
        }

        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut itm) = aux11::init();

    // Echo server
    // loop {
    //     while usart1.isr.read().rxne().bit_is_clear() {}

    //     let byte = usart1.rdr.read().rdr().bits() as u8;

    //     while usart1.isr.read().txe().bit_is_clear() {}

    //     usart1.tdr.write(|w| w.tdr().bits(byte as u16));
    // }

    // Respond to the client with the reverse of the text they sent. The server responds each time
    // they press the enter key.
    let mut serial = SerialPort { usart1 };
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        let mut byte = 0;
        while byte != '\n' as u8 {
            byte = serial.read_byte();

            match buffer.push(byte) {
                Ok(_) => (),
                Err(byte) => {
                    uprintln!(serial, "Error: buffer is full, cannot write {}", byte);
                    continue;
                }
            }
            iprintln!(&mut itm.stim[0], "{} ({})\n{:?}", byte as char, byte, buffer);
        }

        buffer.reverse();

        for byte in &buffer {
            uprint!(serial, "{}", *byte as char);
        }
        uprintln!(serial, "");
    }
}
