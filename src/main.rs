#![no_std]
#![no_main]

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

#[repr(C)]
struct GpioGPRCMRegs {
    PWREN: u32,
    RSTCTL: u32,
    reserved0: [u32; 3],
    STAT: u32,
}

#[repr(C)]
struct GpioCpuIntRegs {
    IIDX: u32, /* !< (@ 0x00001020) Interrupt index */
    reserved0: u32,
    IMASK: u32, /* !< (@ 0x00001028) Interrupt mask */
    reserved1: u32,
    RIS: u32, /* !< (@ 0x00001030) Raw interrupt status */
    reserved2: u32,
    MIS: u32, /* !< (@ 0x00001038) Masked interrupt status */
    reserved3: u32,
    ISET: u32, /* !< (@ 0x00001040) Interrupt set */
    reserved4: u32,
    ICLR: u32, /* !< (@ 0x00001048) Interrupt clear */
}

#[repr(C)]
struct GPIO_GEN_EVENT0_Regs {
    IIDX: u32, /* !< (@ 0x00001050) Interrupt index */
    reserved0: u32,
    IMASK: u32, /* !< (@ 0x00001058) Interrupt mask */
    reserved1: u32,
    RIS: u32, /* !< (@ 0x00001060) Raw interrupt status */
    reserved2: u32,
    MIS: u32, /* !< (@ 0x00001068) Masked interrupt status */
    reserved3: u32,
    ISET: u32, /* !< (@ 0x00001070) Interrupt set */
    reserved4: u32,
    ICLR: u32, /* !< (@ 0x00001078) Interrupt clear */
}

#[repr(C)]
struct GpioReg {
    reserved0: [u32; 256],
    FSUB_0: u32, /* !< (@ 0x00000400) Subsciber Port 0 */
    FSUB_1: u32, /* !< (@ 0x00000404) Subscriber Port 1 */
    reserved1: [u32; 15],
    FPUB_0: u32, /* !< (@ 0x00000444) Publisher Port 0 */
    FPUB_1: u32, /* !< (@ 0x00000448) Publisher Port 1 */
    reserved2: [u32; 237],
    GPRCM: GpioGPRCMRegs, /* !< (@ 0x00000800) */
    reserved3: [u32; 510],
    CLKOVR: u32, /* !< (@ 0x00001010) Clock Override */
    reserved4: u32,
    PDBGCTL: u32, /* !< (@ 0x00001018) Peripheral Debug Control */
    reserved5: u32,
    CPU_INT: GpioCpuIntRegs, /* !< (@ 0x00001020) */
    reserved6: u32,
    GEN_EVENT0: GPIO_GEN_EVENT0_Regs, /* !< (@ 0x00001050) */
    reserved7: u32,
    GEN_EVENT1: GPIO_GEN_EVENT0_Regs, /* !< (@ 0x00001080) */
    reserved8: [u32; 13],
    EVT_MODE: u32, /* !< (@ 0x000010E0) Event Mode */
    reserved9: [u32; 6],
    DESC: u32, /* !< (@ 0x000010FC) Module Description */
    reserved10: [u32; 64],
    DOUT3_0: u32,   /* !< (@ 0x00001200) Data output 3 to 0 */
    DOUT7_4: u32,   /* !< (@ 0x00001204) Data output 7 to 4 */
    DOUT11_8: u32,  /* !< (@ 0x00001208) Data output 11 to 8 */
    DOUT15_12: u32, /* !< (@ 0x0000120C) Data output 15 to 12 */
    DOUT19_16: u32, /* !< (@ 0x00001210) Data output 19 to 16 */
    DOUT23_20: u32, /* !< (@ 0x00001214) Data output 23 to 20 */
    DOUT27_24: u32, /* !< (@ 0x00001218) Data output 27 to 24 */
    DOUT31_28: u32, /* !< (@ 0x0000121C) Data output 31 to 28 */
    reserved11: [u32; 24],
    DOUT31_0: u32, /* !< (@ 0x00001280) Data output 31 to 0 */
    reserved12: [u32; 3],
    DOUTSET31_0: u32, /* !< (@ 0x00001290) Data output set 31 to 0 */
    reserved13: [u32; 3],
    DOUTCLR31_0: u32, /* !< (@ 0x000012A0) Data output clear 31 to 0 */
    reserved14: [u32; 3],
    DOUTTGL31_0: u32, /* !< (@ 0x000012B0) Data output toggle 31 to 0 */
    reserved15: [u32; 3],
    DOE31_0: u32, /* !< (@ 0x000012C0) Data output enable 31 to 0 */
    reserved16: [u32; 3],
    DOESET31_0: u32, /* !< (@ 0x000012D0) Data output enable set 31 to 0 */
    reserved17: [u32; 3],
    DOECLR31_0: u32, /* !< (@ 0x000012E0) Data output enable clear 31 to 0 */
    reserved18: [u32; 7],
    DIN3_0: u32,   /* !< (@ 0x00001300) Data input 3 to 0 */
    DIN7_4: u32,   /* !< (@ 0x00001304) Data input 7 to 4 */
    DIN11_8: u32,  /* !< (@ 0x00001308) Data input 11 to 8 */
    DIN15_12: u32, /* !< (@ 0x0000130C) Data input 15 to 12 */
    DIN19_16: u32, /* !< (@ 0x00001310) Data input 19 to 16 */
    DIN23_20: u32, /* !< (@ 0x00001314) Data input 23 to 20 */
    DIN27_24: u32, /* !< (@ 0x00001318) Data input 27 to 24 */
    DIN31_28: u32, /* !< (@ 0x0000131C) Data input 31 to 28 */
    reserved19: [u32; 24],
    DIN31_0: u32, /* !< (@ 0x00001380) Data input 31 to 0 */
    reserved20: [u32; 3],
    POLARITY15_0: u32, /* !< (@ 0x00001390) Polarity 15 to 0 */
    reserved21: [u32; 3],
    POLARITY31_16: u32, /* !< (@ 0x000013A0) Polarity 31 to 16 */
    reserved22: [u32; 23],
    CTL: u32,      /* !< (@ 0x00001400) FAST WAKE GLOBAL EN */
    FASTWAKE: u32, /* !< (@ 0x00001404) FAST WAKE ENABLE */
    reserved23: [u32; 62],
    SUB0CFG: u32, /* !< (@ 0x00001500) Subscriber 0 configuration */
    reserved24: u32,
    FILTEREN15_0: u32,  /* !< (@ 0x00001508) Filter Enable 15 to 0 */
    FILTEREN31_16: u32, /* !< (@ 0x0000150C) Filter Enable 31 to 16 */
    DMAMASK: u32,       /* !< (@ 0x00001510) DMA Write MASK */
    reserved25: [u32; 3],
    SUB1CFG: u32, /* !< (@ 0x00001520) Subscriber 1 configuration */
}

impl GpioReg {
    fn reset(&mut self) {
        const GPIO_RSTCTL_KEY_UNLOCK_W: u32 = 0xB1000000;
        const GPIO_RSTCTL_RESETSTKYCLR_CLR: u32 = 0x00000002;
        const GPIO_RSTCTL_RESETASSERT_ASSERT: u32 = 0x00000001;
        self.GPRCM.RSTCTL = GPIO_RSTCTL_KEY_UNLOCK_W
            | GPIO_RSTCTL_RESETSTKYCLR_CLR
            | GPIO_RSTCTL_RESETASSERT_ASSERT;
    }

    fn enable_power(&mut self) {
        const GPIO_PWREN_KEY_UNLOCK_W: u32 = 0x26000000;
        const GPIO_PWREN_ENABLE_ENABLE: u32 = 0x00000001;
        self.GPRCM.PWREN = GPIO_PWREN_KEY_UNLOCK_W | GPIO_PWREN_ENABLE_ENABLE;
    }

    fn pin_low(&mut self, pins: u32) {
        self.DOUTCLR31_0 = pins;
    }
    fn pin_high(&mut self, pins: u32) {
        self.DOUTSET31_0 = pins;
    }
    fn gpio_enable_output(&mut self, pins: u32) {
        self.DOESET31_0 = pins;
    }

    fn gpio_toggle(&mut self, pins: u32) {
        self.DOUTTGL31_0 = pins;
    }

    #[inline(always)]
    fn from_addr(addr: u32) -> &'static mut Self {
        unsafe { &mut *(addr as *mut Self) }
    }
}

// Option<u32>
// A: u32
// tag: u32
// Some(69) => { 69, 1 }
// None => {garbage, 0}

#[repr(C)]
struct IOMUX_SECCFG_Regs {
    reserved0: u32,
    PINCM: [u32; 251],
}

#[repr(C)]
struct IOMUX_Regs {
    SECCFG: IOMUX_SECCFG_Regs,
}

#[repr(C)]
struct SYSCTL_SOCLOCK_Regs {
    reserved0: [u32; 8],
    IIDX: u32, /* !< (@ 0x00001020) SYSCTL interrupt index */
    reserved1: u32,
    IMASK: u32, /* !< (@ 0x00001028) SYSCTL interrupt mask */
    reserved2: u32,
    RIS: u32, /* !< (@ 0x00001030) SYSCTL raw interrupt status */
    reserved3: u32,
    MIS: u32, /* !< (@ 0x00001038) SYSCTL masked interrupt status */
    reserved4: u32,
    ISET: u32, /* !< (@ 0x00001040) SYSCTL interrupt set */
    reserved5: u32,
    ICLR: u32, /* !< (@ 0x00001048) SYSCTL interrupt clear */
    reserved6: u32,
    NMIIIDX: u32, /* !< (@ 0x00001050) NMI interrupt index */
    reserved7: [u32; 3],
    NMIRIS: u32, /* !< (@ 0x00001060) NMI raw interrupt status */
    reserved8: [u32; 3],
    NMIISET: u32, /* !< (@ 0x00001070) NMI interrupt set */
    reserved9: u32,
    NMIICLR: u32, /* !< (@ 0x00001078) NMI interrupt clear */
    reserved10: [u32; 33],
    SYSOSCCFG: u32,   /* !< (@ 0x00001100) SYSOSC configuration */
    MCLKCFG: u32,     /* !< (@ 0x00001104) Main clock (MCLK) configuration */
    HSCLKEN: u32,     /* !< (@ 0x00001108) High-speed clock (HSCLK) source enable/disable */
    HSCLKCFG: u32,    /* !< (@ 0x0000110C) High-speed clock (HSCLK) source selection */
    HFCLKCLKCFG: u32, /* !< (@ 0x00001110) High-frequency clock (HFCLK) configuration */
    LFCLKCFG: u32,    /* !< (@ 0x00001114) Low frequency crystal oscillator (LFXT)
                      configuration */
    reserved11: [u32; 8],
    GENCLKCFG: u32, /* !< (@ 0x00001138) General clock configuration */
    GENCLKEN: u32,  /* !< (@ 0x0000113C) General clock enable control */
    PMODECFG: u32,  /* !< (@ 0x00001140) Power mode configuration */
    reserved12: [u32; 3],
    FCC: u32, /* !< (@ 0x00001150) Frequency clock counter (FCC) count */
    reserved13: [u32; 7],
    SYSOSCTRIMUSER: u32, /* !< (@ 0x00001170) SYSOSC user-specified trim */
    reserved14: u32,
    SRAMBOUNDARY: u32, /* !< (@ 0x00001178) SRAM Write Boundary */
    reserved15: u32,
    SYSTEMCFG: u32, /* !< (@ 0x00001180) System configuration */
    reserved16: [u32; 31],
    WRITELOCK: u32,  /* !< (@ 0x00001200) SYSCTL register write lockout */
    CLKSTATUS: u32,  /* !< (@ 0x00001204) Clock module (CKM) status */
    SYSSTATUS: u32,  /* !< (@ 0x00001208) System status information */
    DEDERRADDR: u32, /* !< (@ 0x0000120C) Memory DED Address */
    reserved17: [u32; 4],
    RSTCAUSE: u32, /* !< (@ 0x00001220) Reset cause */
    reserved18: [u32; 55],
    RESETLEVEL: u32, /* !< (@ 0x00001300) Reset level for application-triggered reset
                     command */
    RESETCMD: u32, /* !< (@ 0x00001304) Execute an application-triggered reset command */
    BORTHRESHOLD: u32, /* !< (@ 0x00001308) BOR threshold selection */
    BORCLRCMD: u32, /* !< (@ 0x0000130C) Set the BOR threshold */
    SYSOSCFCLCTL: u32, /* !< (@ 0x00001310) SYSOSC frequency correction loop (FCL) ROSC enable */
    LFXTCTL: u32,  /* !< (@ 0x00001314) LFXT and LFCLK control */
    EXLFCTL: u32,  /* !< (@ 0x00001318) LFCLK_IN and LFCLK control */
    SHDNIOREL: u32, /* !< (@ 0x0000131C) SHUTDOWN IO release control */
    EXRSTPIN: u32, /* !< (@ 0x00001320) Disable the reset function of the NRST pin */
    SYSSTATUSCLR: u32, /* !< (@ 0x00001324) Clear sticky bits of SYSSTATUS */
    SWDCFG: u32,   /* !< (@ 0x00001328) Disable the SWD function on the SWD pins */
    FCCCMD: u32,   /* !< (@ 0x0000132C) Frequency clock counter start capture */
    reserved19: [u32; 52],
    SHUTDNSTORE0: u32, /* !< (@ 0x00001400) Shutdown storage memory (byte 0) */
    SHUTDNSTORE1: u32, /* !< (@ 0x00001404) Shutdown storage memory (byte 1) */
    SHUTDNSTORE2: u32, /* !< (@ 0x00001408) Shutdown storage memory (byte 2) */
    SHUTDNSTORE3: u32, /* !< (@ 0x0000140C) Shutdown storage memory (byte 3) */
}

#[repr(C)]
struct SYSCTL_SECCFG_Regs {
    FWEPROTMAIN: u32, /* !< (@ 0x00003000) 1 Sector Write-Erase per bit starting at address
                      0x0 of flash */
    reserved0: [u32; 5],
    FRXPROTMAINSTART: u32, /* !< (@ 0x00003018) Flash RX Protection Start Address */
    FRXPROTMAINEND: u32,   /* !< (@ 0x0000301C) Flash RX Protection End Address */
    FIPPROTMAINSTART: u32, /* !< (@ 0x00003020) Flash IP Protection Start Address */
    FIPPROTMAINEND: u32,   /* !< (@ 0x00003024) Flash IP Protection End Address */
    reserved1: [u32; 4],
    FLBANKSWPPOLICY: u32, /* !< (@ 0x00003038) Flash Bank Swap Policy */
    FLBANKSWP: u32,       /* !< (@ 0x0000303C) Flash MAIN bank address swap */
    reserved2: u32,
    FWENABLE: u32,  /* !< (@ 0x00003044) Security Firewall Enable Register */
    SECSTATUS: u32, /* !< (@ 0x00003048) Security Configuration  status */
    reserved3: [u32; 5],
    INITDONE: u32, /* !< (@ 0x00003060) INITCODE PASS */
}

#[repr(C)]
struct SYSCTL_Regs {
    reserved0: [u32; 1024],
    SOCLOCK: SYSCTL_SOCLOCK_Regs, /* !< (@ 0x00001000) SYSCTL SOCLOCK Region */
    reserved1: [u32; 1788],
    SECCFG: SYSCTL_SECCFG_Regs, /* !< (@ 0x00003000) SYSCTL SECCFG Region */
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
