pub enum DL_UART_OVERSAMPLING_RATE {
    DL_UART_OVERSAMPLING_RATE_16X = 0x00000000, // Set oversampling rate to 16x
    DL_UART_OVERSAMPLING_RATE_8X = 0x00008000,  // Set oversampling rate to 8x
    DL_UART_OVERSAMPLING_RATE_3X = 0x00010000,  // Set oversampling rate to 3x.
} /* IrDA, Manchester and DALI are not supported when 3x oversampling is
   * enabled. */

use crate::uart::UART_Regs;
use crate::utils::update_reg;
impl UART_Regs {
    pub fn set_oversampling(&mut self, rate: DL_UART_OVERSAMPLING_RATE) {
        let UART_CTL0_HSE_MASK = 0x00018000;
        update_reg(&mut self.CTL0, rate as u32, UART_CTL0_HSE_MASK);
    }
}
