#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM};

/// Print the current contents of ODR
fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            // Read bits 0..15 of the ODR register
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn main() -> ! {
    let (mut itm, gpioe) = aux7::init();

    unsafe {
        // This address points to a register, a register is a special region of memory that controls a peripheral
        // A peripheral is a piece of electronics that sits right next to the processor within the microcontroller
        // package and provides the processor with extra functionality. This register controls the GPIO (general
        // purpose input/output) pins, and can be used to drive each of those pins low or high

        // A pin is an electrical contact. Our microcontroller has several of them and some of them are connected
        // to LEDs. An LED, a Light Emitting Diode, will only emit light when voltage is applied to it with a 
        // certain polarity. The microcontroller's pins are connected to the LEDs with the right polarity. All that
        // we have to do is output some non-zero voltage through the pin to turn the LED on. The pins attached to
        // the LEDs are configured as digital output and can only output two different voltage levels, "low", 0
        // volts, or "high", 3 volts. A "high" voltage level will turn the LED on whereas a "low" voltage level 
        // will turn it off. These "low" and "high" states map directly to the concept of digital logic. Low is 0 
        // or false and high is 1 or true. This is why this pin configuration is known as digital output.
        
        // The microcontroller has several pins, and for convenience, these pins are grouped in ports of 16 pins. 
        // Each port is named with a letter: Port A, Port B, etc. and the pins within each port are named with 
        // numbers from 0 to 15
        // Each peripheral has a register block associated to it. A register block is a collection of registers 
        // allocated in contiguous memory. The address at which the register block starts is known as its base 
        // address. We need to figure out what's the base address of the GPIOE peripheral.
        // In section 3.2.2 Memory map and register boundary addresses (pg 51) of the reference manual it says the
        // base address of the GPIOE register block is 0x48001000. In the documentation each peripheral has its own
        // section and each of these sections ends with a table of the registers that the peripherals register block
        // contains. 
        // In section 11.4.12 GPIO register map (pg. 243) of the regerence manual, we can look at the table. BSRR
        // is the register we will be using to set/reset. Its offset value is 0x18 from the base address of the 
        // GPIOE. We can look up BSRR in the reference manual. GPIO Registers -> GPIO port bit set/reset register
        // (GPIOx_BSRR). The documentation says that this register is write only, so reading its value in lets say
        // gdb will always return 0x00000000 even after we set it.
        const GPIOE_BSRR: u32 = 0x48001018;

        // The other thing that the documentation says is that the bits 0 to 15 can be used to set the corresponding
        // pin. That is bit 0 sets the pin 0, set means outputtting a high value on the pin.
        // Turn on the "North" LED (red)
        // In the manual it says `LD3, the North LED, is connected to the pin PE9. PE9 is the short form of: Pin 9
        // on Port E`.
        // So what this is doing is writing 1 << 9 (9th bit to 1) to BSRR, and that sets PE9 to high
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        // This LED (LD7) is connected to the pin PE11 (pin 11 on port E) 
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // The documentation also says that bits 16 to 31 can be used to reset the corresponding pin. In this case,
        // bit 16 resets the pin number 0. Reseting means outputting a low value on the pin
        // Turn off the "North" LED
        // So what this is doing is writing 1 << 25 to BSSR, and that sets PE9 to low
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);

        // This all works when we run it with debug optimizations but it doesnt work with release
        // optimizations. In short this is because reads/writes to registers are special. We just
        // wrote four different values to the same register, if you didnt know that address was a
        // register the logic may have been simpilfied to write just the final value, 1 << (11 + 16)
        // into the register. LLVM, the compilers backend / optimizer does not know we are dealing
        // with a register and will merge the writes. To prevent LLVM from misoptimizing our 
        // program, we can use volatile operations instead of plain reads/writes:
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));

        // Not all the peripheral memory can be accessed, this address is close to the GPIOE_BSSR
        // address we used before but this address is invalid. Invalid in the sense that there's
        // no register at this address. 
        // ptr::read_volatile(0x48001800 as *const u32);
        // We tried to do an invalid operation, reading memory that doesn't exist, so the processor
        // raised an exception, a hardware exception. In most cases, exceptions are raised when the
        // processor attempts to perform an invalid operation. Exceptions break the normal control
        // flow of a program and force the processor to execute an exception handler, which is just
        // a function/subroutine. There are different kinds of exceptions. Each kind of exception
        // is raised by different conditions and each one is handled by a different exception 
        // handler. The aux7 crate depends on the cortex-m-rt crate which defines a default `hard
        // fault` handler, named HardFault, that handles the "invalid memory address" exception.
        
        // BSRR is not the only register that can control the pins of port E. The ODR register also
        // lets you change the value of the pins. Furthermore, ODR also lets you retrieve the 
        // current output status of port E. (ODR is documented on page 239 of the ref. manual)
        // Print the initial contents of ODR
        iprint_odr(&mut itm);

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);

        // Type safe manipulation
        // The last register we were working with, ODR, had this in its documentation: "Bits 32:16
        // Reserved, must be kept at reset value". We are not supposed to write to those bits of
        // the register or bad stuff may happen. There's also the fact the registers have different
        // read/write permissions. Some of them are write only, other can be read and written to
        // and there must be others that are read only. Finnaly directly working with hexadecimal
        // addresses is error prone. You already saw that trying to access an invalid memory address
        // causes an exception which disrupts the execution of our program. It would be a better 
        // idea to manipulate these registers in a safe manner using an API. Ideally, the API 
        // should encode these three points mentioned: No messing around with the actual addresses
        // should respect read/write permissions, and should prevent modification of the reserved
        // parts of a register. Well there actually is an API for this, the aux7::init() actaully
        // returns a value that provides a type safe API to manipulate the registers of the GPIOE
        // peripheral.
        
        // As you may remember: a group of registers associated to a peripheral is called a 
        // register block, and it's located in a contigous region of memory. In this type safe API
        // each register block is modeled as a struct where each of its fields represents a 
        // register. Each register field is a different newtype over e.g. `u32` that exposes a 
        // combination of the following methods, `read`, `write`, or `modify` according to its 
        // read/write permissions. Finnaly, these methods don't take primitive values like `u32`, 
        // instead they take yet another newtype that can be constructed using the builder pattern
        // and that prevent the modification of the reserved parts of the register.
        // Here is the example of turning on and off the LED's ported to that API
        gpioe.bsrr.write(|w| w.bs9().set_bit());
        gpioe.bsrr.write(|w| w.bs11().set_bit());
        gpioe.bsrr.write(|w| w.br9().set_bit());
        gpioe.bsrr.write(|w| w.br11().set_bit());
    }

    loop {}
}