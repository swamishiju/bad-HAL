#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TxFifoLevel {
    /// Interrupt triggers when FIFO ≤ 3/4 empty
    ThreeQuartersEmpty = 0x00000001,
    /// Interrupt triggers when FIFO ≤ 1/2 empty
    HalfEmpty = 0x00000002,
    /// Interrupt triggers when FIFO ≤ 1/4 empty
    QuarterEmpty = 0x00000003,
    /// Interrupt triggers when FIFO is empty
    Empty = 0x00000005,
    /// Interrupt triggers when FIFO ≥ 1 entry
    OneEntry = 0x00000007,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum RxFifoLevel {
    /// Interrupt triggers when FIFO ≥ 1 entry available (required for DMA trigger)
    OneEntry = 0x00000070,
    /// Interrupt triggers when FIFO is full
    Full = 0x00000050,
    /// Interrupt triggers when FIFO ≥ 3/4 full
    ThreeQuartersFull = 0x00000030,
    /// Interrupt triggers when FIFO ≥ 1/2 full
    HalfFull = 0x00000020,
    /// Interrupt triggers when FIFO ≥ 1/4 full
    QuarterFull = 0x00000010,
}

use crate::uart::UartRegs;
use crate::utils::update_reg;

impl UartRegs {
    /// Enables both TX and RX FIFOs.
    pub fn enable_fifos(&mut self) {
        const UART_CTL0_FEN_ENABLE: u32 = 0x0002_0000;
        self.ctl0 |= UART_CTL0_FEN_ENABLE;
    }

    /// Sets the RX FIFO interrupt trigger threshold.
    pub fn set_rx_fifo_threshold(&mut self, threshold: RxFifoLevel) {
        const UART_IFLS_RXIFLSEL_MASK: u32 = 0x0000_0070;
        update_reg(&mut self.ifls, threshold as u32, UART_IFLS_RXIFLSEL_MASK);
    }

    /// Sets the TX FIFO interrupt trigger threshold.
    pub fn set_tx_fifo_threshold(&mut self, threshold: TxFifoLevel) {
        const UART_IFLS_TXIFLSEL_MASK: u32 = 0x0000_0007;
        update_reg(&mut self.ifls, threshold as u32, UART_IFLS_TXIFLSEL_MASK);
    }
}

// pub enum DL_UART_TX_FIFO_LEVEL {
// DL_UART_TX_FIFO_LEVEL_3_4_EMPTY = 0x00000001, // Interrupt triggers when FIFO <= 3/4 empty
// DL_UART_TX_FIFO_LEVEL_1_2_EMPTY = 0x00000002, // Interrupt triggers when FIFO <= 1/2 empty
// DL_UART_TX_FIFO_LEVEL_1_4_EMPTY = 0x00000003, // Interrupt triggers when FIFO <= 1/4 empty
// DL_UART_TX_FIFO_LEVEL_EMPTY = 0x00000005,     // Interrupt triggers when FIFO is empty
// DL_UART_TX_FIFO_LEVEL_ONE_ENTRY = 0x00000007, // Interrupt triggers when FIFO >= 1 entry
// }
//
// pub enum DL_UART_RX_FIFO_LEVEL {
// DL_UART_RX_FIFO_LEVEL_ONE_ENTRY = 0x00000070, // Interrupt triggers when FIFO >= 1 entry available. Required for DMA trigger
// DL_UART_RX_FIFO_LEVEL_FULL = 0x00000050,      // Interrupt triggers when FIFO is full
// DL_UART_RX_FIFO_LEVEL_3_4_FULL = 0x00000030,  // Interrupt triggers when FIFO >= 3/4 full
// DL_UART_RX_FIFO_LEVEL_1_2_FULL = 0x00000020,  // Interrupt triggers when FIFO >= 1/2 full
// DL_UART_RX_FIFO_LEVEL_1_4_FULL = 0x00000010,  // Interrupt triggers when FIFO >= 1/4 full
// }
//
// use crate::uart::UartRegs;
// use crate::utils::update_reg;
// impl UartRegs {
// pub fn enable_fifos(&mut self) {
// let UART_CTL0_FEN_ENABLE: u32 = 0x00020000;
// self.ctl0 |= UART_CTL0_FEN_ENABLE;
// }
//
// pub fn set_rx_fifo_threshold(&mut self, threshold: DL_UART_RX_FIFO_LEVEL) {
// let UART_IFLS_RXIFLSEL_MASK = 0x00000070;
// update_reg(&mut self.ifls, threshold as u32, UART_IFLS_RXIFLSEL_MASK);
// }
//
// pub fn set_tx_fifo_threshold(&mut self, threshold: DL_UART_TX_FIFO_LEVEL) {
// let UART_IFLS_RXIFLSEL_MASK = 0x00000007;
// update_reg(&mut self.ifls, threshold as u32, UART_IFLS_RXIFLSEL_MASK);
// }
// }
