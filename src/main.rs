#![no_std]
#![no_main]

use volatile_register::RW;

#[repr(C)]
struct Gpio {
    pub moder: RW<u32>,     // __IO uint32_t MODER;   /*!< GPIO port mode register,               Address offset: 0x00 */
    pub otyper: RW<u32>,    // __IO uint32_t OTYPER;  /*!< GPIO port output type register,        Address offset: 0x04 */
    pub ospeedr: RW<u32>,   // __IO uint32_t OSPEEDR; /*!< GPIO port output speed register,       Address offset: 0x08 */
    pub pupdr: RW<u32>,     // __IO uint32_t PUPDR;   /*!< GPIO port pull-up/pull-down register,  Address offset: 0x0C */
    pub idr: RW<u32>,       // __IO uint32_t IDR;     /*!< GPIO port input data register,         Address offset: 0x10 */
    pub odr: RW<u32>,       // __IO uint32_t ODR;     /*!< GPIO port output data register,        Address offset: 0x14 */
    pub bsrr: RW<u32>,      // __IO uint32_t BSRR;    /*!< GPIO port bit set/reset register,      Address offset: 0x1A */
    pub lckr: RW<u32>,      // __IO uint32_t LCKR;    /*!< GPIO port configuration lock register, Address offset: 0x1C */
    pub afr: [RW<u32>; 2],  // __IO uint32_t AFR[2];  /*!< GPIO alternate function registers,     Address offset: 0x20-0x24 */
    pub bfr: RW<u32>        // __IO uint32_t BRR;     /*!< GPIO bit reset register,               Address offset: 0x28 */
}

#[repr(C)]
struct Ahb2 {
    pub ahb2: RW<u32>
}

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    led_setup();

    loop {
	led_on();
	spin_a_while();
	led_off();
	spin_a_while();
    }
}

fn led_setup() {
    let ahb2 = 0x4002_104C as *const Ahb2;
    let gpioa = 0x4800_0000 as *const Gpio;

    unsafe {
	// switch on AHB2 clock for GPIOA peripheral
     	(*ahb2).ahb2.modify(|val| val | (0x01 << 0));

	// Set GPIOA_5 pin to digital output
     	(*gpioa).moder.modify(|val| val &! (0b11 << 10));
     	(*gpioa).moder.modify(|val| val | (0b1 << 10));
    }
}

fn led_on() {
    let gpioa = 0x4800_0000 as *const Gpio;

    unsafe {
	// Switch on GPIOA_5
     	(*gpioa).odr.modify(|val| val | (0x01 << 5));
    }
}

fn led_off() {
    let gpioa = 0x4800_0000 as *const Gpio;

    unsafe {
	// Switch off GPIOA_5
     	(*gpioa).odr.modify(|val| val &! (0x01 << 5));
    }
}

fn spin_a_while() {
    for i in 0..1000 {
	asm::nop();
    }
}
