#![no_std]
#![no_main]

mod gpio;
mod iomux;
mod sysctl;
mod uart;

use crate::gpio::GpioReg;
use crate::iomux::IOMUX_Regs;
use crate::sysctl::SYSCTL_Regs;
use crate::uart::clk_config::{DL_UART_ClockConfig, DL_UART_CLOCK, DL_UART_CLOCK_DIVIDE_RATIO};
use crate::uart::uart_config::{
    DL_UART_Config, DL_UART_DIRECTION, DL_UART_FLOW_CONTROL, DL_UART_MODE, DL_UART_PARITY,
    DL_UART_STOP_BITS, DL_UART_WORD_LENGTH,
};
use crate::uart::oversampling_config::{DL_UART_OVERSAMPLING_RATE};
use crate::uart::fifo_config::{DL_UART_TX_FIFO_LEVEL, DL_UART_RX_FIFO_LEVEL};
use crate::uart::UART_Regs;
use core::{f64::consts::PI, panic::PanicInfo, time::Duration};

mod utils;
use crate::utils::MemoryMapped;

unsafe extern "C" {
    static mut __sbss: u32;
    static mut __ebss: u32;
    static mut __sdata: u32;
    static mut __edata: u32;
    static __sidata: u32;
    static _estack: u32;

}

fn UART_init(uart: &mut UART_Regs) {
    let clock_config = DL_UART_ClockConfig {
        clockSel: DL_UART_CLOCK::DL_UART_CLOCK_BUSCLK,
        divideRatio: DL_UART_CLOCK_DIVIDE_RATIO::DL_UART_CLOCK_DIVIDE_RATIO_1,
    };

    let uart_config = DL_UART_Config {
        mode: DL_UART_MODE::DL_UART_MODE_NORMAL,
        direction: DL_UART_DIRECTION::DL_UART_DIRECTION_TX_RX,
        flowControl: DL_UART_FLOW_CONTROL::DL_UART_FLOW_CONTROL_NONE,
        parity: DL_UART_PARITY::DL_UART_PARITY_NONE,
        wordLength: DL_UART_WORD_LENGTH::DL_UART_WORD_LENGTH_8_BITS,
        stopBits: DL_UART_STOP_BITS::DL_UART_STOP_BITS_ONE,
    };

    uart.set_clk_config(clock_config);
    uart.uart_init(uart_config);
    uart.set_oversampling(DL_UART_OVERSAMPLING_RATE::DL_UART_OVERSAMPLING_RATE_16X);
    uart.set_baud_rate_divisor(208, 21);

    uart.enable_fifos();
    uart.set_rx_fifo_threshold(DL_UART_RX_FIFO_LEVEL::DL_UART_RX_FIFO_LEVEL_FULL);
    uart.set_tx_fifo_threshold(DL_UART_TX_FIFO_LEVEL::DL_UART_TX_FIFO_LEVEL_EMPTY);

    uart.enable();
}

#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    let GPIOA: &mut GpioReg = GpioReg::from_addr(0x400A0000);
    let GPIOB: &mut GpioReg = GpioReg::from_addr(0x400A2000);
    let GPIOC: &mut GpioReg = GpioReg::from_addr(0x400A4000);

    let IOMUX: &mut IOMUX_Regs = IOMUX_Regs::from_addr(0x40428000);
    let SYSCTL: &mut SYSCTL_Regs = SYSCTL_Regs::from_addr(0x400AF000);

    let UART0: &mut UART_Regs = UART_Regs::from_addr(0x40108000);
    const UART0_freq: u32 = 32000000;
    const UART0_INT_IRQN: u32 = 15;
    const UART0_BAUD_RATE: u32 = 9600;

    const GPIO_PIN_9: u32 = 0x200;
    const GPIO_PIN_10: u32 = 0x00000400;
    const GPIO_PIN_11: u32 = 0x00000800;
    const GPIO_PIN_16: u32 = 0x00010000;
    const GPIO_PIN_23: u32 = 0x800000;
    const GPIO_PIN_TEST: u32 = 0x200000;

    GPIOA.reset();
    GPIOB.reset();
    GPIOC.reset();
    UART0.reset();

    GPIOA.enable_power();
    GPIOB.enable_power();
    GPIOC.enable_power();
    UART0.enable_power();

    IOMUX.SECCFG.PINCM[24] = 0x80 | 0x2;
    IOMUX.SECCFG.PINCM[25] = 0x80 | 0x2;

    IOMUX.SECCFG.PINCM[41] = 0x80 | 0x1;
    IOMUX.SECCFG.PINCM[66] = 0x80 | 0x1;
    IOMUX.SECCFG.PINCM[30] = 0x80 | 0x1;
    IOMUX.SECCFG.PINCM[29] = 0x80 | 0x1;
    IOMUX.SECCFG.PINCM[55] = 0x80 | 0x1;

    const LED1: u32 = GPIO_PIN_16;
    const LED2: u32 = GPIO_PIN_10;
    const LED3: u32 = GPIO_PIN_9;
    const LED_TEST: u32 = GPIO_PIN_TEST;

    UART_init(UART0);
    UART0.transmit(&0x61);
    UART0.transmit(&0x61);
    UART0.transmit(&0x61);
    UART0.transmit(&0x61);
    GPIOA.pin_low(LED1);
    GPIOA.gpio_enable_output(LED1);
    GPIOB.pin_low(LED2);
    GPIOB.gpio_enable_output(LED2);
    GPIOB.pin_low(LED3);
    GPIOB.gpio_enable_output(LED3);

    SYSCTL.SOCLOCK.BORTHRESHOLD = 0;
    SYSCTL.SOCLOCK.HSCLKEN &= !(1 as u32);

    GPIOA.pin_low(LED1);
    GPIOB.pin_low(LED2);
    GPIOB.pin_low(LED3);

    let mut brightness = 0.0;
    let mut state = 0.0;

    loop {
        GPIOA.pin_high(LED1);
        delay(Duration::from_millis(
            (((1.0 + sine(state)) / 2.0) * 10.0) as /* hi */ u64,
        ));
        gpioA.pin_low(LED1);
        delay(Duration::from_millis(
            (((1.0 - sine(state)) / 2.0) * 10.0) as /* hi */ u64,
        ));

        gpioB.pin_high(LED2);
        delay(Duration::from_millis(
            (((1.0 + sine(state + 2.094)) / 2.0) * 10.0) as /* hi */ u64,
        ));
        gpioB.pin_low(LED2);
        delay(Duration::from_millis(
            (((1.0 - sine(state + 2.094)) / 2.0) * 10.0) as /* hi */ u64,
        ));

        gpioB.pin_high(LED3);
        delay(Duration::from_millis(
            (((1.0 + sine(state + 4.188)) / 2.0) * 10.0) as /* hi */ u64,
        ));
        gpioB.pin_low(LED3);
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
