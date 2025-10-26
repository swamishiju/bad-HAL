pub enum DL_UART_PULSE_WIDTH {
    DL_UART_PULSE_WIDTH_5_NS = 0x00000000, // Pulses shorter then 5ns length are filtered
    DL_UART_PULSE_WIDTH_10_NS = 0x00000200, // Pulses shorter then 10ns length are filtered
    DL_UART_PULSE_WIDTH_25_NS = 0x00000400, // Pulses shorter then 25ns length are filtered
    DL_UART_PULSE_WIDTH_50_NS = 0x00000600, // Pulses shorter then 50ns length are filtered
}

pub enum DL_UART_PARITY {
    DL_UART_PARITY_EVEN = (0x00000002 | 0x00000004), // Enable even parity generation, checks for an even number of 1s
    DL_UART_PARITY_ODD = (0x00000002 | 0x00000000), // Enable odd parity generation, checks for an odd number of 1s
    DL_UART_PARITY_STICK_ONE = (0x00000002 | 0x00000040 | 0x00000000), // Enable stick parity with a parity bit of '1'
    DL_UART_PARITY_STICK_ZERO = (0x00000002 | 0x00000040 | 0x00000004), // Enable stick parity with a parity bit of '0'
    DL_UART_PARITY_NONE = 0x00000000, // Disable parity checking and generation
} /* When enabled, a permanent '1' is set as parity when transmitting and
   * checked as '1' when receiving data.
   *
   * When enabled, a permanent '0' is set as parity when transmitting and
   * checked as '0' when receiving data. */

pub enum DL_UART_WORD_LENGTH {
    DL_UART_WORD_LENGTH_5_BITS = 0x00000000, // Word length is 5 bits
    DL_UART_WORD_LENGTH_6_BITS = 0x00000010, // Word length is 6 bits
    DL_UART_WORD_LENGTH_7_BITS = 0x00000020, // Word length is 7 bits
    DL_UART_WORD_LENGTH_8_BITS = 0x00000030, // Word length is 8 bits
}

pub enum DL_UART_MODE {
    DL_UART_MODE_NORMAL = 0x00000000,     // Normal operation
    DL_UART_MODE_RS485 = 0x00000100,      // Operate in RS485 mode
    DL_UART_MODE_IDLE_LINE = 0x00000200,  // Operate in Idle Line mode
    DL_UART_MODE_ADDR_9_BIT = 0x00000300, // Operate in 9 Bit Address mode
    DL_UART_MODE_SMART_CARD = 0x00000400, // Operate in ISO7816 Smart Card Support mode
    DL_UART_MODE_DALI = 0x00000500,       // Operate in DALI mode
}

pub enum DL_UART_STOP_BITS {
    DL_UART_STOP_BITS_ONE = 0x00000000, // One stop bit is transmitted at the end of the frame
    DL_UART_STOP_BITS_TWO = 0x00000008, // Two stop bits are transmitted at the end of the frame
}

pub enum DL_UART_DIRECTION {
    DL_UART_DIRECTION_TX = 0x00000010, // Enable UART transmitter
    DL_UART_DIRECTION_RX = 0x00000008, // Enable UART receiver
    DL_UART_DIRECTION_TX_RX = (0x00000008 | 0x00000010), // Enable UART transmitter and receiver
    DL_UART_DIRECTION_NONE = 0x00000000, // Disable UART transmitter and receiver
}

pub enum DL_UART_FLOW_CONTROL {
    DL_UART_FLOW_CONTROL_RTS = 0x00002000, // Enable request to send
    DL_UART_FLOW_CONTROL_CTS = 0x00004000, // Enable clear to send
    DL_UART_FLOW_CONTROL_RTS_CTS = (0x00002000 | 0x00004000), // Enable request to send and clear to send
    DL_UART_FLOW_CONTROL_NONE = 0x00000000,                   // Disable flow control
}

pub struct DL_UART_Config {
    pub mode: DL_UART_MODE, // The communication mode and protocol used. One of @ref DL_UART_MODE
    pub direction: DL_UART_DIRECTION, // The communication direction. One of @ref DL_UART_DIRECTION.
    pub flowControl: DL_UART_FLOW_CONTROL, // The flow control configuration. One of @ref DL_UART_FLOW_CONTROL
    pub parity: DL_UART_PARITY,            // The parity configuration. One of @ref DL_UART_PARITY
    pub wordLength: DL_UART_WORD_LENGTH, // The size of the data transfer. One of @ref DL_UART_WORD_LENGTH
    pub stopBits: DL_UART_STOP_BITS,     // One of @ref DL_UART_STOP_BITS
}

use crate::uart::UART_Regs;
use crate::utils::update_reg;
impl UART_Regs {
    pub fn uart_init(&mut self, config: DL_UART_Config) {
        self.disable();

        let UART_CTL0_RXE_MASK: u32 = 0x00000008;
        let UART_CTL0_TXE_MASK: u32 = 0x00000010;
        let UART_CTL0_MODE_MASK: u32 = 0x0000700;
        let UART_CTL0_RTSEN_MASK: u32 = 0x00002000;
        let UART_CTL0_CTSEN_MASK: u32 = 0x00004000;
        let UART_CTL0_FEN_MASK: u32 = 0x00020000;

        update_reg(
            &mut self.CTL0,
            config.mode as u32 | config.direction as u32 | config.flowControl as u32,
            UART_CTL0_RXE_MASK
                | UART_CTL0_TXE_MASK
                | UART_CTL0_MODE_MASK
                | UART_CTL0_RTSEN_MASK
                | UART_CTL0_CTSEN_MASK
                | UART_CTL0_FEN_MASK,
        );

        let UART_LCRH_PEN_ENABLE: u32 = 0x00000002;
        let UART_LCRH_EPS_MASK: u32 = 0x00000004;
        let UART_LCRH_SPS_MASK: u32 = 0x00000040;
        let UART_LCRH_WLEN_MASK: u32 = 0x00000030;
        let UART_LCRH_STP2_MASK: u32 = 0x00000008;

        update_reg(
            &mut self.LCRH,
            config.parity as u32 | config.wordLength as u32 | config.stopBits as u32,
            UART_LCRH_PEN_ENABLE
                | UART_LCRH_EPS_MASK
                | UART_LCRH_SPS_MASK
                | UART_LCRH_WLEN_MASK
                | UART_LCRH_STP2_MASK,
        );
    }
}
