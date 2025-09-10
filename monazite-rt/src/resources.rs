use crate::pins;
use hal::{
    dma::{dma, mdma},
    gpio::GpioExt as _,
    pac,
    prelude::*,
    rcc::rec,
};
use stm32h7xx_hal as hal;

pub struct Resources {
    pub shared: Shared,
    pub adc: Adc,
    pub gpio: Gpio,
    pub btmgr: Btmgr,
    pub iflash: Iflash,
    pub direct_uart: DirectUart,
    pub ramecc: Ramecc,
    pub thermometer: Thermometer,
    pub wdt: Wdt,
    pub dbgmcu: Dbgmcu,
}

pub struct Shared {
    pub clocks: hal::rcc::CoreClocks,
    pub delay: hal::delay::DelayFromCountDownTimer<hal::timer::Timer<pac::TIM2>>,
}

pub struct Adc {
    pub pins: pins::Adc,
    pub adc2: pac::ADC2,
    pub ccdrp_adc12: rec::Adc12,
    pub ccdrp_dac12: rec::Dac12,
    pub dma1s7: dma::Stream7<pac::DMA1>,
}

pub struct Gpio {
    pub input_pins: pins::GpioInput,
    pub output_pins: pins::GpioOutput,
}

pub struct Btmgr {
    pub rtc: pac::RTC,
}

pub struct Iflash {
    pub flash: pac::FLASH,
}

pub struct DirectUartChannel<const S: u8, UART, REC> {
    pub uart: UART,
    pub rec: REC,
    pub dma_tx: dma::StreamX<pac::DMA1, S>,
    pub dma_rx: dma::StreamX<pac::DMA2, S>,
}

pub struct DirectUart {
    pub pins: pins::DirectUart,
    #[allow(clippy::type_complexity)]
    pub channels: (
        DirectUartChannel<0, pac::USART1, rec::Usart1>,
        DirectUartChannel<1, pac::USART2, rec::Usart2>,
        DirectUartChannel<2, pac::USART3, rec::Usart3>,
        DirectUartChannel<3, pac::UART4, rec::Uart4>,
        DirectUartChannel<4, pac::UART5, rec::Uart5>,
        DirectUartChannel<5, pac::UART7, rec::Uart7>,
    ),
}

pub struct Ramecc {
    pub ramecc1: pac::RAMECC1,
    pub ramecc2: pac::RAMECC2,
    pub ramecc3: pac::RAMECC3,
    pub mdma_s0: mdma::Stream0<pac::MDMA>,
}

pub struct Thermometer {
    pub adc3: pac::ADC3,
    pub ccdrp_adc3: rec::Adc3,
    pub dma2s7: dma::Stream7<pac::DMA2>,
}

pub struct Wdt {
    pub iwdg: pac::IWDG,
}

pub struct Dbgmcu {
    pub dbgmcu: pac::DBGMCU,
}

impl Resources {
    #[allow(clippy::similar_names, clippy::too_many_lines)]
    pub fn new(dp: pac::Peripherals) -> Self {
        let ccdr = super::init_power_reset_and_clocks(&dp.SYSCFG, dp.PWR, dp.RCC);
        let ccdrp = ccdr.peripheral;
        let clocks = ccdr.clocks;

        let gpioa = dp.GPIOA.split(ccdrp.GPIOA);
        let gpiob = dp.GPIOB.split(ccdrp.GPIOB);
        let gpioc = dp.GPIOC.split(ccdrp.GPIOC);
        let gpiod = dp.GPIOD.split(ccdrp.GPIOD);
        let gpioe = dp.GPIOE.split(ccdrp.GPIOE);
        let gpiof = dp.GPIOF.split(ccdrp.GPIOF);
        let gpiog = dp.GPIOG.split(ccdrp.GPIOG);
        let pins = pins::Pins::new(gpioa, gpiob, gpioc, gpiod, gpioe, gpiof, gpiog);

        let dma1s = dma::StreamsTuple::new(dp.DMA1, ccdrp.DMA1);
        let dma2s = dma::StreamsTuple::new(dp.DMA2, ccdrp.DMA2);
        let mdma_s = mdma::StreamsTuple::new(dp.MDMA, ccdrp.MDMA);
        let timer = dp.TIM2.timer(1.Hz(), ccdrp.TIM2, &ccdr.clocks);
        let delay = hal::delay::DelayFromCountDownTimer::new(timer);

        Self {
            shared: Shared { clocks, delay },
            adc: Adc {
                pins: pins.adc,
                adc2: dp.ADC2,
                ccdrp_adc12: ccdrp.ADC12,
                ccdrp_dac12: ccdrp.DAC12,
                dma1s7: dma1s.7,
            },
            gpio: Gpio {
                input_pins: pins.gpio_input,
                output_pins: pins.gpio_output,
            },
            btmgr: Btmgr { rtc: dp.RTC },
            iflash: Iflash { flash: dp.FLASH },
            direct_uart: DirectUart {
                pins: pins.direct_uart,
                channels: (
                    DirectUartChannel {
                        uart: dp.USART1,
                        rec: ccdrp.USART1,
                        dma_tx: dma1s.0,
                        dma_rx: dma2s.0,
                    },
                    DirectUartChannel {
                        uart: dp.USART2,
                        rec: ccdrp.USART2,
                        dma_tx: dma1s.1,
                        dma_rx: dma2s.1,
                    },
                    DirectUartChannel {
                        uart: dp.USART3,
                        rec: ccdrp.USART3,
                        dma_tx: dma1s.2,
                        dma_rx: dma2s.2,
                    },
                    DirectUartChannel {
                        uart: dp.UART4,
                        rec: ccdrp.UART4,
                        dma_tx: dma1s.3,
                        dma_rx: dma2s.3,
                    },
                    DirectUartChannel {
                        uart: dp.UART5,
                        rec: ccdrp.UART5,
                        dma_tx: dma1s.4,
                        dma_rx: dma2s.4,
                    },
                    DirectUartChannel {
                        uart: dp.UART7,
                        rec: ccdrp.UART7,
                        dma_tx: dma1s.5,
                        dma_rx: dma2s.5,
                    },
                ),
            },
            ramecc: Ramecc {
                ramecc1: dp.RAMECC1,
                ramecc2: dp.RAMECC2,
                ramecc3: dp.RAMECC3,
                mdma_s0: mdma_s.0,
            },
            thermometer: Thermometer {
                adc3: dp.ADC3,
                ccdrp_adc3: ccdrp.ADC3,
                dma2s7: dma2s.7,
            },
            wdt: Wdt { iwdg: dp.IWDG },
            dbgmcu: Dbgmcu { dbgmcu: dp.DBGMCU },
        }
    }
}
