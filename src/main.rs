#![no_std]
#![no_main]

mod gpio;
mod sysctl;
mod iomux;
use crate::gpio::GpioReg;
use crate::sysctl::SYSCTL_Regs;
use crate::iomux::IOMUX_Regs;
use core::{f64::consts::PI, panic::PanicInfo, time::Duration};

// #[unsafe(link_section = "intvecs")]

unsafe extern "C" {
    static mut __sbss: u32;
    static mut __ebss: u32;
    static mut __sdata: u32;
    static mut __edata: u32;
    static __sidata: u32;
    static _estack: u32;

}

#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    // const GPIOA: *mut GpioReg = 0x400A0000 as *mut GpioReg;
    let GPIOA: &mut GpioReg = GpioReg::from_addr(0x400A0000);
    let GPIOB: &mut GpioReg = GpioReg::from_addr(0x400A2000);
    let GPIOC: &mut GpioReg = GpioReg::from_addr(0x400A4000);

    const IOMUX_BASE: *mut IOMUX_Regs = 0x40428000 as *mut IOMUX_Regs;
    const GPIO_PIN_16: u32 = 0x00010000;
    const GPIO_PIN_23: u32 = 0x800000;
    const GPIO_PIN_10: u32 = 0x400;
    const GPIO_PIN_9: u32 = 0x200;
    const GPIO_PIN_TEST: u32 = 0x200000;
    const SYSCTL: *mut SYSCTL_Regs = 0x400AF000 as *mut SYSCTL_Regs;

    GPIOA.reset();
    GPIOB.reset();
    GPIOC.reset();

    GPIOA.enable_power();
    GPIOB.enable_power();
    GPIOC.enable_power();

    unsafe {
        (*IOMUX_BASE).SECCFG.PINCM[41] = 0x80 | 0x1;
        (*IOMUX_BASE).SECCFG.PINCM[66] = 0x80 | 0x1;
        (*IOMUX_BASE).SECCFG.PINCM[30] = 0x80 | 0x1;
        (*IOMUX_BASE).SECCFG.PINCM[29] = 0x80 | 0x1;
        (*IOMUX_BASE).SECCFG.PINCM[55] = 0x80 | 0x1;
    }

    const LED1: u32 = GPIO_PIN_16;
    const LED2: u32 = GPIO_PIN_10;
    const LED3: u32 = GPIO_PIN_9;
    const LED_TEST: u32 = GPIO_PIN_TEST;

    GPIOA.pin_low(LED1);
    GPIOA.gpio_enable_output(LED1);
    GPIOB.pin_low(LED2);
    GPIOB.gpio_enable_output(LED2);
    GPIOB.pin_low(LED3);
    GPIOB.gpio_enable_output(LED3);

    unsafe {
        (*SYSCTL).SOCLOCK.BORTHRESHOLD = 0;
        (*SYSCTL).SOCLOCK.HSCLKEN &= !(1 as u32);
    }

    GPIOA.pin_low(LED1);
    GPIOB.pin_low(LED2);
    GPIOB.pin_low(LED3);
    // GPIOA.pin_high(LED1);
    // GPIOB.pin_high(LED3);

    // unsafe { core::arch::asm!("ldr r2, =69") };
    // unsafe { core::arch::asm!("ldr r3, =69") };
    // unsafe { core::arch::asm!("ldr r4, =69") };

    let mut brightness = 0.0;
    let mut state = 0.0;

    loop {
        // gpio_toggle(GPIOA, LED1 | LED_TEST);
        // GPIOB
        // LED2 | LED3 = RED
        // LED1 = Green
        // LED2 = YELLOW
        // LED3 = NOTHING
        // GPIOA
        // LED1 = Green
        // LED2 = Green
        // LED3 = Green
        GPIOA.pin_high(LED1);
        delay(Duration::from_millis(
            (((1.0 + sine(state)) / 2.0) * 10.0) as /* hi */ u64,
        ));
        GPIOA.pin_low(LED1);
        delay(Duration::from_millis(
            (((1.0 - sine(state)) / 2.0) * 10.0) as /* hi */ u64,
        ));

        GPIOB.pin_high(LED2);
        delay(Duration::from_millis(
            (((1.0 + sine(state + 2.094)) / 2.0) * 10.0) as /* hi */ u64,
        ));
        GPIOB.pin_low(LED2);
        delay(Duration::from_millis(
            (((1.0 - sine(state + 2.094)) / 2.0) * 10.0) as /* hi */ u64,
        ));

        GPIOB.pin_high(LED3);
        delay(Duration::from_millis(
            (((1.0 + sine(state + 4.188)) / 2.0) * 10.0) as /* hi */ u64,
        ));
        GPIOB.pin_low(LED3);
        delay(Duration::from_millis(
            (((1.0 - sine(state + 4.188)) / 2.0) * 10.0) as /* hi */ u64,
        ));

        // GPIOB.gpio_toggle(LED2);
        // delay(Duration::from_millis(1000));
        state += 0.08;
        if state > 2.0 * PI {
            state = 0.0;
        }
    }
}

#[inline(always)]
fn sine(t: f64) -> f64 {
    if t > PI + PI {
        let t = t - PI - PI;
        16.0 * t * (PI - t) / (5.0 * PI * PI - 4.0 * t * (PI - t))
    } else if t > PI {
        let t = t - PI;
        -16.0 * t * (PI - t) / (5.0 * PI * PI - 4.0 * t * (PI - t))
    } else {
        16.0 * t * (PI - t) / (5.0 * PI * PI - 4.0 * t * (PI - t))
    }
}

// 40% 60%
// 1000 1500
// 400 600

fn delay(duration: Duration) {
    for _ in 0..(duration.as_millis() * 33) {
        unsafe { core::arch::asm!("nop") };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DefaultHandler() -> ! {
    loop {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".intvecs")]
pub static INTERRUPT_VECTORS: [extern "C" fn() -> !; 47] = [
    // unsafe { core::mem::transmute(_estack) },
    Reset,          //
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
    DefaultHandler, // Unfilled IRQ
];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}
