use crate::uart::UartRegs;
use crate::utils::update_reg;
impl UartRegs {
    pub fn set_baud_rate_divisor(&mut self, integerDivisor: u32, fractionalDivisor: u32) {
        let UART_IBRD_DIVINT_MASK = 0x0000FFFF;
        let UART_FBRD_DIVFRAC_MASK = 0x0000003F;
        let UART_LCRH_BRK_MASK = 0x00000001;
        update_reg(&mut self.ibrd, integerDivisor, UART_IBRD_DIVINT_MASK);
        update_reg(&mut self.fbrd, fractionalDivisor, UART_FBRD_DIVFRAC_MASK);

        // When updating the baud-rate divisor (UARTIBRD or UARTIFRD),
        // the LCRH register must also be written to (any bit in LCRH can
        // be written to for updating the baud-rate divisor).

        let value = self.lcrh & UART_LCRH_BRK_MASK;
        update_reg(&mut self.lcrh, value, UART_LCRH_BRK_MASK);
    }
}
