use crate::uart::UartRegs;
use crate::utils::update_reg;
impl UartRegs {
    pub fn set_baud_rate_divisor(&mut self, integer_divisor: u32, fractional_divisor: u32) {
        const UART_IBRD_DIVINT_MASK: u32 = 0x0000FFFF;
        const UART_FBRD_DIVFRAC_MASK: u32 = 0x0000003F;
        const UART_LCRH_BRK_MASK: u32 = 0x00000001;
        update_reg(&mut self.ibrd, integer_divisor, UART_IBRD_DIVINT_MASK);
        update_reg(&mut self.fbrd, fractional_divisor, UART_FBRD_DIVFRAC_MASK);

        // When updating the baud-rate divisor (UARTIBRD or UARTIFRD),
        // the LCRH register must also be written to (any bit in LCRH can
        // be written to for updating the baud-rate divisor).

        let value = self.lcrh & UART_LCRH_BRK_MASK;
        update_reg(&mut self.lcrh, value, UART_LCRH_BRK_MASK);
    }
}
