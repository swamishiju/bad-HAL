#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UartOversamplingRate {
    Rate16x = 0x0000_0000, // Set oversampling rate to 16x
    Rate8x = 0x0000_8000,  // Set oversampling rate to 8x
    Rate3x = 0x0001_0000,  // Set oversampling rate to 3x
                           // Note: IrDA, Manchester, and DALI are not supported when 3x oversampling is enabled.
}

use crate::uart::UartRegs;
use crate::utils::update_reg;

impl UartRegs {
    pub fn set_oversampling(&mut self, rate: UartOversamplingRate) {
        const CTL0_HSE_MASK: u32 = 0x0001_8000;
        update_reg(&mut self.ctl0, rate as u32, CTL0_HSE_MASK);
    }
}

// pub enum DL_UART_OVERSAMPLING_RATE {
// DL_UART_OVERSAMPLING_RATE_16X = 0x00000000, // Set oversampling rate to 16x
// DL_UART_OVERSAMPLING_RATE_8X = 0x00008000,  // Set oversampling rate to 8x
// DL_UART_OVERSAMPLING_RATE_3X = 0x00010000,  // Set oversampling rate to 3x.
// } /* IrDA, Manchester and DALI are not supported when 3x oversampling is
// * enabled. */
//
// use crate::uart::UartRegs;
// use crate::utils::update_reg;
// impl UartRegs {
// pub fn set_oversampling(&mut self, rate: DL_UART_OVERSAMPLING_RATE) {
// let UART_CTL0_HSE_MASK = 0x00018000;
// update_reg(&mut self.ctl0, rate as u32, UART_CTL0_HSE_MASK);
// }
// }
