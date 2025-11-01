pub enum DL_UART_TX_FIFO_LEVEL {
    DL_UART_TX_FIFO_LEVEL_3_4_EMPTY = 0x00000001, // Interrupt triggers when FIFO <= 3/4 empty
    DL_UART_TX_FIFO_LEVEL_1_2_EMPTY = 0x00000002, // Interrupt triggers when FIFO <= 1/2 empty
    DL_UART_TX_FIFO_LEVEL_1_4_EMPTY = 0x00000003, // Interrupt triggers when FIFO <= 1/4 empty
    DL_UART_TX_FIFO_LEVEL_EMPTY = 0x00000005,     // Interrupt triggers when FIFO is empty
    DL_UART_TX_FIFO_LEVEL_ONE_ENTRY = 0x00000007, // Interrupt triggers when FIFO >= 1 entry
}

pub enum DL_UART_RX_FIFO_LEVEL {
    DL_UART_RX_FIFO_LEVEL_ONE_ENTRY = 0x00000070, // Interrupt triggers when FIFO >= 1 entry available. Required for DMA trigger
    DL_UART_RX_FIFO_LEVEL_FULL = 0x00000050,      // Interrupt triggers when FIFO is full
    DL_UART_RX_FIFO_LEVEL_3_4_FULL = 0x00000030,  // Interrupt triggers when FIFO >= 3/4 full
    DL_UART_RX_FIFO_LEVEL_1_2_FULL = 0x00000020,  // Interrupt triggers when FIFO >= 1/2 full
    DL_UART_RX_FIFO_LEVEL_1_4_FULL = 0x00000010,  // Interrupt triggers when FIFO >= 1/4 full
}

use crate::uart::UartRegs;
use crate::utils::update_reg;
impl UartRegs {
    pub fn enable_fifos(&mut self) {
        let UART_CTL0_FEN_ENABLE: u32 = 0x00020000;
        self.ctl0 |= UART_CTL0_FEN_ENABLE;
    }

    pub fn set_rx_fifo_threshold(&mut self, threshold: DL_UART_RX_FIFO_LEVEL) {
        let UART_IFLS_RXIFLSEL_MASK = 0x00000070;
        update_reg(&mut self.ifls, threshold as u32, UART_IFLS_RXIFLSEL_MASK);
    }

    pub fn set_tx_fifo_threshold(&mut self, threshold: DL_UART_TX_FIFO_LEVEL) {
        let UART_IFLS_RXIFLSEL_MASK = 0x00000007;
        update_reg(&mut self.ifls, threshold as u32, UART_IFLS_RXIFLSEL_MASK);
    }
}
