#[allow(clippy::wildcard_imports)]
use hal::gpio::{
    gpioa::{self, *},
    gpiob::{self, *},
    gpioc::{self, *},
    gpiod::{self, *},
    gpioe::{self, *},
    gpiof::{self, *},
    gpiog::{self, *},
};
use stm32h7xx_hal as hal;

pub struct Pins {
    pub adc: Adc,
    pub direct_uart: DirectUart,
    pub gpio_output: GpioOutput,
    pub gpio_input: GpioInput,
}

pub struct Adc {
    pub agc_inc: PA6,
    pub agc_coh: PB0,
    pub freq_err: PC0,
}

pub struct TxRx<TX, RX> {
    pub tx: TX,
    pub rx: RX,
}

pub struct DirectUart(
    pub TxRx<PB14, PB15>,
    pub TxRx<PA2, PA3>,
    pub TxRx<PD8, PD9>,
    pub TxRx<PB9, PB8>,
    pub TxRx<PB13, PB12>,
    pub TxRx<PF7, PF6>,
);

pub struct GpioOutput(
    /// LVTTL IN ch1
    pub PG7,
    /// LVTTL IN ch2
    pub PG14,
    /// LVTTL IN ch3
    pub PA4,
    /// LVTTL IN ch4
    pub PG8,
    /// LVTTL IN ch5
    pub PF9,
    /// LVTTL IN ch6
    pub PD6,
    /// LVTTL IN ch7
    pub PG9,
    /// LVTTL IN ch8
    pub PA5,
    /// LVTTL IN ch9
    pub PB1,
);

pub struct GpioInput(
    /// LVTTL IN ch1
    pub PA0,
    /// LVTTL IN ch2
    pub PC12,
    /// LVTTL IN ch3
    pub PF11,
    /// LVTTL IN ch4
    pub PE5,
    /// LVTTL IN ch5
    pub PE6,
    /// LVTTL IN ch6
    pub PA7,
);

impl Pins {
    #[allow(clippy::similar_names, clippy::too_many_lines)]
    pub fn new(
        gpioa: gpioa::Parts,
        gpiob: gpiob::Parts,
        gpioc: gpioc::Parts,
        gpiod: gpiod::Parts,
        gpioe: gpioe::Parts,
        gpiof: gpiof::Parts,
        gpiog: gpiog::Parts,
    ) -> Self {
        Pins {
            adc: Adc {
                agc_inc: gpioa.pa6,
                agc_coh: gpiob.pb0,
                freq_err: gpioc.pc0,
            },
            direct_uart: DirectUart(
                TxRx {
                    tx: gpiob.pb14,
                    rx: gpiob.pb15,
                },
                TxRx {
                    tx: gpioa.pa2,
                    rx: gpioa.pa3,
                },
                TxRx {
                    tx: gpiod.pd8,
                    rx: gpiod.pd9,
                },
                TxRx {
                    tx: gpiob.pb9,
                    rx: gpiob.pb8,
                },
                TxRx {
                    tx: gpiob.pb13,
                    rx: gpiob.pb12,
                },
                TxRx {
                    tx: gpiof.pf7,
                    rx: gpiof.pf6,
                },
            ),
            gpio_output: GpioOutput(
                gpiog.pg7, gpiog.pg14, gpioa.pa4, gpiog.pg8, gpiof.pf9, gpiod.pd6, gpiog.pg9,
                gpioa.pa5, gpiob.pb1,
            ),
            gpio_input: GpioInput(
                gpioa.pa0, gpioc.pc12, gpiof.pf11, gpioe.pe5, gpioe.pe6, gpioa.pa7,
            ),
        }
    }
}
