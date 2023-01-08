#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

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
    }

    loop {}
}