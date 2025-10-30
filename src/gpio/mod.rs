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
struct GpioGenEvent0Regs {
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
pub struct GpioReg {
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
    GEN_EVENT0: GpioGenEvent0Regs, /* !< (@ 0x00001050) */
    reserved7: u32,
    GEN_EVENT1: GpioGenEvent0Regs, /* !< (@ 0x00001080) */
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

pub fn gpio_init() -> (
    &'static mut GpioReg,
    &'static mut GpioReg,
    &'static mut GpioReg,
) {
    let gpio_a: &mut GpioReg = GpioReg::from_addr(0x400A0000);
    let gpio_b: &mut GpioReg = GpioReg::from_addr(0x400A2000);
    let gpio_c: &mut GpioReg = GpioReg::from_addr(0x400A4000);

    gpio_a.reset();
    gpio_b.reset();
    gpio_c.reset();

    gpio_a.enable_power();
    gpio_b.enable_power();
    gpio_c.enable_power();

    (gpio_a, gpio_b, gpio_c)
}

impl GpioReg {
    pub fn reset(&mut self) {
        const GPIO_RSTCTL_KEY_UNLOCK_W: u32 = 0xB1000000;
        const GPIO_RSTCTL_RESETSTKYCLR_CLR: u32 = 0x00000002;
        const GPIO_RSTCTL_RESETASSERT_ASSERT: u32 = 0x00000001;
        self.GPRCM.RSTCTL = GPIO_RSTCTL_KEY_UNLOCK_W
            | GPIO_RSTCTL_RESETSTKYCLR_CLR
            | GPIO_RSTCTL_RESETASSERT_ASSERT;
    }

    pub fn enable_power(&mut self) {
        const GPIO_PWREN_KEY_UNLOCK_W: u32 = 0x26000000;
        const GPIO_PWREN_ENABLE_ENABLE: u32 = 0x00000001;
        self.GPRCM.PWREN = GPIO_PWREN_KEY_UNLOCK_W | GPIO_PWREN_ENABLE_ENABLE;
    }

    pub fn pin_low(&mut self, pins: u32) {
        self.DOUTCLR31_0 = pins;
    }
    pub fn pin_high(&mut self, pins: u32) {
        self.DOUTSET31_0 = pins;
    }
    pub fn gpio_enable_output(&mut self, pins: u32) {
        self.DOESET31_0 = pins;
    }

    pub fn gpio_toggle(&mut self, pins: u32) {
        self.DOUTTGL31_0 = pins;
    }
}

use crate::utils::MemoryMapped;
impl MemoryMapped for GpioReg {}
