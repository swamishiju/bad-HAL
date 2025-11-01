#![no_std]
#![no_main]

mod gpio;
mod iomux;
mod sysctl;
mod uart;

use crate::gpio::GpioReg;
use crate::iomux::IomuxRegs;
use crate::sysctl::SysctlRegs;
use crate::uart::UartRegs;
use crate::uart::clk_config::{UartClock, UartClockConfig, UartClockDivide};
use crate::uart::fifo_config::{RxFifoLevel, TxFifoLevel};
use crate::uart::oversampling_config::UartOversamplingRate;
use crate::uart::uart_config::{
    UartConfig, UartDirection, UartFlowControl, UartMode, UartParity, UartStopBits, UartWordLength,
};
use core::{f64::consts::PI, panic::PanicInfo, time::Duration};

mod utils;
use crate::utils::MemoryMapped;

/*
Defining all the addresses for gpio and uart
*/
const UART0_FREQ: u32 = 32000000;
const UART0_INT_IRQN: u32 = 15;
const UART0_BAUD_RATE: u32 = 9600;

const GPIO_PIN_9: u32 = 0x200;
const GPIO_PIN_10: u32 = 0x00000400;
const GPIO_PIN_11: u32 = 0x00000800;
const GPIO_PIN_16: u32 = 0x00010000;
const GPIO_PIN_23: u32 = 0x800000;
const GPIO_PIN_TEST: u32 = 0x200000;

unsafe extern "C" {
    static mut __sbss: u32;
    static mut __ebss: u32;
    static mut __sdata: u32;
    static mut __edata: u32;
    static __sidata: u32;
    static _estack: u32;

}

/*
Does some pwm and makes rainbow led

*/
fn rainbow_led(
    GPIOA: &mut GpioReg,
    GPIOB: &mut GpioReg,
    GPIOC: &mut GpioReg,
    state: &mut f64,
) -> () {
    const LED1: u32 = GPIO_PIN_16;
    const LED2: u32 = GPIO_PIN_10;
    const LED3: u32 = GPIO_PIN_9;
    const LED_TEST: u32 = GPIO_PIN_TEST;

    GPIOA.pin_low(LED1);
    GPIOA.enable_output(LED1);
    GPIOB.pin_low(LED2);
    GPIOB.enable_output(LED2);
    GPIOB.pin_low(LED3);
    GPIOB.enable_output(LED3);

    GPIOA.pin_low(LED1);
    GPIOB.pin_low(LED2);
    GPIOB.pin_low(LED3);

    GPIOA.pin_high(LED1);
    delay(Duration::from_millis(
        (((1.0 + sine(*state)) / 2.0) * 10.0) as /* hi */ u64,
    ));
    GPIOA.pin_low(LED1);
    delay(Duration::from_millis(
        (((1.0 - sine(*state)) / 2.0) * 10.0) as /* hi */ u64,
    ));

    GPIOB.pin_high(LED2);
    delay(Duration::from_millis(
        (((1.0 + sine(*state + 2.094)) / 2.0) * 10.0) as /* hi */ u64,
    ));
    GPIOB.pin_low(LED2);
    delay(Duration::from_millis(
        (((1.0 - sine(*state + 2.094)) / 2.0) * 10.0) as /* hi */ u64,
    ));

    GPIOB.pin_high(LED3);
    delay(Duration::from_millis(
        (((1.0 + sine(*state + 4.188)) / 2.0) * 10.0) as /* hi */ u64,
    ));
    GPIOB.pin_low(LED3);
    delay(Duration::from_millis(
        (((1.0 - sine(*state + 4.188)) / 2.0) * 10.0) as /* hi */ u64,
    ));

    // GPIOB.gpio_toggle(LED2);
    // delay(Duration::from_millis(1000));
    *state += 0.08;
    if *state > 2.0 * PI {
        *state = 0.0;
    }
}

fn main() -> ! {
    let (GPIOA, GPIOB, GPIOC) = gpio::gpio_init(); // initializes the gpio ports and enables power

    let IOMUX: &mut IomuxRegs = IomuxRegs::from_addr(0x40428000);
    let SYSCTL: &mut SysctlRegs = SysctlRegs::from_addr(0x400AF000);

    let UART0: &mut UartRegs = UartRegs::from_addr(0x40108000);

    UART0.reset();
    UART0.enable_power();

    IOMUX.sec_cfg.pincm[24] = 0x80 | 0x2;
    IOMUX.sec_cfg.pincm[25] = 0x80 | 0x2;

    IOMUX.sec_cfg.pincm[41] = 0x80 | 0x1;
    IOMUX.sec_cfg.pincm[66] = 0x80 | 0x1;
    IOMUX.sec_cfg.pincm[30] = 0x80 | 0x1;
    IOMUX.sec_cfg.pincm[29] = 0x80 | 0x1;
    IOMUX.sec_cfg.pincm[55] = 0x80 | 0x1;

    uart_init(UART0);

    SYSCTL.soc_lock.hsclk_en &= !(1 as u32);
    SYSCTL.soc_lock.bor_threshold = 0;
    let mut state = 0.0;
    loop {
        UART0.transmit_str("rk was here");
        rainbow_led(GPIOA, GPIOB, GPIOC, &mut state);
    }
}

fn uart_init(uart: &mut UartRegs) {
    let clock_config = UartClockConfig {
        source: UartClock::BusClk,
        divider: UartClockDivide::Div1,
    };

    let uart_config = UartConfig {
        mode: UartMode::Normal,
        direction: UartDirection::TxRx,
        flow_control: UartFlowControl::None,
        parity: UartParity::None,
        word_length: UartWordLength::Bits8,
        stop_bits: UartStopBits::One,
    };

    uart.set_clock_config(clock_config);
    uart.uart_init(uart_config);
    uart.set_oversampling(UartOversamplingRate::Rate16x);
    uart.set_baud_rate_divisor(208, 21);

    uart.enable_fifos();
    uart.set_rx_fifo_threshold(RxFifoLevel::Full);
    uart.set_tx_fifo_threshold(TxFifoLevel::Empty);

    uart.enable();
}

#[unsafe(no_mangle)]
pub extern "C" fn Reset() {
    main();
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

fn delay(duration: Duration) {
    for _ in 0..(duration.as_millis() * 33) {
        unsafe { core::arch::asm!("nop") };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DefaultHandler() {
    loop {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".intvecs")]
pub static INTERRUPT_VECTORS: [extern "C" fn(); 47] = [
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
