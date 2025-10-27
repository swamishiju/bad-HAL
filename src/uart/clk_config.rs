pub enum DL_UART_CLOCK {
    DL_UART_CLOCK_BUSCLK = 0x00000008, // Selects BUSCLK as the clock source
    DL_UART_CLOCK_MFCLK = 0x00000004,  // Selects MFCLK as the clock source
    DL_UART_CLOCK_LFCLK = 0x00000002,  // Selects LFCLK as the clock source
}

pub enum DL_UART_CLOCK_DIVIDE_RATIO {
    DL_UART_CLOCK_DIVIDE_RATIO_1 = 0, // UART source clock divide ratio set to 1
    DL_UART_CLOCK_DIVIDE_RATIO_2 = 1, // UART source clock divide ratio set to 2
    DL_UART_CLOCK_DIVIDE_RATIO_3 = 2, // UART source clock divide ratio set to 3
    DL_UART_CLOCK_DIVIDE_RATIO_4 = 3, // UART source clock divide ratio set to 4
    DL_UART_CLOCK_DIVIDE_RATIO_5 = 4, // UART source clock divide ratio set to 5
    DL_UART_CLOCK_DIVIDE_RATIO_6 = 5, // UART source clock divide ratio set to 6
    DL_UART_CLOCK_DIVIDE_RATIO_7 = 6, // UART source clock divide ratio set to 7
    DL_UART_CLOCK_DIVIDE_RATIO_8 = 7, // UART source clock divide ratio set to 8
}

pub struct DL_UART_ClockConfig {
    pub clockSel: DL_UART_CLOCK, // Selects uart module clock source @ref DL_UART_CLOCK
    pub divideRatio: DL_UART_CLOCK_DIVIDE_RATIO, // Selects the divide ratio. One of @ref DL_UART_CLOCK_DIVIDE_RATIO
}

use crate::uart::UART_Regs;
impl UART_Regs {
    pub fn set_clk_config(&mut self, config: DL_UART_ClockConfig) {
        self.CLKSEL = config.clockSel as u32;
        self.CLKDIV = config.divideRatio as u32;
    }
}
