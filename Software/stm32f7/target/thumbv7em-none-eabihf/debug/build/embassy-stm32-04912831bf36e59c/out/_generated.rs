embassy_hal_internal::peripherals_definition!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD7,
    PD8,
    PD9,
    PD10,
    PD11,
    PD12,
    PD13,
    PD14,
    PD15,
    PE0,
    PE1,
    PE2,
    PE3,
    PE4,
    PE5,
    PE6,
    PE7,
    PE8,
    PE9,
    PE10,
    PE11,
    PE12,
    PE13,
    PE14,
    PE15,
    PF0,
    PF1,
    PF2,
    PF3,
    PF4,
    PF5,
    PF6,
    PF7,
    PF8,
    PF9,
    PF10,
    PF11,
    PF12,
    PF13,
    PF14,
    PF15,
    PG0,
    PG1,
    PG2,
    PG3,
    PG4,
    PG5,
    PG6,
    PG7,
    PG8,
    PG9,
    PG10,
    PG11,
    PG12,
    PG13,
    PG14,
    PG15,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    CAN2,
    CAN3,
    CEC,
    CRC,
    CRYP,
    DAC1,
    DBGMCU,
    DCMI,
    DMA1,
    DMA2,
    DMA2D,
    ETH,
    FLASH,
    FMC,
    HASH,
    I2C1,
    I2C2,
    I2C3,
    I2C4,
    IWDG,
    JPEG,
    LPTIM1,
    LTDC,
    MDIOS,
    PWR,
    QUADSPI,
    MCO1,
    MCO2,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SDMMC1,
    SDMMC2,
    SPDIFRX1,
    SPI1,
    SPI2,
    SPI3,
    SPI4,
    SPI5,
    SPI6,
    SYSCFG,
    TIM1,
    TIM10,
    TIM11,
    TIM12,
    TIM13,
    TIM14,
    TIM2,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TIM9,
    UART4,
    UART5,
    UART7,
    UART8,
    UID,
    USART1,
    USART2,
    USART3,
    USART6,
    USB_OTG_FS,
    USB_OTG_HS,
    VREFINTCAL,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH0,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::peripherals_struct!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD7,
    PD8,
    PD9,
    PD10,
    PD11,
    PD12,
    PD13,
    PD14,
    PD15,
    PE0,
    PE1,
    PE2,
    PE3,
    PE4,
    PE5,
    PE6,
    PE7,
    PE8,
    PE9,
    PE10,
    PE11,
    PE12,
    PE13,
    PE14,
    PE15,
    PF0,
    PF1,
    PF2,
    PF3,
    PF4,
    PF5,
    PF6,
    PF7,
    PF8,
    PF9,
    PF10,
    PF11,
    PF12,
    PF13,
    PF14,
    PF15,
    PG0,
    PG1,
    PG2,
    PG3,
    PG4,
    PG5,
    PG6,
    PG7,
    PG8,
    PG9,
    PG10,
    PG11,
    PG12,
    PG13,
    PG14,
    PG15,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    CAN2,
    CAN3,
    CEC,
    CRC,
    CRYP,
    DAC1,
    DBGMCU,
    DCMI,
    DMA1,
    DMA2,
    DMA2D,
    ETH,
    FLASH,
    FMC,
    HASH,
    I2C1,
    I2C2,
    I2C3,
    I2C4,
    IWDG,
    JPEG,
    LPTIM1,
    LTDC,
    MDIOS,
    PWR,
    QUADSPI,
    MCO1,
    MCO2,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SDMMC1,
    SDMMC2,
    SPDIFRX1,
    SPI1,
    SPI2,
    SPI3,
    SPI4,
    SPI5,
    SPI6,
    SYSCFG,
    TIM1,
    TIM10,
    TIM11,
    TIM13,
    TIM14,
    TIM2,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TIM9,
    UART4,
    UART5,
    UART7,
    UART8,
    UID,
    USART1,
    USART2,
    USART3,
    USART6,
    USB_OTG_FS,
    USB_OTG_HS,
    VREFINTCAL,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH0,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD,
    TAMP_STAMP,
    RTC_WKUP,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_STREAM0,
    DMA1_STREAM1,
    DMA1_STREAM2,
    DMA1_STREAM3,
    DMA1_STREAM4,
    DMA1_STREAM5,
    DMA1_STREAM6,
    ADC,
    CAN1_TX,
    CAN1_RX0,
    CAN1_RX1,
    CAN1_SCE,
    EXTI9_5,
    TIM1_BRK_TIM9,
    TIM1_UP_TIM10,
    TIM1_TRG_COM_TIM11,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    OTG_FS_WKUP,
    TIM8_BRK_TIM12,
    TIM8_UP_TIM13,
    TIM8_TRG_COM_TIM14,
    TIM8_CC,
    DMA1_STREAM7,
    FMC,
    SDMMC1,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6_DAC,
    TIM7,
    DMA2_STREAM0,
    DMA2_STREAM1,
    DMA2_STREAM2,
    DMA2_STREAM3,
    DMA2_STREAM4,
    ETH,
    ETH_WKUP,
    CAN2_TX,
    CAN2_RX0,
    CAN2_RX1,
    CAN2_SCE,
    OTG_FS,
    DMA2_STREAM5,
    DMA2_STREAM6,
    DMA2_STREAM7,
    USART6,
    I2C3_EV,
    I2C3_ER,
    OTG_HS_EP1_OUT,
    OTG_HS_EP1_IN,
    OTG_HS_WKUP,
    OTG_HS,
    DCMI,
    CRYP,
    HASH_RNG,
    FPU,
    UART7,
    UART8,
    SPI4,
    SPI5,
    SPI6,
    SAI1,
    LTDC,
    LTDC_ER,
    DMA2D,
    SAI2,
    QUADSPI,
    LPTIM1,
    CEC,
    I2C4_EV,
    I2C4_ER,
    SPDIF_RX,
    SDMMC2,
    CAN3_TX,
    CAN3_RX0,
    CAN3_RX1,
    CAN3_SCE,
    JPEG,
    MDIOS,
);
pub const MAX_ERASE_SIZE: usize = 262144u32 as usize;
pub mod flash_regions {
    pub const BANK1_REGION1: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        base: 134217728u32,
        size: 131072u32,
        erase_size: 32768u32,
        write_size: 16u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region1<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK1_REGION2: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        base: 134348800u32,
        size: 131072u32,
        erase_size: 131072u32,
        write_size: 16u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region2<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK1_REGION3: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        base: 134479872u32,
        size: 1835008u32,
        erase_size: 262144u32,
        write_size: 16u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region3<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region1: Bank1Region1<'d, MODE>,
        pub bank1_region2: Bank1Region2<'d, MODE>,
        pub bank1_region3: Bank1Region3<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(p: embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>) -> Self {
            Self {
                bank1_region1: Bank1Region1(
                    &BANK1_REGION1,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank1_region2: Bank1Region2(
                    &BANK1_REGION2,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank1_region3: Bank1Region3(
                    &BANK1_REGION3,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 3usize] =
        [&BANK1_REGION1, &BANK1_REGION2, &BANK1_REGION3];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (17u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (17u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (17u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN1" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 25u8)),
            (16u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN2" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 26u8)),
            (16u8, 26u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN3" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 13u8)),
            (16u8, 13u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CEC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().cecsel() {
            crate::pac::rcc::vals::Cecsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "LSE")
            },
            crate::pac::rcc::vals::Cecsel::HSI_DIV488 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi_div488 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "HSI_DIV488")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "CEC"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 27u8)),
            (16u8, 27u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CEC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 12u8)),
            (12u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRYP {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRYP" , "HCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((5u8, 4u8)),
            (13u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRYP {}
impl crate::rcc::SealedRccPeripheral for peripherals::DAC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 29u8)),
            (16u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DAC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DCMI {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DCMI" , "HCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((5u8, 0u8)),
            (13u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DCMI {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 21u8)),
            (12u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 22u8)),
            (12u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2D {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2D" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 23u8)),
            (12u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2D {}
impl crate::rcc::SealedRccPeripheral for peripherals::ETH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ETH" , "HCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 25u8)),
            (12u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ETH {}
impl crate::rcc::SealedRccPeripheral for peripherals::FMC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "HCLK3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((6u8, 0u8)),
            (14u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FMC {}
impl crate::rcc::SealedRccPeripheral for peripherals::HASH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "HASH" , "HCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (13u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::HASH {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().i2c1sel() {
            crate::pac::rcc::vals::I2csel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "PCLK1")
            },
            crate::pac::rcc::vals::I2csel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "SYS")
            },
            crate::pac::rcc::vals::I2csel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "HSI")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C1"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 21u8)),
            (16u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().i2c2sel() {
            crate::pac::rcc::vals::I2csel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "PCLK1")
            },
            crate::pac::rcc::vals::I2csel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "SYS")
            },
            crate::pac::rcc::vals::I2csel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "HSI")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C2"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 22u8)),
            (16u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().i2c3sel() {
            crate::pac::rcc::vals::I2csel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "PCLK1")
            },
            crate::pac::rcc::vals::I2csel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "SYS")
            },
            crate::pac::rcc::vals::I2csel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "HSI")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C3"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 23u8)),
            (16u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C4 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().i2c4sel() {
            crate::pac::rcc::vals::I2csel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "PCLK1")
            },
            crate::pac::rcc::vals::I2csel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "SYS")
            },
            crate::pac::rcc::vals::I2csel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "HSI")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C4"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 24u8)),
            (16u8, 24u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::JPEG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "JPEG" , "HCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (13u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::JPEG {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().lptim1sel() {
            crate::pac::rcc::vals::Lptimsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "PCLK1")
            },
            crate::pac::rcc::vals::Lptimsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "LSI")
            },
            crate::pac::rcc::vals::Lptimsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "HSI")
            },
            crate::pac::rcc::vals::Lptimsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM1"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 9u8)),
            (16u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LTDC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LTDC" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 26u8)),
            (17u8, 26u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LTDC {}
impl crate::rcc::SealedRccPeripheral for peripherals::MDIOS {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "MDIOS" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 30u8)),
            (17u8, 30u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::MDIOS {}
impl crate::rcc::SealedRccPeripheral for peripherals::PWR {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 28u8)),
            (16u8, 28u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::PWR {}
impl crate::rcc::SealedRccPeripheral for peripherals::QUADSPI {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "HCLK3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((6u8, 1u8)),
            (14u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::QUADSPI {}
impl crate::rcc::SealedRccPeripheral for peripherals::RNG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "HCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((5u8, 6u8)),
            (13u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RNG {}
impl crate::rcc::SealedRccPeripheral for peripherals::RTC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (16u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Standby,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RTC {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr1().read().sai1sel() {
            crate::pac::rcc::vals::Saisel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "PLLSAI1_Q")
            },
            crate::pac::rcc::vals::Saisel::PLLI2S1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . plli2s1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "PLLI2S1_Q")
            },
            crate::pac::rcc::vals::Saisel::AFIF => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . afif . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "AFIF")
            },
            crate::pac::rcc::vals::Saisel::HSI_HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi_hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "HSI_HSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI1"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 22u8)),
            (17u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr1().read().sai2sel() {
            crate::pac::rcc::vals::Saisel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "PLLSAI1_Q")
            },
            crate::pac::rcc::vals::Saisel::PLLI2S1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . plli2s1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "PLLI2S1_Q")
            },
            crate::pac::rcc::vals::Saisel::AFIF => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . afif . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "AFIF")
            },
            crate::pac::rcc::vals::Saisel::HSI_HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi_hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "HSI_HSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI2"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 23u8)),
            (17u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SDMMC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().sdmmc1sel() {
            crate::pac::rcc::vals::Sdmmcsel::CLK48 => {
                match crate::pac::RCC.dckcfgr2().read().clk48sel() {
                    crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "PLL1_Q")
                    },
                    crate::pac::rcc::vals::Clk48sel::PLLSAI1_P => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "PLLSAI1_P")
                    },
                    #[allow(unreachable_patterns)]
                    _ => panic!(
                        "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                        "SDMMC1"
                    ),
                }
            }
            crate::pac::rcc::vals::Sdmmcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "SYS")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SDMMC1"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 11u8)),
            (17u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SDMMC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SDMMC2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().sdmmc2sel() {
            crate::pac::rcc::vals::Sdmmcsel::CLK48 => {
                match crate::pac::RCC.dckcfgr2().read().clk48sel() {
                    crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC2" , "PLL1_Q")
                    },
                    crate::pac::rcc::vals::Clk48sel::PLLSAI1_P => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC2" , "PLLSAI1_P")
                    },
                    #[allow(unreachable_patterns)]
                    _ => panic!(
                        "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                        "SDMMC2"
                    ),
                }
            }
            crate::pac::rcc::vals::Sdmmcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC2" , "SYS")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SDMMC2"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 7u8)),
            (17u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SDMMC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPDIFRX1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 16u8)),
            (16u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPDIFRX1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 12u8)),
            (17u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 14u8)),
            (16u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 15u8)),
            (16u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 13u8)),
            (17u8, 13u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI5" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 20u8)),
            (17u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 21u8)),
            (17u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SYSCFG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "PCLK2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 14u8)),
            (17u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SYSCFG {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 0u8)),
            (17u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM10 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM10" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 17u8)),
            (17u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM10 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM11 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM11" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 18u8)),
            (17u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM11 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM12 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM12" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 6u8)),
            (16u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM12 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM13 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM13" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 7u8)),
            (16u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM13 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM14 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 8u8)),
            (16u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM14 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 0u8)),
            (16u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 1u8)),
            (16u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 2u8)),
            (16u8, 2u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 3u8)),
            (16u8, 3u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 4u8)),
            (16u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM7 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "PCLK1_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 5u8)),
            (16u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM8 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 1u8)),
            (17u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM8 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM9 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM9" , "PCLK2_TIM")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 16u8)),
            (17u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM9 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART4 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().uart4sel() {
            crate::pac::rcc::vals::Usart2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "PCLK1")
            },
            crate::pac::rcc::vals::Usart2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "SYS")
            },
            crate::pac::rcc::vals::Usart2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "HSI")
            },
            crate::pac::rcc::vals::Usart2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART4"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 19u8)),
            (16u8, 19u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART5 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().uart5sel() {
            crate::pac::rcc::vals::Usart2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "PCLK1")
            },
            crate::pac::rcc::vals::Usart2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "SYS")
            },
            crate::pac::rcc::vals::Usart2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "HSI")
            },
            crate::pac::rcc::vals::Usart2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART5"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 20u8)),
            (16u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART7 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().uart7sel() {
            crate::pac::rcc::vals::Usart2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "PCLK1")
            },
            crate::pac::rcc::vals::Usart2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "SYS")
            },
            crate::pac::rcc::vals::Usart2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "HSI")
            },
            crate::pac::rcc::vals::Usart2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART7"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 30u8)),
            (16u8, 30u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART8 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().uart8sel() {
            crate::pac::rcc::vals::Usart2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "PCLK1")
            },
            crate::pac::rcc::vals::Usart2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "SYS")
            },
            crate::pac::rcc::vals::Usart2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "HSI")
            },
            crate::pac::rcc::vals::Usart2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART8"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 31u8)),
            (16u8, 31u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART8 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().usart1sel() {
            crate::pac::rcc::vals::Usart1sel::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "PCLK2")
            },
            crate::pac::rcc::vals::Usart1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "SYS")
            },
            crate::pac::rcc::vals::Usart1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "HSI")
            },
            crate::pac::rcc::vals::Usart1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART1"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 4u8)),
            (17u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().usart2sel() {
            crate::pac::rcc::vals::Usart2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "PCLK1")
            },
            crate::pac::rcc::vals::Usart2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "SYS")
            },
            crate::pac::rcc::vals::Usart2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "HSI")
            },
            crate::pac::rcc::vals::Usart2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART2"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 17u8)),
            (16u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().usart3sel() {
            crate::pac::rcc::vals::Usart2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "PCLK1")
            },
            crate::pac::rcc::vals::Usart2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "SYS")
            },
            crate::pac::rcc::vals::Usart2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "HSI")
            },
            crate::pac::rcc::vals::Usart2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART3"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 18u8)),
            (16u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART6 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().usart6sel() {
            crate::pac::rcc::vals::Usart1sel::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "PCLK2")
            },
            crate::pac::rcc::vals::Usart1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "SYS")
            },
            crate::pac::rcc::vals::Usart1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "HSI")
            },
            crate::pac::rcc::vals::Usart1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "LSE")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART6"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 5u8)),
            (17u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_FS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "PLL1_Q")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "PLLSAI1_P")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_FS"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((5u8, 7u8)),
            (13u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_FS {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_HS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "PLL1_Q")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "PLLSAI1_P")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_HS"
            ),
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 29u8)),
            (12u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_HS {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "PCLK1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 11u8)),
            (16u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG {}
pub(crate) static mut REFCOUNTS: [u8; 0usize] = [];
pub mod mux {
    pub use crate::pac::rcc::vals::Cecsel;
    pub use crate::pac::rcc::vals::Clk48sel;
    pub use crate::pac::rcc::vals::I2csel;
    pub use crate::pac::rcc::vals::Lptimsel;
    pub use crate::pac::rcc::vals::Saisel;
    pub use crate::pac::rcc::vals::Sdmmcsel;
    pub use crate::pac::rcc::vals::Usart1sel;
    pub use crate::pac::rcc::vals::Usart2sel;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {
        pub sai1sel: Saisel,
        pub sai2sel: Saisel,
        pub cecsel: Cecsel,
        pub clk48sel: Clk48sel,
        pub i2c1sel: I2csel,
        pub i2c2sel: I2csel,
        pub i2c3sel: I2csel,
        pub i2c4sel: I2csel,
        pub lptim1sel: Lptimsel,
        pub sdmmc1sel: Sdmmcsel,
        pub sdmmc2sel: Sdmmcsel,
        pub uart4sel: Usart2sel,
        pub uart5sel: Usart2sel,
        pub uart7sel: Usart2sel,
        pub uart8sel: Usart2sel,
        pub usart1sel: Usart1sel,
        pub usart2sel: Usart2sel,
        pub usart3sel: Usart2sel,
        pub usart6sel: Usart1sel,
    }
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {
            crate::pac::RCC.dckcfgr1().modify(|w| {
                w.set_sai1sel(self.sai1sel);
                w.set_sai2sel(self.sai2sel);
            });
            crate::pac::RCC.dckcfgr2().modify(|w| {
                w.set_cecsel(self.cecsel);
                w.set_clk48sel(self.clk48sel);
                w.set_i2c1sel(self.i2c1sel);
                w.set_i2c2sel(self.i2c2sel);
                w.set_i2c3sel(self.i2c3sel);
                w.set_i2c4sel(self.i2c4sel);
                w.set_lptim1sel(self.lptim1sel);
                w.set_sdmmc1sel(self.sdmmc1sel);
                w.set_sdmmc2sel(self.sdmmc2sel);
                w.set_uart4sel(self.uart4sel);
                w.set_uart5sel(self.uart5sel);
                w.set_uart7sel(self.uart7sel);
                w.set_uart8sel(self.uart8sel);
                w.set_usart1sel(self.usart1sel);
                w.set_usart2sel(self.usart2sel);
                w.set_usart3sel(self.usart3sel);
                w.set_usart6sel(self.usart6sel);
            });
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub afif: crate::time::MaybeHertz,
    pub hclk1: crate::time::MaybeHertz,
    pub hclk2: crate::time::MaybeHertz,
    pub hclk3: crate::time::MaybeHertz,
    pub hsi: crate::time::MaybeHertz,
    pub hsi_div488: crate::time::MaybeHertz,
    pub hsi_hse: crate::time::MaybeHertz,
    pub lse: crate::time::MaybeHertz,
    pub lsi: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pclk2: crate::time::MaybeHertz,
    pub pclk2_tim: crate::time::MaybeHertz,
    pub pll1_q: crate::time::MaybeHertz,
    pub plli2s1_q: crate::time::MaybeHertz,
    pub pllsai1_p: crate::time::MaybeHertz,
    pub pllsai1_q: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
}
pub unsafe fn init_dma() {
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma1en(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma2en(true));
}
pub unsafe fn init_bdma() {}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioeen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiofen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiogen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiohen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioien(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiojen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioken(true));
}
impl_adc_pin!(ADC1, PA0, 0u8);
impl_adc_pin!(ADC1, PA1, 1u8);
impl_adc_pin!(ADC1, PA2, 2u8);
impl_adc_pin!(ADC1, PA3, 3u8);
impl_adc_pin!(ADC1, PA4, 4u8);
impl_adc_pin!(ADC1, PA5, 5u8);
impl_adc_pin!(ADC1, PA6, 6u8);
impl_adc_pin!(ADC1, PA7, 7u8);
impl_adc_pin!(ADC1, PB0, 8u8);
impl_adc_pin!(ADC1, PB1, 9u8);
impl_adc_pin!(ADC1, PC0, 10u8);
impl_adc_pin!(ADC1, PC1, 11u8);
impl_adc_pin!(ADC1, PC2, 12u8);
impl_adc_pin!(ADC1, PC3, 13u8);
impl_adc_pin!(ADC1, PC4, 14u8);
impl_adc_pin!(ADC1, PC5, 15u8);
impl_adc_pin!(ADC2, PA0, 0u8);
impl_adc_pin!(ADC2, PA1, 1u8);
impl_adc_pin!(ADC2, PA2, 2u8);
impl_adc_pin!(ADC2, PA3, 3u8);
impl_adc_pin!(ADC2, PA4, 4u8);
impl_adc_pin!(ADC2, PA5, 5u8);
impl_adc_pin!(ADC2, PA6, 6u8);
impl_adc_pin!(ADC2, PA7, 7u8);
impl_adc_pin!(ADC2, PB0, 8u8);
impl_adc_pin!(ADC2, PB1, 9u8);
impl_adc_pin!(ADC2, PC0, 10u8);
impl_adc_pin!(ADC2, PC1, 11u8);
impl_adc_pin!(ADC2, PC2, 12u8);
impl_adc_pin!(ADC2, PC3, 13u8);
impl_adc_pin!(ADC2, PC4, 14u8);
impl_adc_pin!(ADC2, PC5, 15u8);
impl_adc_pin!(ADC3, PA0, 0u8);
impl_adc_pin!(ADC3, PA1, 1u8);
impl_adc_pin!(ADC3, PA2, 2u8);
impl_adc_pin!(ADC3, PA3, 3u8);
impl_adc_pin!(ADC3, PC0, 10u8);
impl_adc_pin!(ADC3, PC1, 11u8);
impl_adc_pin!(ADC3, PC2, 12u8);
impl_adc_pin!(ADC3, PC3, 13u8);
impl_adc_pin!(ADC3, PF10, 8u8);
impl_adc_pin!(ADC3, PF3, 9u8);
impl_adc_pin!(ADC3, PF4, 14u8);
impl_adc_pin!(ADC3, PF5, 15u8);
impl_adc_pin!(ADC3, PF6, 4u8);
impl_adc_pin!(ADC3, PF7, 5u8);
impl_adc_pin!(ADC3, PF8, 6u8);
impl_adc_pin!(ADC3, PF9, 7u8);
pin_trait_impl!(crate::can::RxPin, CAN1, PA11, 9u8);
pin_trait_impl!(crate::can::TxPin, CAN1, PA12, 9u8);
pin_trait_impl!(crate::can::RxPin, CAN1, PB8, 9u8);
pin_trait_impl!(crate::can::TxPin, CAN1, PB9, 9u8);
pin_trait_impl!(crate::can::RxPin, CAN1, PD0, 9u8);
pin_trait_impl!(crate::can::TxPin, CAN1, PD1, 9u8);
pin_trait_impl!(crate::can::RxPin, CAN2, PB12, 9u8);
pin_trait_impl!(crate::can::TxPin, CAN2, PB13, 9u8);
pin_trait_impl!(crate::can::RxPin, CAN2, PB5, 9u8);
pin_trait_impl!(crate::can::TxPin, CAN2, PB6, 9u8);
pin_trait_impl!(crate::can::TxPin, CAN3, PA15, 11u8);
pin_trait_impl!(crate::can::RxPin, CAN3, PA8, 11u8);
pin_trait_impl!(crate::can::RxPin, CAN3, PB3, 11u8);
pin_trait_impl!(crate::can::TxPin, CAN3, PB4, 11u8);
pin_trait_impl!(crate::dac::DacPin<Ch1>, DAC1, PA4, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch2>, DAC1, PA5, 0u8);
pin_trait_impl!(crate::dcmi::D1Pin, DCMI, PA10, 13u8);
pin_trait_impl!(crate::dcmi::HSyncPin, DCMI, PA4, 13u8);
pin_trait_impl!(crate::dcmi::PixClkPin, DCMI, PA6, 13u8);
pin_trait_impl!(crate::dcmi::D0Pin, DCMI, PA9, 13u8);
pin_trait_impl!(crate::dcmi::D10Pin, DCMI, PB5, 13u8);
pin_trait_impl!(crate::dcmi::D5Pin, DCMI, PB6, 13u8);
pin_trait_impl!(crate::dcmi::VSyncPin, DCMI, PB7, 13u8);
pin_trait_impl!(crate::dcmi::D6Pin, DCMI, PB8, 13u8);
pin_trait_impl!(crate::dcmi::D7Pin, DCMI, PB9, 13u8);
pin_trait_impl!(crate::dcmi::D8Pin, DCMI, PC10, 13u8);
pin_trait_impl!(crate::dcmi::D4Pin, DCMI, PC11, 13u8);
pin_trait_impl!(crate::dcmi::D9Pin, DCMI, PC12, 13u8);
pin_trait_impl!(crate::dcmi::D0Pin, DCMI, PC6, 13u8);
pin_trait_impl!(crate::dcmi::D1Pin, DCMI, PC7, 13u8);
pin_trait_impl!(crate::dcmi::D2Pin, DCMI, PC8, 13u8);
pin_trait_impl!(crate::dcmi::D3Pin, DCMI, PC9, 13u8);
pin_trait_impl!(crate::dcmi::D11Pin, DCMI, PD2, 13u8);
pin_trait_impl!(crate::dcmi::D5Pin, DCMI, PD3, 13u8);
pin_trait_impl!(crate::dcmi::D10Pin, DCMI, PD6, 13u8);
pin_trait_impl!(crate::dcmi::D2Pin, DCMI, PE0, 13u8);
pin_trait_impl!(crate::dcmi::D3Pin, DCMI, PE1, 13u8);
pin_trait_impl!(crate::dcmi::D4Pin, DCMI, PE4, 13u8);
pin_trait_impl!(crate::dcmi::D6Pin, DCMI, PE5, 13u8);
pin_trait_impl!(crate::dcmi::D7Pin, DCMI, PE6, 13u8);
pin_trait_impl!(crate::dcmi::D11Pin, DCMI, PF10, 13u8);
pin_trait_impl!(crate::dcmi::D12Pin, DCMI, PF11, 13u8);
pin_trait_impl!(crate::dcmi::D2Pin, DCMI, PG10, 13u8);
pin_trait_impl!(crate::dcmi::D3Pin, DCMI, PG11, 13u8);
pin_trait_impl!(crate::dcmi::D13Pin, DCMI, PG15, 13u8);
pin_trait_impl!(crate::dcmi::D12Pin, DCMI, PG6, 13u8);
pin_trait_impl!(crate::dcmi::D13Pin, DCMI, PG7, 13u8);
pin_trait_impl!(crate::dcmi::VSyncPin, DCMI, PG9, 13u8);
pin_trait_impl!(crate::eth::RefClkPin, ETH, PA1, 11u8);
pin_trait_impl!(crate::eth::RXClkPin, ETH, PA1, 11u8);
pin_trait_impl!(crate::eth::MDIOPin, ETH, PA2, 11u8);
pin_trait_impl!(crate::eth::CRSPin, ETH, PA7, 11u8);
pin_trait_impl!(crate::eth::RXDVPin, ETH, PA7, 11u8);
pin_trait_impl!(crate::eth::RXD2Pin, ETH, PB0, 11u8);
pin_trait_impl!(crate::eth::RXD3Pin, ETH, PB1, 11u8);
pin_trait_impl!(crate::eth::TXEnPin, ETH, PB11, 11u8);
pin_trait_impl!(crate::eth::TXD0Pin, ETH, PB12, 11u8);
pin_trait_impl!(crate::eth::TXD1Pin, ETH, PB13, 11u8);
pin_trait_impl!(crate::eth::TXD3Pin, ETH, PB8, 11u8);
pin_trait_impl!(crate::eth::MDCPin, ETH, PC1, 11u8);
pin_trait_impl!(crate::eth::TXD2Pin, ETH, PC2, 11u8);
pin_trait_impl!(crate::eth::TXClkPin, ETH, PC3, 11u8);
pin_trait_impl!(crate::eth::RXD0Pin, ETH, PC4, 11u8);
pin_trait_impl!(crate::eth::RXD1Pin, ETH, PC5, 11u8);
pin_trait_impl!(crate::eth::TXD3Pin, ETH, PE2, 11u8);
pin_trait_impl!(crate::eth::TXEnPin, ETH, PG11, 11u8);
pin_trait_impl!(crate::eth::TXD0Pin, ETH, PG13, 11u8);
pin_trait_impl!(crate::eth::TXD1Pin, ETH, PG14, 11u8);
pin_trait_impl!(crate::fmc::SDNWEPin, FMC, PA7, 12u8);
pin_trait_impl!(crate::fmc::SDCKE1Pin, FMC, PB5, 12u8);
pin_trait_impl!(crate::fmc::SDNE1Pin, FMC, PB6, 12u8);
pin_trait_impl!(crate::fmc::NLPin, FMC, PB7, 12u8);
pin_trait_impl!(crate::fmc::SDNWEPin, FMC, PC0, 12u8);
pin_trait_impl!(crate::fmc::SDNE0Pin, FMC, PC2, 12u8);
pin_trait_impl!(crate::fmc::SDCKE0Pin, FMC, PC3, 12u8);
pin_trait_impl!(crate::fmc::SDNE0Pin, FMC, PC4, 12u8);
pin_trait_impl!(crate::fmc::SDCKE0Pin, FMC, PC5, 12u8);
pin_trait_impl!(crate::fmc::NWaitPin, FMC, PC6, 9u8);
pin_trait_impl!(crate::fmc::NE1Pin, FMC, PC7, 9u8);
pin_trait_impl!(crate::fmc::NCEPin, FMC, PC8, 9u8);
pin_trait_impl!(crate::fmc::NE2Pin, FMC, PC8, 9u8);
pin_trait_impl!(crate::fmc::D2Pin, FMC, PD0, 12u8);
pin_trait_impl!(crate::fmc::DA2Pin, FMC, PD0, 12u8);
pin_trait_impl!(crate::fmc::D3Pin, FMC, PD1, 12u8);
pin_trait_impl!(crate::fmc::DA3Pin, FMC, PD1, 12u8);
pin_trait_impl!(crate::fmc::D15Pin, FMC, PD10, 12u8);
pin_trait_impl!(crate::fmc::DA15Pin, FMC, PD10, 12u8);
pin_trait_impl!(crate::fmc::A16Pin, FMC, PD11, 12u8);
pin_trait_impl!(crate::fmc::A17Pin, FMC, PD12, 12u8);
pin_trait_impl!(crate::fmc::A18Pin, FMC, PD13, 12u8);
pin_trait_impl!(crate::fmc::D0Pin, FMC, PD14, 12u8);
pin_trait_impl!(crate::fmc::DA0Pin, FMC, PD14, 12u8);
pin_trait_impl!(crate::fmc::D1Pin, FMC, PD15, 12u8);
pin_trait_impl!(crate::fmc::DA1Pin, FMC, PD15, 12u8);
pin_trait_impl!(crate::fmc::ClkPin, FMC, PD3, 12u8);
pin_trait_impl!(crate::fmc::NOEPin, FMC, PD4, 12u8);
pin_trait_impl!(crate::fmc::NWEPin, FMC, PD5, 12u8);
pin_trait_impl!(crate::fmc::NWaitPin, FMC, PD6, 12u8);
pin_trait_impl!(crate::fmc::NE1Pin, FMC, PD7, 12u8);
pin_trait_impl!(crate::fmc::D13Pin, FMC, PD8, 12u8);
pin_trait_impl!(crate::fmc::DA13Pin, FMC, PD8, 12u8);
pin_trait_impl!(crate::fmc::D14Pin, FMC, PD9, 12u8);
pin_trait_impl!(crate::fmc::DA14Pin, FMC, PD9, 12u8);
pin_trait_impl!(crate::fmc::NBL0Pin, FMC, PE0, 12u8);
pin_trait_impl!(crate::fmc::NBL1Pin, FMC, PE1, 12u8);
pin_trait_impl!(crate::fmc::D7Pin, FMC, PE10, 12u8);
pin_trait_impl!(crate::fmc::DA7Pin, FMC, PE10, 12u8);
pin_trait_impl!(crate::fmc::D8Pin, FMC, PE11, 12u8);
pin_trait_impl!(crate::fmc::DA8Pin, FMC, PE11, 12u8);
pin_trait_impl!(crate::fmc::D9Pin, FMC, PE12, 12u8);
pin_trait_impl!(crate::fmc::DA9Pin, FMC, PE12, 12u8);
pin_trait_impl!(crate::fmc::D10Pin, FMC, PE13, 12u8);
pin_trait_impl!(crate::fmc::DA10Pin, FMC, PE13, 12u8);
pin_trait_impl!(crate::fmc::D11Pin, FMC, PE14, 12u8);
pin_trait_impl!(crate::fmc::DA11Pin, FMC, PE14, 12u8);
pin_trait_impl!(crate::fmc::D12Pin, FMC, PE15, 12u8);
pin_trait_impl!(crate::fmc::DA12Pin, FMC, PE15, 12u8);
pin_trait_impl!(crate::fmc::A23Pin, FMC, PE2, 12u8);
pin_trait_impl!(crate::fmc::A19Pin, FMC, PE3, 12u8);
pin_trait_impl!(crate::fmc::A20Pin, FMC, PE4, 12u8);
pin_trait_impl!(crate::fmc::A21Pin, FMC, PE5, 12u8);
pin_trait_impl!(crate::fmc::A22Pin, FMC, PE6, 12u8);
pin_trait_impl!(crate::fmc::D4Pin, FMC, PE7, 12u8);
pin_trait_impl!(crate::fmc::DA4Pin, FMC, PE7, 12u8);
pin_trait_impl!(crate::fmc::D5Pin, FMC, PE8, 12u8);
pin_trait_impl!(crate::fmc::DA5Pin, FMC, PE8, 12u8);
pin_trait_impl!(crate::fmc::D6Pin, FMC, PE9, 12u8);
pin_trait_impl!(crate::fmc::DA6Pin, FMC, PE9, 12u8);
pin_trait_impl!(crate::fmc::A0Pin, FMC, PF0, 12u8);
pin_trait_impl!(crate::fmc::A1Pin, FMC, PF1, 12u8);
pin_trait_impl!(crate::fmc::SDNRASPin, FMC, PF11, 12u8);
pin_trait_impl!(crate::fmc::A6Pin, FMC, PF12, 12u8);
pin_trait_impl!(crate::fmc::A7Pin, FMC, PF13, 12u8);
pin_trait_impl!(crate::fmc::A8Pin, FMC, PF14, 12u8);
pin_trait_impl!(crate::fmc::A9Pin, FMC, PF15, 12u8);
pin_trait_impl!(crate::fmc::A2Pin, FMC, PF2, 12u8);
pin_trait_impl!(crate::fmc::A3Pin, FMC, PF3, 12u8);
pin_trait_impl!(crate::fmc::A4Pin, FMC, PF4, 12u8);
pin_trait_impl!(crate::fmc::A5Pin, FMC, PF5, 12u8);
pin_trait_impl!(crate::fmc::A10Pin, FMC, PG0, 12u8);
pin_trait_impl!(crate::fmc::A11Pin, FMC, PG1, 12u8);
pin_trait_impl!(crate::fmc::NE3Pin, FMC, PG10, 12u8);
pin_trait_impl!(crate::fmc::NE4Pin, FMC, PG12, 12u8);
pin_trait_impl!(crate::fmc::A24Pin, FMC, PG13, 12u8);
pin_trait_impl!(crate::fmc::A25Pin, FMC, PG14, 12u8);
pin_trait_impl!(crate::fmc::SDNCASPin, FMC, PG15, 12u8);
pin_trait_impl!(crate::fmc::A12Pin, FMC, PG2, 12u8);
pin_trait_impl!(crate::fmc::A13Pin, FMC, PG3, 12u8);
pin_trait_impl!(crate::fmc::A14Pin, FMC, PG4, 12u8);
pin_trait_impl!(crate::fmc::BA0Pin, FMC, PG4, 12u8);
pin_trait_impl!(crate::fmc::A15Pin, FMC, PG5, 12u8);
pin_trait_impl!(crate::fmc::BA1Pin, FMC, PG5, 12u8);
pin_trait_impl!(crate::fmc::NE3Pin, FMC, PG6, 12u8);
pin_trait_impl!(crate::fmc::INTPin, FMC, PG7, 12u8);
pin_trait_impl!(crate::fmc::SDCLKPin, FMC, PG8, 12u8);
pin_trait_impl!(crate::fmc::NCEPin, FMC, PG9, 12u8);
pin_trait_impl!(crate::fmc::NE2Pin, FMC, PG9, 12u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB6, 4u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB7, 4u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB8, 4u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB9, 4u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PB10, 4u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PB11, 4u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PF0, 4u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PF1, 4u8);
pin_trait_impl!(crate::i2c::SclPin, I2C3, PA8, 4u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C3, PC9, 4u8);
pin_trait_impl!(crate::i2c::SclPin, I2C4, PB6, 11u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C4, PB7, 11u8);
pin_trait_impl!(crate::i2c::SclPin, I2C4, PB8, 1u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C4, PB9, 1u8);
pin_trait_impl!(crate::i2c::SclPin, I2C4, PD12, 4u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C4, PD13, 4u8);
pin_trait_impl!(crate::i2c::SclPin, I2C4, PF14, 4u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C4, PF15, 4u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PD13, 3u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PG13, 3u8);
pin_trait_impl!(crate::ltdc::R2Pin, LTDC, PA1, 14u8);
pin_trait_impl!(crate::ltdc::B1Pin, LTDC, PA10, 14u8);
pin_trait_impl!(crate::ltdc::B4Pin, LTDC, PA10, 9u8);
pin_trait_impl!(crate::ltdc::R4Pin, LTDC, PA11, 14u8);
pin_trait_impl!(crate::ltdc::R5Pin, LTDC, PA12, 14u8);
pin_trait_impl!(crate::ltdc::R1Pin, LTDC, PA2, 14u8);
pin_trait_impl!(crate::ltdc::B2Pin, LTDC, PA3, 9u8);
pin_trait_impl!(crate::ltdc::B5Pin, LTDC, PA3, 14u8);
pin_trait_impl!(crate::ltdc::VsyncPin, LTDC, PA4, 14u8);
pin_trait_impl!(crate::ltdc::R4Pin, LTDC, PA5, 14u8);
pin_trait_impl!(crate::ltdc::G2Pin, LTDC, PA6, 14u8);
pin_trait_impl!(crate::ltdc::B3Pin, LTDC, PA8, 13u8);
pin_trait_impl!(crate::ltdc::R6Pin, LTDC, PA8, 14u8);
pin_trait_impl!(crate::ltdc::R5Pin, LTDC, PA9, 14u8);
pin_trait_impl!(crate::ltdc::G1Pin, LTDC, PB0, 14u8);
pin_trait_impl!(crate::ltdc::R3Pin, LTDC, PB0, 9u8);
pin_trait_impl!(crate::ltdc::G0Pin, LTDC, PB1, 14u8);
pin_trait_impl!(crate::ltdc::R6Pin, LTDC, PB1, 9u8);
pin_trait_impl!(crate::ltdc::G4Pin, LTDC, PB10, 14u8);
pin_trait_impl!(crate::ltdc::G5Pin, LTDC, PB11, 14u8);
pin_trait_impl!(crate::ltdc::G7Pin, LTDC, PB5, 14u8);
pin_trait_impl!(crate::ltdc::B6Pin, LTDC, PB8, 14u8);
pin_trait_impl!(crate::ltdc::B7Pin, LTDC, PB9, 14u8);
pin_trait_impl!(crate::ltdc::R5Pin, LTDC, PC0, 14u8);
pin_trait_impl!(crate::ltdc::R2Pin, LTDC, PC10, 14u8);
pin_trait_impl!(crate::ltdc::HsyncPin, LTDC, PC6, 14u8);
pin_trait_impl!(crate::ltdc::G6Pin, LTDC, PC7, 14u8);
pin_trait_impl!(crate::ltdc::B2Pin, LTDC, PC9, 14u8);
pin_trait_impl!(crate::ltdc::G3Pin, LTDC, PC9, 10u8);
pin_trait_impl!(crate::ltdc::B3Pin, LTDC, PD10, 14u8);
pin_trait_impl!(crate::ltdc::G7Pin, LTDC, PD3, 14u8);
pin_trait_impl!(crate::ltdc::B2Pin, LTDC, PD6, 14u8);
pin_trait_impl!(crate::ltdc::G3Pin, LTDC, PE11, 14u8);
pin_trait_impl!(crate::ltdc::B4Pin, LTDC, PE12, 14u8);
pin_trait_impl!(crate::ltdc::DePin, LTDC, PE13, 14u8);
pin_trait_impl!(crate::ltdc::ClkPin, LTDC, PE14, 14u8);
pin_trait_impl!(crate::ltdc::R7Pin, LTDC, PE15, 14u8);
pin_trait_impl!(crate::ltdc::B0Pin, LTDC, PE4, 14u8);
pin_trait_impl!(crate::ltdc::G0Pin, LTDC, PE5, 14u8);
pin_trait_impl!(crate::ltdc::G1Pin, LTDC, PE6, 14u8);
pin_trait_impl!(crate::ltdc::DePin, LTDC, PF10, 14u8);
pin_trait_impl!(crate::ltdc::B2Pin, LTDC, PG10, 14u8);
pin_trait_impl!(crate::ltdc::G3Pin, LTDC, PG10, 9u8);
pin_trait_impl!(crate::ltdc::B3Pin, LTDC, PG11, 14u8);
pin_trait_impl!(crate::ltdc::B1Pin, LTDC, PG12, 14u8);
pin_trait_impl!(crate::ltdc::B4Pin, LTDC, PG12, 9u8);
pin_trait_impl!(crate::ltdc::R0Pin, LTDC, PG13, 14u8);
pin_trait_impl!(crate::ltdc::B0Pin, LTDC, PG14, 14u8);
pin_trait_impl!(crate::ltdc::R7Pin, LTDC, PG6, 14u8);
pin_trait_impl!(crate::ltdc::ClkPin, LTDC, PG7, 14u8);
pin_trait_impl!(crate::ltdc::G7Pin, LTDC, PG8, 14u8);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PA1, 9u8);
pin_trait_impl!(crate::qspi::BK1NSSPin, QUADSPI, PB10, 9u8);
pin_trait_impl!(crate::qspi::SckPin, QUADSPI, PB2, 9u8);
pin_trait_impl!(crate::qspi::BK1NSSPin, QUADSPI, PB6, 10u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PC10, 9u8);
pin_trait_impl!(crate::qspi::BK2NSSPin, QUADSPI, PC11, 9u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PC9, 9u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PD11, 9u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PD12, 9u8);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PD13, 9u8);
pin_trait_impl!(crate::qspi::BK2D3Pin, QUADSPI, PE10, 10u8);
pin_trait_impl!(crate::qspi::BK1D2Pin, QUADSPI, PE2, 9u8);
pin_trait_impl!(crate::qspi::BK2D0Pin, QUADSPI, PE7, 10u8);
pin_trait_impl!(crate::qspi::BK2D1Pin, QUADSPI, PE8, 10u8);
pin_trait_impl!(crate::qspi::BK2D2Pin, QUADSPI, PE9, 10u8);
pin_trait_impl!(crate::qspi::SckPin, QUADSPI, PF10, 9u8);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PF6, 9u8);
pin_trait_impl!(crate::qspi::BK1D2Pin, QUADSPI, PF7, 9u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PF8, 10u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PF9, 10u8);
pin_trait_impl!(crate::qspi::BK2D3Pin, QUADSPI, PG14, 9u8);
pin_trait_impl!(crate::qspi::BK2D2Pin, QUADSPI, PG9, 9u8);
pin_trait_impl!(crate::rcc::McoPin, MCO1, PA8, 0u8);
pin_trait_impl!(crate::rcc::McoPin, MCO2, PC9, 0u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PB2, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PC1, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PD6, 6u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI1, PE2, 6u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI1, PE3, 6u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI1, PE4, 6u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI1, PE5, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PE6, 6u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI1, PF6, 6u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI1, PF7, 6u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI1, PF8, 6u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI1, PF9, 6u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI1, PG7, 6u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PA0, 10u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PA1, 10u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PA12, 8u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI2, PA2, 8u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PC0, 8u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI2, PD11, 10u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI2, PD12, 10u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI2, PD13, 10u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI2, PE0, 10u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PE11, 10u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI2, PE12, 10u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PE13, 10u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PE14, 10u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PE6, 10u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PF11, 10u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PG10, 10u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PG9, 10u8);
pin_trait_impl!(crate::sdmmc::D4Pin, SDMMC1, PB8, 12u8);
pin_trait_impl!(crate::sdmmc::D5Pin, SDMMC1, PB9, 12u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDMMC1, PC10, 12u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDMMC1, PC11, 12u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDMMC1, PC12, 12u8);
pin_trait_impl!(crate::sdmmc::D7Pin, SDMMC1, PC6, 12u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDMMC1, PC8, 12u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDMMC1, PC9, 12u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDMMC1, PD2, 12u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDMMC2, PB14, 10u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDMMC2, PB15, 10u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDMMC2, PB3, 10u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDMMC2, PB4, 10u8);
pin_trait_impl!(crate::sdmmc::D4Pin, SDMMC2, PB8, 10u8);
pin_trait_impl!(crate::sdmmc::D5Pin, SDMMC2, PB9, 10u8);
pin_trait_impl!(crate::sdmmc::D7Pin, SDMMC2, PC6, 10u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDMMC2, PD6, 11u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDMMC2, PD7, 11u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDMMC2, PG10, 11u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDMMC2, PG11, 10u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDMMC2, PG12, 11u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDMMC2, PG9, 11u8);
impl_spdifrx_pin!(SPDIFRX1, PC4, 8u8, 2u8);
impl_spdifrx_pin!(SPDIFRX1, PC5, 8u8, 3u8);
impl_spdifrx_pin!(SPDIFRX1, PD7, 8u8, 0u8);
impl_spdifrx_pin!(SPDIFRX1, PD8, 8u8, 1u8);
impl_spdifrx_pin!(SPDIFRX1, PG11, 7u8, 0u8);
impl_spdifrx_pin!(SPDIFRX1, PG12, 7u8, 1u8);
impl_spdifrx_pin!(SPDIFRX1, PG8, 7u8, 2u8);
impl_spdifrx_pin!(SPDIFRX1, PG9, 7u8, 3u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PA15, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA15, 5u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PA4, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA4, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PA5, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PA5, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PA6, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PA7, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PB3, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PB3, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PB4, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PB5, 5u8);
pin_trait_impl!(crate::spi::MckPin, SPI1, PC4, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PD7, 5u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PG10, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PG10, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PG11, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PG11, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PG9, 5u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PA11, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PA11, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PA12, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PA12, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PA9, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PA9, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PB10, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PB10, 5u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PB12, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PB12, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PB13, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PB13, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PB14, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PB15, 5u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PB4, 7u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PB4, 7u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PB9, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PB9, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PC1, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PC2, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PC3, 5u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PC6, 5u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PD3, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PD3, 5u8);
pin_trait_impl!(crate::spi::WsPin, SPI3, PA15, 6u8);
pin_trait_impl!(crate::spi::CsPin, SPI3, PA15, 6u8);
pin_trait_impl!(crate::spi::WsPin, SPI3, PA4, 6u8);
pin_trait_impl!(crate::spi::CsPin, SPI3, PA4, 6u8);
pin_trait_impl!(crate::spi::MosiPin, SPI3, PB2, 7u8);
pin_trait_impl!(crate::spi::CkPin, SPI3, PB3, 6u8);
pin_trait_impl!(crate::spi::SckPin, SPI3, PB3, 6u8);
pin_trait_impl!(crate::spi::MisoPin, SPI3, PB4, 6u8);
pin_trait_impl!(crate::spi::MosiPin, SPI3, PB5, 6u8);
pin_trait_impl!(crate::spi::CkPin, SPI3, PC10, 6u8);
pin_trait_impl!(crate::spi::SckPin, SPI3, PC10, 6u8);
pin_trait_impl!(crate::spi::MisoPin, SPI3, PC11, 6u8);
pin_trait_impl!(crate::spi::MosiPin, SPI3, PC12, 6u8);
pin_trait_impl!(crate::spi::MckPin, SPI3, PC7, 6u8);
pin_trait_impl!(crate::spi::MosiPin, SPI3, PD6, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI4, PE11, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI4, PE12, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI4, PE13, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI4, PE14, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI4, PE2, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI4, PE4, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI4, PE5, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI4, PE6, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI5, PF11, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI5, PF6, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI5, PF7, 5u8);
pin_trait_impl!(crate::spi::MisoPin, SPI5, PF8, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI5, PF9, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI6, PA15, 7u8);
pin_trait_impl!(crate::spi::CsPin, SPI6, PA4, 8u8);
pin_trait_impl!(crate::spi::SckPin, SPI6, PA5, 8u8);
pin_trait_impl!(crate::spi::MisoPin, SPI6, PA6, 8u8);
pin_trait_impl!(crate::spi::MosiPin, SPI6, PA7, 8u8);
pin_trait_impl!(crate::spi::SckPin, SPI6, PB3, 8u8);
pin_trait_impl!(crate::spi::MisoPin, SPI6, PB4, 8u8);
pin_trait_impl!(crate::spi::MosiPin, SPI6, PB5, 8u8);
pin_trait_impl!(crate::spi::MisoPin, SPI6, PG12, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI6, PG13, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI6, PG14, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI6, PG8, 5u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM1, PA10, 1u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM1, PA11, 1u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM1, PA12, 1u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM1, PA6, 1u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM1, PA7, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM1, PA8, 1u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM1, PA9, 1u8);
pin_trait_impl!(crate::timer::Channel2ComplementaryPin, TIM1, PB0, 1u8);
pin_trait_impl!(crate::timer::Channel3ComplementaryPin, TIM1, PB1, 1u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM1, PB12, 1u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM1, PB13, 1u8);
pin_trait_impl!(crate::timer::Channel2ComplementaryPin, TIM1, PB14, 1u8);
pin_trait_impl!(crate::timer::Channel3ComplementaryPin, TIM1, PB15, 1u8);
pin_trait_impl!(crate::timer::Channel2ComplementaryPin, TIM1, PE10, 1u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM1, PE11, 1u8);
pin_trait_impl!(crate::timer::Channel3ComplementaryPin, TIM1, PE12, 1u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM1, PE13, 1u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM1, PE14, 1u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM1, PE15, 1u8);
pin_trait_impl!(crate::timer::BreakInput2Pin, TIM1, PE6, 1u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM1, PE7, 1u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM1, PE8, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM1, PE9, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM10, PB8, 3u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM10, PF6, 3u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM11, PB9, 3u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM11, PF7, 3u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM12, PB14, 9u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM12, PB15, 9u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM13, PA6, 9u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM13, PF8, 9u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM14, PA7, 9u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM14, PF9, 9u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM2, PA0, 1u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA0, 1u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM2, PA1, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM2, PA15, 1u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA15, 1u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM2, PA2, 1u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM2, PA3, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM2, PA5, 1u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA5, 1u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM2, PB10, 1u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM2, PB11, 1u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM2, PB3, 1u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM3, PA6, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM3, PA7, 2u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM3, PB0, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM3, PB1, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM3, PB4, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM3, PB5, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM3, PC6, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM3, PC7, 2u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM3, PC8, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM3, PC9, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM3, PD2, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM4, PB6, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM4, PB7, 2u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM4, PB8, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM4, PB9, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM4, PD12, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM4, PD13, 2u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM4, PD14, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM4, PD15, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM4, PE0, 2u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM5, PA0, 2u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM5, PA1, 2u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM5, PA2, 2u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM5, PA3, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM8, PA0, 3u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM8, PA5, 3u8);
pin_trait_impl!(crate::timer::BreakInputPin, TIM8, PA6, 3u8);
pin_trait_impl!(crate::timer::Channel1ComplementaryPin, TIM8, PA7, 3u8);
pin_trait_impl!(crate::timer::BreakInput2Pin, TIM8, PA8, 3u8);
pin_trait_impl!(crate::timer::Channel2ComplementaryPin, TIM8, PB0, 3u8);
pin_trait_impl!(crate::timer::Channel3ComplementaryPin, TIM8, PB1, 3u8);
pin_trait_impl!(crate::timer::Channel2ComplementaryPin, TIM8, PB14, 3u8);
pin_trait_impl!(crate::timer::Channel3ComplementaryPin, TIM8, PB15, 3u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM8, PC6, 3u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM8, PC7, 3u8);
pin_trait_impl!(crate::timer::Channel3Pin, TIM8, PC8, 3u8);
pin_trait_impl!(crate::timer::Channel4Pin, TIM8, PC9, 3u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM9, PA2, 3u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM9, PA3, 3u8);
pin_trait_impl!(crate::timer::Channel1Pin, TIM9, PE5, 3u8);
pin_trait_impl!(crate::timer::Channel2Pin, TIM9, PE6, 3u8);
pin_trait_impl!(crate::usart::TxPin, UART4, PA0, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART4, PA1, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART4, PA11, 6u8);
pin_trait_impl!(crate::usart::TxPin, UART4, PA12, 6u8);
pin_trait_impl!(crate::usart::DePin, UART4, PA15, 8u8);
pin_trait_impl!(crate::usart::RtsPin, UART4, PA15, 8u8);
pin_trait_impl!(crate::usart::CtsPin, UART4, PB0, 8u8);
pin_trait_impl!(crate::usart::DePin, UART4, PB14, 8u8);
pin_trait_impl!(crate::usart::RtsPin, UART4, PB14, 8u8);
pin_trait_impl!(crate::usart::CtsPin, UART4, PB15, 8u8);
pin_trait_impl!(crate::usart::TxPin, UART4, PC10, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART4, PC11, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART4, PD0, 8u8);
pin_trait_impl!(crate::usart::TxPin, UART4, PD1, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART5, PB12, 8u8);
pin_trait_impl!(crate::usart::TxPin, UART5, PB13, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART5, PB5, 1u8);
pin_trait_impl!(crate::usart::TxPin, UART5, PB6, 1u8);
pin_trait_impl!(crate::usart::RxPin, UART5, PB8, 7u8);
pin_trait_impl!(crate::usart::TxPin, UART5, PB9, 7u8);
pin_trait_impl!(crate::usart::TxPin, UART5, PC12, 8u8);
pin_trait_impl!(crate::usart::DePin, UART5, PC8, 7u8);
pin_trait_impl!(crate::usart::RtsPin, UART5, PC8, 7u8);
pin_trait_impl!(crate::usart::CtsPin, UART5, PC9, 7u8);
pin_trait_impl!(crate::usart::RxPin, UART5, PD2, 8u8);
pin_trait_impl!(crate::usart::TxPin, UART7, PA15, 12u8);
pin_trait_impl!(crate::usart::RxPin, UART7, PA8, 12u8);
pin_trait_impl!(crate::usart::RxPin, UART7, PB3, 12u8);
pin_trait_impl!(crate::usart::TxPin, UART7, PB4, 12u8);
pin_trait_impl!(crate::usart::CtsPin, UART7, PE10, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART7, PE7, 8u8);
pin_trait_impl!(crate::usart::TxPin, UART7, PE8, 8u8);
pin_trait_impl!(crate::usart::DePin, UART7, PE9, 8u8);
pin_trait_impl!(crate::usart::RtsPin, UART7, PE9, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART7, PF6, 8u8);
pin_trait_impl!(crate::usart::TxPin, UART7, PF7, 8u8);
pin_trait_impl!(crate::usart::DePin, UART7, PF8, 8u8);
pin_trait_impl!(crate::usart::RtsPin, UART7, PF8, 8u8);
pin_trait_impl!(crate::usart::CtsPin, UART7, PF9, 8u8);
pin_trait_impl!(crate::usart::CtsPin, UART8, PD14, 8u8);
pin_trait_impl!(crate::usart::DePin, UART8, PD15, 8u8);
pin_trait_impl!(crate::usart::RtsPin, UART8, PD15, 8u8);
pin_trait_impl!(crate::usart::RxPin, UART8, PE0, 8u8);
pin_trait_impl!(crate::usart::TxPin, UART8, PE1, 8u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PA10, 7u8);
pin_trait_impl!(crate::usart::CtsPin, USART1, PA11, 7u8);
pin_trait_impl!(crate::usart::DePin, USART1, PA12, 7u8);
pin_trait_impl!(crate::usart::RtsPin, USART1, PA12, 7u8);
pin_trait_impl!(crate::usart::CkPin, USART1, PA8, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PA9, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PB14, 4u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PB15, 4u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PB6, 7u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PB7, 7u8);
pin_trait_impl!(crate::usart::CtsPin, USART2, PA0, 7u8);
pin_trait_impl!(crate::usart::DePin, USART2, PA1, 7u8);
pin_trait_impl!(crate::usart::RtsPin, USART2, PA1, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PA2, 7u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PA3, 7u8);
pin_trait_impl!(crate::usart::CkPin, USART2, PA4, 7u8);
pin_trait_impl!(crate::usart::CtsPin, USART2, PD3, 7u8);
pin_trait_impl!(crate::usart::DePin, USART2, PD4, 7u8);
pin_trait_impl!(crate::usart::RtsPin, USART2, PD4, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PD5, 7u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PD6, 7u8);
pin_trait_impl!(crate::usart::CkPin, USART2, PD7, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PB10, 7u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PB11, 7u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PB12, 7u8);
pin_trait_impl!(crate::usart::CtsPin, USART3, PB13, 7u8);
pin_trait_impl!(crate::usart::DePin, USART3, PB14, 7u8);
pin_trait_impl!(crate::usart::RtsPin, USART3, PB14, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PC10, 7u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PC11, 7u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PC12, 7u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PD10, 7u8);
pin_trait_impl!(crate::usart::CtsPin, USART3, PD11, 7u8);
pin_trait_impl!(crate::usart::DePin, USART3, PD12, 7u8);
pin_trait_impl!(crate::usart::RtsPin, USART3, PD12, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PD8, 7u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PD9, 7u8);
pin_trait_impl!(crate::usart::TxPin, USART6, PC6, 8u8);
pin_trait_impl!(crate::usart::RxPin, USART6, PC7, 8u8);
pin_trait_impl!(crate::usart::CkPin, USART6, PC8, 8u8);
pin_trait_impl!(crate::usart::DePin, USART6, PG12, 8u8);
pin_trait_impl!(crate::usart::RtsPin, USART6, PG12, 8u8);
pin_trait_impl!(crate::usart::CtsPin, USART6, PG13, 8u8);
pin_trait_impl!(crate::usart::TxPin, USART6, PG14, 8u8);
pin_trait_impl!(crate::usart::CtsPin, USART6, PG15, 8u8);
pin_trait_impl!(crate::usart::CkPin, USART6, PG7, 8u8);
pin_trait_impl!(crate::usart::DePin, USART6, PG8, 8u8);
pin_trait_impl!(crate::usart::RtsPin, USART6, PG8, 8u8);
pin_trait_impl!(crate::usart::RxPin, USART6, PG9, 8u8);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_FS, PA11, 10u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_FS, PA12, 10u8);
pin_trait_impl!(crate::usb::UlpiD0Pin, USB_OTG_HS, PA3, 10u8);
pin_trait_impl!(crate::usb::UlpiClkPin, USB_OTG_HS, PA5, 10u8);
pin_trait_impl!(crate::usb::UlpiD1Pin, USB_OTG_HS, PB0, 10u8);
pin_trait_impl!(crate::usb::UlpiD2Pin, USB_OTG_HS, PB1, 10u8);
pin_trait_impl!(crate::usb::UlpiD3Pin, USB_OTG_HS, PB10, 10u8);
pin_trait_impl!(crate::usb::UlpiD4Pin, USB_OTG_HS, PB11, 10u8);
pin_trait_impl!(crate::usb::UlpiD5Pin, USB_OTG_HS, PB12, 10u8);
pin_trait_impl!(crate::usb::UlpiD6Pin, USB_OTG_HS, PB13, 10u8);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_HS, PB14, 12u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_HS, PB15, 12u8);
pin_trait_impl!(crate::usb::UlpiD7Pin, USB_OTG_HS, PB5, 10u8);
pin_trait_impl!(crate::usb::UlpiStpPin, USB_OTG_HS, PC0, 10u8);
pin_trait_impl!(crate::usb::UlpiDirPin, USB_OTG_HS, PC2, 10u8);
pin_trait_impl!(crate::usb::UlpiNxtPin, USB_OTG_HS, PC3, 10u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH0, 0u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH4, 0u8);
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH2, 1u8);
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH3, 1u8);
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH0, 2u8);
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH1, 2u8);
dma_trait_impl!(crate::cryp::DmaOut, CRYP, DMA2_CH5, 2u8);
dma_trait_impl!(crate::cryp::DmaIn, CRYP, DMA2_CH6, 2u8);
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH5, 7u8);
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH6, 7u8);
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH1, 1u8);
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH7, 1u8);
dma_trait_impl!(crate::hash::Dma, HASH, DMA2_CH7, 2u8);
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH0, 1u8);
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH5, 1u8);
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH6, 1u8);
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH7, 1u8);
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH2, 7u8);
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH3, 7u8);
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH4, 8u8);
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH7, 7u8);
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH0, 8u8);
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH1, 1u8);
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH2, 3u8);
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH4, 3u8);
dma_trait_impl!(crate::i2c::RxDma, I2C4, DMA1_CH1, 8u8);
dma_trait_impl!(crate::i2c::RxDma, I2C4, DMA1_CH2, 2u8);
dma_trait_impl!(crate::i2c::TxDma, I2C4, DMA1_CH5, 2u8);
dma_trait_impl!(crate::i2c::TxDma, I2C4, DMA1_CH6, 8u8);
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, DMA2_CH2, 11u8);
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, DMA2_CH7, 3u8);
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH0, 10u8);
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH1, 0u8);
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH3, 0u8);
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH4, 1u8);
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH5, 0u8);
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH6, 10u8);
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH1, 10u8);
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH2, 10u8);
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH4, 3u8);
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH6, 3u8);
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH7, 0u8);
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC1, DMA2_CH3, 4u8);
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC1, DMA2_CH6, 4u8);
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC2, DMA2_CH0, 11u8);
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC2, DMA2_CH5, 11u8);
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH1, 0u8);
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH6, 0u8);
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH0, 3u8);
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH2, 3u8);
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH3, 3u8);
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH5, 3u8);
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH1, 9u8);
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH3, 0u8);
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH4, 0u8);
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH6, 9u8);
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH0, 0u8);
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH2, 0u8);
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH5, 0u8);
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH7, 0u8);
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH0, 4u8);
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH1, 4u8);
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH2, 9u8);
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH3, 5u8);
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH4, 5u8);
dma_trait_impl!(crate::spi::RxDma, SPI5, DMA2_CH3, 2u8);
dma_trait_impl!(crate::spi::TxDma, SPI5, DMA2_CH4, 2u8);
dma_trait_impl!(crate::spi::RxDma, SPI5, DMA2_CH5, 7u8);
dma_trait_impl!(crate::spi::TxDma, SPI5, DMA2_CH6, 7u8);
dma_trait_impl!(crate::spi::TxDma, SPI6, DMA2_CH5, 1u8);
dma_trait_impl!(crate::spi::RxDma, SPI6, DMA2_CH6, 1u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM1, DMA2_CH1, 6u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM1, DMA2_CH2, 6u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM1, DMA2_CH3, 6u8);
dma_trait_impl!(crate::timer::Ch4Dma, TIM1, DMA2_CH4, 6u8);
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH5, 6u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM1, DMA2_CH6, 0u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM1, DMA2_CH6, 0u8);
dma_trait_impl!(crate::timer::Ch3Dma, TIM1, DMA2_CH6, 0u8);
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH1, 3u8);
dma_trait_impl!(crate::timer::Ch3Dma, TIM2, DMA1_CH1, 3u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM2, DMA1_CH5, 3u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM2, DMA1_CH6, 3u8);
dma_trait_impl!(crate::timer::Ch4Dma, TIM2, DMA1_CH6, 3u8);
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH7, 3u8);
dma_trait_impl!(crate::timer::Ch4Dma, TIM2, DMA1_CH7, 3u8);
dma_trait_impl!(crate::timer::Ch4Dma, TIM3, DMA1_CH2, 5u8);
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH2, 5u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM3, DMA1_CH4, 5u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM3, DMA1_CH5, 5u8);
dma_trait_impl!(crate::timer::Ch3Dma, TIM3, DMA1_CH7, 5u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM4, DMA1_CH0, 2u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM4, DMA1_CH3, 2u8);
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH6, 2u8);
dma_trait_impl!(crate::timer::Ch3Dma, TIM4, DMA1_CH7, 2u8);
dma_trait_impl!(crate::timer::Ch3Dma, TIM5, DMA1_CH0, 6u8);
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH0, 6u8);
dma_trait_impl!(crate::timer::Ch4Dma, TIM5, DMA1_CH1, 6u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM5, DMA1_CH2, 6u8);
dma_trait_impl!(crate::timer::Ch4Dma, TIM5, DMA1_CH3, 6u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM5, DMA1_CH4, 6u8);
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH6, 6u8);
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH1, 7u8);
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH2, 1u8);
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH4, 1u8);
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH1, 7u8);
dma_trait_impl!(crate::timer::Ch1Dma, TIM8, DMA2_CH2, 0u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM8, DMA2_CH2, 0u8);
dma_trait_impl!(crate::timer::Ch3Dma, TIM8, DMA2_CH2, 0u8);
dma_trait_impl!(crate::timer::Ch2Dma, TIM8, DMA2_CH3, 7u8);
dma_trait_impl!(crate::timer::Ch3Dma, TIM8, DMA2_CH4, 7u8);
dma_trait_impl!(crate::timer::Ch4Dma, TIM8, DMA2_CH7, 7u8);
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH2, 4u8);
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH4, 4u8);
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH0, 4u8);
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH7, 4u8);
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH1, 5u8);
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH3, 5u8);
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH0, 5u8);
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH6, 5u8);
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH2, 4u8);
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH5, 4u8);
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH7, 4u8);
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH5, 4u8);
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH6, 4u8);
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH1, 4u8);
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH3, 4u8);
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH4, 7u8);
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH1, 5u8);
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH2, 5u8);
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH6, 5u8);
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH7, 5u8);
impl core::ops::Div<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 1u32 / 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 1u32 / 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 1u32 / 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 128u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 256u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 512u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Mcopre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Mcopre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Mcopre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Mcopre::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Mcopre::DIV5 => self * 1u32 / 5u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Mcopre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Mcopre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Mcopre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV5 => self * 5u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllm> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllm) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllm::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllm::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllm::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllm::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllm::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllm::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllm::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Pllm::DIV9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Pllm::DIV10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Pllm::DIV11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Pllm::DIV12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Pllm::DIV13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Pllm::DIV14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Pllm::DIV15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Pllm::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Pllm::DIV17 => self * 1u32 / 17u32,
            crate::pac::rcc::vals::Pllm::DIV18 => self * 1u32 / 18u32,
            crate::pac::rcc::vals::Pllm::DIV19 => self * 1u32 / 19u32,
            crate::pac::rcc::vals::Pllm::DIV20 => self * 1u32 / 20u32,
            crate::pac::rcc::vals::Pllm::DIV21 => self * 1u32 / 21u32,
            crate::pac::rcc::vals::Pllm::DIV22 => self * 1u32 / 22u32,
            crate::pac::rcc::vals::Pllm::DIV23 => self * 1u32 / 23u32,
            crate::pac::rcc::vals::Pllm::DIV24 => self * 1u32 / 24u32,
            crate::pac::rcc::vals::Pllm::DIV25 => self * 1u32 / 25u32,
            crate::pac::rcc::vals::Pllm::DIV26 => self * 1u32 / 26u32,
            crate::pac::rcc::vals::Pllm::DIV27 => self * 1u32 / 27u32,
            crate::pac::rcc::vals::Pllm::DIV28 => self * 1u32 / 28u32,
            crate::pac::rcc::vals::Pllm::DIV29 => self * 1u32 / 29u32,
            crate::pac::rcc::vals::Pllm::DIV30 => self * 1u32 / 30u32,
            crate::pac::rcc::vals::Pllm::DIV31 => self * 1u32 / 31u32,
            crate::pac::rcc::vals::Pllm::DIV32 => self * 1u32 / 32u32,
            crate::pac::rcc::vals::Pllm::DIV33 => self * 1u32 / 33u32,
            crate::pac::rcc::vals::Pllm::DIV34 => self * 1u32 / 34u32,
            crate::pac::rcc::vals::Pllm::DIV35 => self * 1u32 / 35u32,
            crate::pac::rcc::vals::Pllm::DIV36 => self * 1u32 / 36u32,
            crate::pac::rcc::vals::Pllm::DIV37 => self * 1u32 / 37u32,
            crate::pac::rcc::vals::Pllm::DIV38 => self * 1u32 / 38u32,
            crate::pac::rcc::vals::Pllm::DIV39 => self * 1u32 / 39u32,
            crate::pac::rcc::vals::Pllm::DIV40 => self * 1u32 / 40u32,
            crate::pac::rcc::vals::Pllm::DIV41 => self * 1u32 / 41u32,
            crate::pac::rcc::vals::Pllm::DIV42 => self * 1u32 / 42u32,
            crate::pac::rcc::vals::Pllm::DIV43 => self * 1u32 / 43u32,
            crate::pac::rcc::vals::Pllm::DIV44 => self * 1u32 / 44u32,
            crate::pac::rcc::vals::Pllm::DIV45 => self * 1u32 / 45u32,
            crate::pac::rcc::vals::Pllm::DIV46 => self * 1u32 / 46u32,
            crate::pac::rcc::vals::Pllm::DIV47 => self * 1u32 / 47u32,
            crate::pac::rcc::vals::Pllm::DIV48 => self * 1u32 / 48u32,
            crate::pac::rcc::vals::Pllm::DIV49 => self * 1u32 / 49u32,
            crate::pac::rcc::vals::Pllm::DIV50 => self * 1u32 / 50u32,
            crate::pac::rcc::vals::Pllm::DIV51 => self * 1u32 / 51u32,
            crate::pac::rcc::vals::Pllm::DIV52 => self * 1u32 / 52u32,
            crate::pac::rcc::vals::Pllm::DIV53 => self * 1u32 / 53u32,
            crate::pac::rcc::vals::Pllm::DIV54 => self * 1u32 / 54u32,
            crate::pac::rcc::vals::Pllm::DIV55 => self * 1u32 / 55u32,
            crate::pac::rcc::vals::Pllm::DIV56 => self * 1u32 / 56u32,
            crate::pac::rcc::vals::Pllm::DIV57 => self * 1u32 / 57u32,
            crate::pac::rcc::vals::Pllm::DIV58 => self * 1u32 / 58u32,
            crate::pac::rcc::vals::Pllm::DIV59 => self * 1u32 / 59u32,
            crate::pac::rcc::vals::Pllm::DIV60 => self * 1u32 / 60u32,
            crate::pac::rcc::vals::Pllm::DIV61 => self * 1u32 / 61u32,
            crate::pac::rcc::vals::Pllm::DIV62 => self * 1u32 / 62u32,
            crate::pac::rcc::vals::Pllm::DIV63 => self * 1u32 / 63u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllm> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllm) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllm::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV17 => self * 17u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV18 => self * 18u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV19 => self * 19u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV20 => self * 20u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV21 => self * 21u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV22 => self * 22u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV23 => self * 23u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV24 => self * 24u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV25 => self * 25u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV26 => self * 26u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV27 => self * 27u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV28 => self * 28u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV29 => self * 29u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV30 => self * 30u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV31 => self * 31u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV32 => self * 32u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV33 => self * 33u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV34 => self * 34u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV35 => self * 35u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV36 => self * 36u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV37 => self * 37u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV38 => self * 38u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV39 => self * 39u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV40 => self * 40u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV41 => self * 41u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV42 => self * 42u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV43 => self * 43u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV44 => self * 44u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV45 => self * 45u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV46 => self * 46u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV47 => self * 47u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV48 => self * 48u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV49 => self * 49u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV50 => self * 50u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV51 => self * 51u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV52 => self * 52u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV53 => self * 53u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV54 => self * 54u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV55 => self * 55u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV56 => self * 56u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV57 => self * 57u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV58 => self * 58u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV59 => self * 59u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV60 => self * 60u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV61 => self * 61u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV62 => self * 62u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV63 => self * 63u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Plln> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Plln) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Plln::MUL50 => self * 1u32 / 50u32,
            crate::pac::rcc::vals::Plln::MUL51 => self * 1u32 / 51u32,
            crate::pac::rcc::vals::Plln::MUL52 => self * 1u32 / 52u32,
            crate::pac::rcc::vals::Plln::MUL53 => self * 1u32 / 53u32,
            crate::pac::rcc::vals::Plln::MUL54 => self * 1u32 / 54u32,
            crate::pac::rcc::vals::Plln::MUL55 => self * 1u32 / 55u32,
            crate::pac::rcc::vals::Plln::MUL56 => self * 1u32 / 56u32,
            crate::pac::rcc::vals::Plln::MUL57 => self * 1u32 / 57u32,
            crate::pac::rcc::vals::Plln::MUL58 => self * 1u32 / 58u32,
            crate::pac::rcc::vals::Plln::MUL59 => self * 1u32 / 59u32,
            crate::pac::rcc::vals::Plln::MUL60 => self * 1u32 / 60u32,
            crate::pac::rcc::vals::Plln::MUL61 => self * 1u32 / 61u32,
            crate::pac::rcc::vals::Plln::MUL62 => self * 1u32 / 62u32,
            crate::pac::rcc::vals::Plln::MUL63 => self * 1u32 / 63u32,
            crate::pac::rcc::vals::Plln::MUL64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Plln::MUL65 => self * 1u32 / 65u32,
            crate::pac::rcc::vals::Plln::MUL66 => self * 1u32 / 66u32,
            crate::pac::rcc::vals::Plln::MUL67 => self * 1u32 / 67u32,
            crate::pac::rcc::vals::Plln::MUL68 => self * 1u32 / 68u32,
            crate::pac::rcc::vals::Plln::MUL69 => self * 1u32 / 69u32,
            crate::pac::rcc::vals::Plln::MUL70 => self * 1u32 / 70u32,
            crate::pac::rcc::vals::Plln::MUL71 => self * 1u32 / 71u32,
            crate::pac::rcc::vals::Plln::MUL72 => self * 1u32 / 72u32,
            crate::pac::rcc::vals::Plln::MUL73 => self * 1u32 / 73u32,
            crate::pac::rcc::vals::Plln::MUL74 => self * 1u32 / 74u32,
            crate::pac::rcc::vals::Plln::MUL75 => self * 1u32 / 75u32,
            crate::pac::rcc::vals::Plln::MUL76 => self * 1u32 / 76u32,
            crate::pac::rcc::vals::Plln::MUL77 => self * 1u32 / 77u32,
            crate::pac::rcc::vals::Plln::MUL78 => self * 1u32 / 78u32,
            crate::pac::rcc::vals::Plln::MUL79 => self * 1u32 / 79u32,
            crate::pac::rcc::vals::Plln::MUL80 => self * 1u32 / 80u32,
            crate::pac::rcc::vals::Plln::MUL81 => self * 1u32 / 81u32,
            crate::pac::rcc::vals::Plln::MUL82 => self * 1u32 / 82u32,
            crate::pac::rcc::vals::Plln::MUL83 => self * 1u32 / 83u32,
            crate::pac::rcc::vals::Plln::MUL84 => self * 1u32 / 84u32,
            crate::pac::rcc::vals::Plln::MUL85 => self * 1u32 / 85u32,
            crate::pac::rcc::vals::Plln::MUL86 => self * 1u32 / 86u32,
            crate::pac::rcc::vals::Plln::MUL87 => self * 1u32 / 87u32,
            crate::pac::rcc::vals::Plln::MUL88 => self * 1u32 / 88u32,
            crate::pac::rcc::vals::Plln::MUL89 => self * 1u32 / 89u32,
            crate::pac::rcc::vals::Plln::MUL90 => self * 1u32 / 90u32,
            crate::pac::rcc::vals::Plln::MUL91 => self * 1u32 / 91u32,
            crate::pac::rcc::vals::Plln::MUL92 => self * 1u32 / 92u32,
            crate::pac::rcc::vals::Plln::MUL93 => self * 1u32 / 93u32,
            crate::pac::rcc::vals::Plln::MUL94 => self * 1u32 / 94u32,
            crate::pac::rcc::vals::Plln::MUL95 => self * 1u32 / 95u32,
            crate::pac::rcc::vals::Plln::MUL96 => self * 1u32 / 96u32,
            crate::pac::rcc::vals::Plln::MUL97 => self * 1u32 / 97u32,
            crate::pac::rcc::vals::Plln::MUL98 => self * 1u32 / 98u32,
            crate::pac::rcc::vals::Plln::MUL99 => self * 1u32 / 99u32,
            crate::pac::rcc::vals::Plln::MUL100 => self * 1u32 / 100u32,
            crate::pac::rcc::vals::Plln::MUL101 => self * 1u32 / 101u32,
            crate::pac::rcc::vals::Plln::MUL102 => self * 1u32 / 102u32,
            crate::pac::rcc::vals::Plln::MUL103 => self * 1u32 / 103u32,
            crate::pac::rcc::vals::Plln::MUL104 => self * 1u32 / 104u32,
            crate::pac::rcc::vals::Plln::MUL105 => self * 1u32 / 105u32,
            crate::pac::rcc::vals::Plln::MUL106 => self * 1u32 / 106u32,
            crate::pac::rcc::vals::Plln::MUL107 => self * 1u32 / 107u32,
            crate::pac::rcc::vals::Plln::MUL108 => self * 1u32 / 108u32,
            crate::pac::rcc::vals::Plln::MUL109 => self * 1u32 / 109u32,
            crate::pac::rcc::vals::Plln::MUL110 => self * 1u32 / 110u32,
            crate::pac::rcc::vals::Plln::MUL111 => self * 1u32 / 111u32,
            crate::pac::rcc::vals::Plln::MUL112 => self * 1u32 / 112u32,
            crate::pac::rcc::vals::Plln::MUL113 => self * 1u32 / 113u32,
            crate::pac::rcc::vals::Plln::MUL114 => self * 1u32 / 114u32,
            crate::pac::rcc::vals::Plln::MUL115 => self * 1u32 / 115u32,
            crate::pac::rcc::vals::Plln::MUL116 => self * 1u32 / 116u32,
            crate::pac::rcc::vals::Plln::MUL117 => self * 1u32 / 117u32,
            crate::pac::rcc::vals::Plln::MUL118 => self * 1u32 / 118u32,
            crate::pac::rcc::vals::Plln::MUL119 => self * 1u32 / 119u32,
            crate::pac::rcc::vals::Plln::MUL120 => self * 1u32 / 120u32,
            crate::pac::rcc::vals::Plln::MUL121 => self * 1u32 / 121u32,
            crate::pac::rcc::vals::Plln::MUL122 => self * 1u32 / 122u32,
            crate::pac::rcc::vals::Plln::MUL123 => self * 1u32 / 123u32,
            crate::pac::rcc::vals::Plln::MUL124 => self * 1u32 / 124u32,
            crate::pac::rcc::vals::Plln::MUL125 => self * 1u32 / 125u32,
            crate::pac::rcc::vals::Plln::MUL126 => self * 1u32 / 126u32,
            crate::pac::rcc::vals::Plln::MUL127 => self * 1u32 / 127u32,
            crate::pac::rcc::vals::Plln::MUL128 => self * 1u32 / 128u32,
            crate::pac::rcc::vals::Plln::MUL129 => self * 1u32 / 129u32,
            crate::pac::rcc::vals::Plln::MUL130 => self * 1u32 / 130u32,
            crate::pac::rcc::vals::Plln::MUL131 => self * 1u32 / 131u32,
            crate::pac::rcc::vals::Plln::MUL132 => self * 1u32 / 132u32,
            crate::pac::rcc::vals::Plln::MUL133 => self * 1u32 / 133u32,
            crate::pac::rcc::vals::Plln::MUL134 => self * 1u32 / 134u32,
            crate::pac::rcc::vals::Plln::MUL135 => self * 1u32 / 135u32,
            crate::pac::rcc::vals::Plln::MUL136 => self * 1u32 / 136u32,
            crate::pac::rcc::vals::Plln::MUL137 => self * 1u32 / 137u32,
            crate::pac::rcc::vals::Plln::MUL138 => self * 1u32 / 138u32,
            crate::pac::rcc::vals::Plln::MUL139 => self * 1u32 / 139u32,
            crate::pac::rcc::vals::Plln::MUL140 => self * 1u32 / 140u32,
            crate::pac::rcc::vals::Plln::MUL141 => self * 1u32 / 141u32,
            crate::pac::rcc::vals::Plln::MUL142 => self * 1u32 / 142u32,
            crate::pac::rcc::vals::Plln::MUL143 => self * 1u32 / 143u32,
            crate::pac::rcc::vals::Plln::MUL144 => self * 1u32 / 144u32,
            crate::pac::rcc::vals::Plln::MUL145 => self * 1u32 / 145u32,
            crate::pac::rcc::vals::Plln::MUL146 => self * 1u32 / 146u32,
            crate::pac::rcc::vals::Plln::MUL147 => self * 1u32 / 147u32,
            crate::pac::rcc::vals::Plln::MUL148 => self * 1u32 / 148u32,
            crate::pac::rcc::vals::Plln::MUL149 => self * 1u32 / 149u32,
            crate::pac::rcc::vals::Plln::MUL150 => self * 1u32 / 150u32,
            crate::pac::rcc::vals::Plln::MUL151 => self * 1u32 / 151u32,
            crate::pac::rcc::vals::Plln::MUL152 => self * 1u32 / 152u32,
            crate::pac::rcc::vals::Plln::MUL153 => self * 1u32 / 153u32,
            crate::pac::rcc::vals::Plln::MUL154 => self * 1u32 / 154u32,
            crate::pac::rcc::vals::Plln::MUL155 => self * 1u32 / 155u32,
            crate::pac::rcc::vals::Plln::MUL156 => self * 1u32 / 156u32,
            crate::pac::rcc::vals::Plln::MUL157 => self * 1u32 / 157u32,
            crate::pac::rcc::vals::Plln::MUL158 => self * 1u32 / 158u32,
            crate::pac::rcc::vals::Plln::MUL159 => self * 1u32 / 159u32,
            crate::pac::rcc::vals::Plln::MUL160 => self * 1u32 / 160u32,
            crate::pac::rcc::vals::Plln::MUL161 => self * 1u32 / 161u32,
            crate::pac::rcc::vals::Plln::MUL162 => self * 1u32 / 162u32,
            crate::pac::rcc::vals::Plln::MUL163 => self * 1u32 / 163u32,
            crate::pac::rcc::vals::Plln::MUL164 => self * 1u32 / 164u32,
            crate::pac::rcc::vals::Plln::MUL165 => self * 1u32 / 165u32,
            crate::pac::rcc::vals::Plln::MUL166 => self * 1u32 / 166u32,
            crate::pac::rcc::vals::Plln::MUL167 => self * 1u32 / 167u32,
            crate::pac::rcc::vals::Plln::MUL168 => self * 1u32 / 168u32,
            crate::pac::rcc::vals::Plln::MUL169 => self * 1u32 / 169u32,
            crate::pac::rcc::vals::Plln::MUL170 => self * 1u32 / 170u32,
            crate::pac::rcc::vals::Plln::MUL171 => self * 1u32 / 171u32,
            crate::pac::rcc::vals::Plln::MUL172 => self * 1u32 / 172u32,
            crate::pac::rcc::vals::Plln::MUL173 => self * 1u32 / 173u32,
            crate::pac::rcc::vals::Plln::MUL174 => self * 1u32 / 174u32,
            crate::pac::rcc::vals::Plln::MUL175 => self * 1u32 / 175u32,
            crate::pac::rcc::vals::Plln::MUL176 => self * 1u32 / 176u32,
            crate::pac::rcc::vals::Plln::MUL177 => self * 1u32 / 177u32,
            crate::pac::rcc::vals::Plln::MUL178 => self * 1u32 / 178u32,
            crate::pac::rcc::vals::Plln::MUL179 => self * 1u32 / 179u32,
            crate::pac::rcc::vals::Plln::MUL180 => self * 1u32 / 180u32,
            crate::pac::rcc::vals::Plln::MUL181 => self * 1u32 / 181u32,
            crate::pac::rcc::vals::Plln::MUL182 => self * 1u32 / 182u32,
            crate::pac::rcc::vals::Plln::MUL183 => self * 1u32 / 183u32,
            crate::pac::rcc::vals::Plln::MUL184 => self * 1u32 / 184u32,
            crate::pac::rcc::vals::Plln::MUL185 => self * 1u32 / 185u32,
            crate::pac::rcc::vals::Plln::MUL186 => self * 1u32 / 186u32,
            crate::pac::rcc::vals::Plln::MUL187 => self * 1u32 / 187u32,
            crate::pac::rcc::vals::Plln::MUL188 => self * 1u32 / 188u32,
            crate::pac::rcc::vals::Plln::MUL189 => self * 1u32 / 189u32,
            crate::pac::rcc::vals::Plln::MUL190 => self * 1u32 / 190u32,
            crate::pac::rcc::vals::Plln::MUL191 => self * 1u32 / 191u32,
            crate::pac::rcc::vals::Plln::MUL192 => self * 1u32 / 192u32,
            crate::pac::rcc::vals::Plln::MUL193 => self * 1u32 / 193u32,
            crate::pac::rcc::vals::Plln::MUL194 => self * 1u32 / 194u32,
            crate::pac::rcc::vals::Plln::MUL195 => self * 1u32 / 195u32,
            crate::pac::rcc::vals::Plln::MUL196 => self * 1u32 / 196u32,
            crate::pac::rcc::vals::Plln::MUL197 => self * 1u32 / 197u32,
            crate::pac::rcc::vals::Plln::MUL198 => self * 1u32 / 198u32,
            crate::pac::rcc::vals::Plln::MUL199 => self * 1u32 / 199u32,
            crate::pac::rcc::vals::Plln::MUL200 => self * 1u32 / 200u32,
            crate::pac::rcc::vals::Plln::MUL201 => self * 1u32 / 201u32,
            crate::pac::rcc::vals::Plln::MUL202 => self * 1u32 / 202u32,
            crate::pac::rcc::vals::Plln::MUL203 => self * 1u32 / 203u32,
            crate::pac::rcc::vals::Plln::MUL204 => self * 1u32 / 204u32,
            crate::pac::rcc::vals::Plln::MUL205 => self * 1u32 / 205u32,
            crate::pac::rcc::vals::Plln::MUL206 => self * 1u32 / 206u32,
            crate::pac::rcc::vals::Plln::MUL207 => self * 1u32 / 207u32,
            crate::pac::rcc::vals::Plln::MUL208 => self * 1u32 / 208u32,
            crate::pac::rcc::vals::Plln::MUL209 => self * 1u32 / 209u32,
            crate::pac::rcc::vals::Plln::MUL210 => self * 1u32 / 210u32,
            crate::pac::rcc::vals::Plln::MUL211 => self * 1u32 / 211u32,
            crate::pac::rcc::vals::Plln::MUL212 => self * 1u32 / 212u32,
            crate::pac::rcc::vals::Plln::MUL213 => self * 1u32 / 213u32,
            crate::pac::rcc::vals::Plln::MUL214 => self * 1u32 / 214u32,
            crate::pac::rcc::vals::Plln::MUL215 => self * 1u32 / 215u32,
            crate::pac::rcc::vals::Plln::MUL216 => self * 1u32 / 216u32,
            crate::pac::rcc::vals::Plln::MUL217 => self * 1u32 / 217u32,
            crate::pac::rcc::vals::Plln::MUL218 => self * 1u32 / 218u32,
            crate::pac::rcc::vals::Plln::MUL219 => self * 1u32 / 219u32,
            crate::pac::rcc::vals::Plln::MUL220 => self * 1u32 / 220u32,
            crate::pac::rcc::vals::Plln::MUL221 => self * 1u32 / 221u32,
            crate::pac::rcc::vals::Plln::MUL222 => self * 1u32 / 222u32,
            crate::pac::rcc::vals::Plln::MUL223 => self * 1u32 / 223u32,
            crate::pac::rcc::vals::Plln::MUL224 => self * 1u32 / 224u32,
            crate::pac::rcc::vals::Plln::MUL225 => self * 1u32 / 225u32,
            crate::pac::rcc::vals::Plln::MUL226 => self * 1u32 / 226u32,
            crate::pac::rcc::vals::Plln::MUL227 => self * 1u32 / 227u32,
            crate::pac::rcc::vals::Plln::MUL228 => self * 1u32 / 228u32,
            crate::pac::rcc::vals::Plln::MUL229 => self * 1u32 / 229u32,
            crate::pac::rcc::vals::Plln::MUL230 => self * 1u32 / 230u32,
            crate::pac::rcc::vals::Plln::MUL231 => self * 1u32 / 231u32,
            crate::pac::rcc::vals::Plln::MUL232 => self * 1u32 / 232u32,
            crate::pac::rcc::vals::Plln::MUL233 => self * 1u32 / 233u32,
            crate::pac::rcc::vals::Plln::MUL234 => self * 1u32 / 234u32,
            crate::pac::rcc::vals::Plln::MUL235 => self * 1u32 / 235u32,
            crate::pac::rcc::vals::Plln::MUL236 => self * 1u32 / 236u32,
            crate::pac::rcc::vals::Plln::MUL237 => self * 1u32 / 237u32,
            crate::pac::rcc::vals::Plln::MUL238 => self * 1u32 / 238u32,
            crate::pac::rcc::vals::Plln::MUL239 => self * 1u32 / 239u32,
            crate::pac::rcc::vals::Plln::MUL240 => self * 1u32 / 240u32,
            crate::pac::rcc::vals::Plln::MUL241 => self * 1u32 / 241u32,
            crate::pac::rcc::vals::Plln::MUL242 => self * 1u32 / 242u32,
            crate::pac::rcc::vals::Plln::MUL243 => self * 1u32 / 243u32,
            crate::pac::rcc::vals::Plln::MUL244 => self * 1u32 / 244u32,
            crate::pac::rcc::vals::Plln::MUL245 => self * 1u32 / 245u32,
            crate::pac::rcc::vals::Plln::MUL246 => self * 1u32 / 246u32,
            crate::pac::rcc::vals::Plln::MUL247 => self * 1u32 / 247u32,
            crate::pac::rcc::vals::Plln::MUL248 => self * 1u32 / 248u32,
            crate::pac::rcc::vals::Plln::MUL249 => self * 1u32 / 249u32,
            crate::pac::rcc::vals::Plln::MUL250 => self * 1u32 / 250u32,
            crate::pac::rcc::vals::Plln::MUL251 => self * 1u32 / 251u32,
            crate::pac::rcc::vals::Plln::MUL252 => self * 1u32 / 252u32,
            crate::pac::rcc::vals::Plln::MUL253 => self * 1u32 / 253u32,
            crate::pac::rcc::vals::Plln::MUL254 => self * 1u32 / 254u32,
            crate::pac::rcc::vals::Plln::MUL255 => self * 1u32 / 255u32,
            crate::pac::rcc::vals::Plln::MUL256 => self * 1u32 / 256u32,
            crate::pac::rcc::vals::Plln::MUL257 => self * 1u32 / 257u32,
            crate::pac::rcc::vals::Plln::MUL258 => self * 1u32 / 258u32,
            crate::pac::rcc::vals::Plln::MUL259 => self * 1u32 / 259u32,
            crate::pac::rcc::vals::Plln::MUL260 => self * 1u32 / 260u32,
            crate::pac::rcc::vals::Plln::MUL261 => self * 1u32 / 261u32,
            crate::pac::rcc::vals::Plln::MUL262 => self * 1u32 / 262u32,
            crate::pac::rcc::vals::Plln::MUL263 => self * 1u32 / 263u32,
            crate::pac::rcc::vals::Plln::MUL264 => self * 1u32 / 264u32,
            crate::pac::rcc::vals::Plln::MUL265 => self * 1u32 / 265u32,
            crate::pac::rcc::vals::Plln::MUL266 => self * 1u32 / 266u32,
            crate::pac::rcc::vals::Plln::MUL267 => self * 1u32 / 267u32,
            crate::pac::rcc::vals::Plln::MUL268 => self * 1u32 / 268u32,
            crate::pac::rcc::vals::Plln::MUL269 => self * 1u32 / 269u32,
            crate::pac::rcc::vals::Plln::MUL270 => self * 1u32 / 270u32,
            crate::pac::rcc::vals::Plln::MUL271 => self * 1u32 / 271u32,
            crate::pac::rcc::vals::Plln::MUL272 => self * 1u32 / 272u32,
            crate::pac::rcc::vals::Plln::MUL273 => self * 1u32 / 273u32,
            crate::pac::rcc::vals::Plln::MUL274 => self * 1u32 / 274u32,
            crate::pac::rcc::vals::Plln::MUL275 => self * 1u32 / 275u32,
            crate::pac::rcc::vals::Plln::MUL276 => self * 1u32 / 276u32,
            crate::pac::rcc::vals::Plln::MUL277 => self * 1u32 / 277u32,
            crate::pac::rcc::vals::Plln::MUL278 => self * 1u32 / 278u32,
            crate::pac::rcc::vals::Plln::MUL279 => self * 1u32 / 279u32,
            crate::pac::rcc::vals::Plln::MUL280 => self * 1u32 / 280u32,
            crate::pac::rcc::vals::Plln::MUL281 => self * 1u32 / 281u32,
            crate::pac::rcc::vals::Plln::MUL282 => self * 1u32 / 282u32,
            crate::pac::rcc::vals::Plln::MUL283 => self * 1u32 / 283u32,
            crate::pac::rcc::vals::Plln::MUL284 => self * 1u32 / 284u32,
            crate::pac::rcc::vals::Plln::MUL285 => self * 1u32 / 285u32,
            crate::pac::rcc::vals::Plln::MUL286 => self * 1u32 / 286u32,
            crate::pac::rcc::vals::Plln::MUL287 => self * 1u32 / 287u32,
            crate::pac::rcc::vals::Plln::MUL288 => self * 1u32 / 288u32,
            crate::pac::rcc::vals::Plln::MUL289 => self * 1u32 / 289u32,
            crate::pac::rcc::vals::Plln::MUL290 => self * 1u32 / 290u32,
            crate::pac::rcc::vals::Plln::MUL291 => self * 1u32 / 291u32,
            crate::pac::rcc::vals::Plln::MUL292 => self * 1u32 / 292u32,
            crate::pac::rcc::vals::Plln::MUL293 => self * 1u32 / 293u32,
            crate::pac::rcc::vals::Plln::MUL294 => self * 1u32 / 294u32,
            crate::pac::rcc::vals::Plln::MUL295 => self * 1u32 / 295u32,
            crate::pac::rcc::vals::Plln::MUL296 => self * 1u32 / 296u32,
            crate::pac::rcc::vals::Plln::MUL297 => self * 1u32 / 297u32,
            crate::pac::rcc::vals::Plln::MUL298 => self * 1u32 / 298u32,
            crate::pac::rcc::vals::Plln::MUL299 => self * 1u32 / 299u32,
            crate::pac::rcc::vals::Plln::MUL300 => self * 1u32 / 300u32,
            crate::pac::rcc::vals::Plln::MUL301 => self * 1u32 / 301u32,
            crate::pac::rcc::vals::Plln::MUL302 => self * 1u32 / 302u32,
            crate::pac::rcc::vals::Plln::MUL303 => self * 1u32 / 303u32,
            crate::pac::rcc::vals::Plln::MUL304 => self * 1u32 / 304u32,
            crate::pac::rcc::vals::Plln::MUL305 => self * 1u32 / 305u32,
            crate::pac::rcc::vals::Plln::MUL306 => self * 1u32 / 306u32,
            crate::pac::rcc::vals::Plln::MUL307 => self * 1u32 / 307u32,
            crate::pac::rcc::vals::Plln::MUL308 => self * 1u32 / 308u32,
            crate::pac::rcc::vals::Plln::MUL309 => self * 1u32 / 309u32,
            crate::pac::rcc::vals::Plln::MUL310 => self * 1u32 / 310u32,
            crate::pac::rcc::vals::Plln::MUL311 => self * 1u32 / 311u32,
            crate::pac::rcc::vals::Plln::MUL312 => self * 1u32 / 312u32,
            crate::pac::rcc::vals::Plln::MUL313 => self * 1u32 / 313u32,
            crate::pac::rcc::vals::Plln::MUL314 => self * 1u32 / 314u32,
            crate::pac::rcc::vals::Plln::MUL315 => self * 1u32 / 315u32,
            crate::pac::rcc::vals::Plln::MUL316 => self * 1u32 / 316u32,
            crate::pac::rcc::vals::Plln::MUL317 => self * 1u32 / 317u32,
            crate::pac::rcc::vals::Plln::MUL318 => self * 1u32 / 318u32,
            crate::pac::rcc::vals::Plln::MUL319 => self * 1u32 / 319u32,
            crate::pac::rcc::vals::Plln::MUL320 => self * 1u32 / 320u32,
            crate::pac::rcc::vals::Plln::MUL321 => self * 1u32 / 321u32,
            crate::pac::rcc::vals::Plln::MUL322 => self * 1u32 / 322u32,
            crate::pac::rcc::vals::Plln::MUL323 => self * 1u32 / 323u32,
            crate::pac::rcc::vals::Plln::MUL324 => self * 1u32 / 324u32,
            crate::pac::rcc::vals::Plln::MUL325 => self * 1u32 / 325u32,
            crate::pac::rcc::vals::Plln::MUL326 => self * 1u32 / 326u32,
            crate::pac::rcc::vals::Plln::MUL327 => self * 1u32 / 327u32,
            crate::pac::rcc::vals::Plln::MUL328 => self * 1u32 / 328u32,
            crate::pac::rcc::vals::Plln::MUL329 => self * 1u32 / 329u32,
            crate::pac::rcc::vals::Plln::MUL330 => self * 1u32 / 330u32,
            crate::pac::rcc::vals::Plln::MUL331 => self * 1u32 / 331u32,
            crate::pac::rcc::vals::Plln::MUL332 => self * 1u32 / 332u32,
            crate::pac::rcc::vals::Plln::MUL333 => self * 1u32 / 333u32,
            crate::pac::rcc::vals::Plln::MUL334 => self * 1u32 / 334u32,
            crate::pac::rcc::vals::Plln::MUL335 => self * 1u32 / 335u32,
            crate::pac::rcc::vals::Plln::MUL336 => self * 1u32 / 336u32,
            crate::pac::rcc::vals::Plln::MUL337 => self * 1u32 / 337u32,
            crate::pac::rcc::vals::Plln::MUL338 => self * 1u32 / 338u32,
            crate::pac::rcc::vals::Plln::MUL339 => self * 1u32 / 339u32,
            crate::pac::rcc::vals::Plln::MUL340 => self * 1u32 / 340u32,
            crate::pac::rcc::vals::Plln::MUL341 => self * 1u32 / 341u32,
            crate::pac::rcc::vals::Plln::MUL342 => self * 1u32 / 342u32,
            crate::pac::rcc::vals::Plln::MUL343 => self * 1u32 / 343u32,
            crate::pac::rcc::vals::Plln::MUL344 => self * 1u32 / 344u32,
            crate::pac::rcc::vals::Plln::MUL345 => self * 1u32 / 345u32,
            crate::pac::rcc::vals::Plln::MUL346 => self * 1u32 / 346u32,
            crate::pac::rcc::vals::Plln::MUL347 => self * 1u32 / 347u32,
            crate::pac::rcc::vals::Plln::MUL348 => self * 1u32 / 348u32,
            crate::pac::rcc::vals::Plln::MUL349 => self * 1u32 / 349u32,
            crate::pac::rcc::vals::Plln::MUL350 => self * 1u32 / 350u32,
            crate::pac::rcc::vals::Plln::MUL351 => self * 1u32 / 351u32,
            crate::pac::rcc::vals::Plln::MUL352 => self * 1u32 / 352u32,
            crate::pac::rcc::vals::Plln::MUL353 => self * 1u32 / 353u32,
            crate::pac::rcc::vals::Plln::MUL354 => self * 1u32 / 354u32,
            crate::pac::rcc::vals::Plln::MUL355 => self * 1u32 / 355u32,
            crate::pac::rcc::vals::Plln::MUL356 => self * 1u32 / 356u32,
            crate::pac::rcc::vals::Plln::MUL357 => self * 1u32 / 357u32,
            crate::pac::rcc::vals::Plln::MUL358 => self * 1u32 / 358u32,
            crate::pac::rcc::vals::Plln::MUL359 => self * 1u32 / 359u32,
            crate::pac::rcc::vals::Plln::MUL360 => self * 1u32 / 360u32,
            crate::pac::rcc::vals::Plln::MUL361 => self * 1u32 / 361u32,
            crate::pac::rcc::vals::Plln::MUL362 => self * 1u32 / 362u32,
            crate::pac::rcc::vals::Plln::MUL363 => self * 1u32 / 363u32,
            crate::pac::rcc::vals::Plln::MUL364 => self * 1u32 / 364u32,
            crate::pac::rcc::vals::Plln::MUL365 => self * 1u32 / 365u32,
            crate::pac::rcc::vals::Plln::MUL366 => self * 1u32 / 366u32,
            crate::pac::rcc::vals::Plln::MUL367 => self * 1u32 / 367u32,
            crate::pac::rcc::vals::Plln::MUL368 => self * 1u32 / 368u32,
            crate::pac::rcc::vals::Plln::MUL369 => self * 1u32 / 369u32,
            crate::pac::rcc::vals::Plln::MUL370 => self * 1u32 / 370u32,
            crate::pac::rcc::vals::Plln::MUL371 => self * 1u32 / 371u32,
            crate::pac::rcc::vals::Plln::MUL372 => self * 1u32 / 372u32,
            crate::pac::rcc::vals::Plln::MUL373 => self * 1u32 / 373u32,
            crate::pac::rcc::vals::Plln::MUL374 => self * 1u32 / 374u32,
            crate::pac::rcc::vals::Plln::MUL375 => self * 1u32 / 375u32,
            crate::pac::rcc::vals::Plln::MUL376 => self * 1u32 / 376u32,
            crate::pac::rcc::vals::Plln::MUL377 => self * 1u32 / 377u32,
            crate::pac::rcc::vals::Plln::MUL378 => self * 1u32 / 378u32,
            crate::pac::rcc::vals::Plln::MUL379 => self * 1u32 / 379u32,
            crate::pac::rcc::vals::Plln::MUL380 => self * 1u32 / 380u32,
            crate::pac::rcc::vals::Plln::MUL381 => self * 1u32 / 381u32,
            crate::pac::rcc::vals::Plln::MUL382 => self * 1u32 / 382u32,
            crate::pac::rcc::vals::Plln::MUL383 => self * 1u32 / 383u32,
            crate::pac::rcc::vals::Plln::MUL384 => self * 1u32 / 384u32,
            crate::pac::rcc::vals::Plln::MUL385 => self * 1u32 / 385u32,
            crate::pac::rcc::vals::Plln::MUL386 => self * 1u32 / 386u32,
            crate::pac::rcc::vals::Plln::MUL387 => self * 1u32 / 387u32,
            crate::pac::rcc::vals::Plln::MUL388 => self * 1u32 / 388u32,
            crate::pac::rcc::vals::Plln::MUL389 => self * 1u32 / 389u32,
            crate::pac::rcc::vals::Plln::MUL390 => self * 1u32 / 390u32,
            crate::pac::rcc::vals::Plln::MUL391 => self * 1u32 / 391u32,
            crate::pac::rcc::vals::Plln::MUL392 => self * 1u32 / 392u32,
            crate::pac::rcc::vals::Plln::MUL393 => self * 1u32 / 393u32,
            crate::pac::rcc::vals::Plln::MUL394 => self * 1u32 / 394u32,
            crate::pac::rcc::vals::Plln::MUL395 => self * 1u32 / 395u32,
            crate::pac::rcc::vals::Plln::MUL396 => self * 1u32 / 396u32,
            crate::pac::rcc::vals::Plln::MUL397 => self * 1u32 / 397u32,
            crate::pac::rcc::vals::Plln::MUL398 => self * 1u32 / 398u32,
            crate::pac::rcc::vals::Plln::MUL399 => self * 1u32 / 399u32,
            crate::pac::rcc::vals::Plln::MUL400 => self * 1u32 / 400u32,
            crate::pac::rcc::vals::Plln::MUL401 => self * 1u32 / 401u32,
            crate::pac::rcc::vals::Plln::MUL402 => self * 1u32 / 402u32,
            crate::pac::rcc::vals::Plln::MUL403 => self * 1u32 / 403u32,
            crate::pac::rcc::vals::Plln::MUL404 => self * 1u32 / 404u32,
            crate::pac::rcc::vals::Plln::MUL405 => self * 1u32 / 405u32,
            crate::pac::rcc::vals::Plln::MUL406 => self * 1u32 / 406u32,
            crate::pac::rcc::vals::Plln::MUL407 => self * 1u32 / 407u32,
            crate::pac::rcc::vals::Plln::MUL408 => self * 1u32 / 408u32,
            crate::pac::rcc::vals::Plln::MUL409 => self * 1u32 / 409u32,
            crate::pac::rcc::vals::Plln::MUL410 => self * 1u32 / 410u32,
            crate::pac::rcc::vals::Plln::MUL411 => self * 1u32 / 411u32,
            crate::pac::rcc::vals::Plln::MUL412 => self * 1u32 / 412u32,
            crate::pac::rcc::vals::Plln::MUL413 => self * 1u32 / 413u32,
            crate::pac::rcc::vals::Plln::MUL414 => self * 1u32 / 414u32,
            crate::pac::rcc::vals::Plln::MUL415 => self * 1u32 / 415u32,
            crate::pac::rcc::vals::Plln::MUL416 => self * 1u32 / 416u32,
            crate::pac::rcc::vals::Plln::MUL417 => self * 1u32 / 417u32,
            crate::pac::rcc::vals::Plln::MUL418 => self * 1u32 / 418u32,
            crate::pac::rcc::vals::Plln::MUL419 => self * 1u32 / 419u32,
            crate::pac::rcc::vals::Plln::MUL420 => self * 1u32 / 420u32,
            crate::pac::rcc::vals::Plln::MUL421 => self * 1u32 / 421u32,
            crate::pac::rcc::vals::Plln::MUL422 => self * 1u32 / 422u32,
            crate::pac::rcc::vals::Plln::MUL423 => self * 1u32 / 423u32,
            crate::pac::rcc::vals::Plln::MUL424 => self * 1u32 / 424u32,
            crate::pac::rcc::vals::Plln::MUL425 => self * 1u32 / 425u32,
            crate::pac::rcc::vals::Plln::MUL426 => self * 1u32 / 426u32,
            crate::pac::rcc::vals::Plln::MUL427 => self * 1u32 / 427u32,
            crate::pac::rcc::vals::Plln::MUL428 => self * 1u32 / 428u32,
            crate::pac::rcc::vals::Plln::MUL429 => self * 1u32 / 429u32,
            crate::pac::rcc::vals::Plln::MUL430 => self * 1u32 / 430u32,
            crate::pac::rcc::vals::Plln::MUL431 => self * 1u32 / 431u32,
            crate::pac::rcc::vals::Plln::MUL432 => self * 1u32 / 432u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Plln> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Plln) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Plln::MUL50 => self * 50u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL51 => self * 51u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL52 => self * 52u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL53 => self * 53u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL54 => self * 54u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL55 => self * 55u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL56 => self * 56u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL57 => self * 57u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL58 => self * 58u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL59 => self * 59u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL60 => self * 60u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL61 => self * 61u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL62 => self * 62u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL63 => self * 63u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL65 => self * 65u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL66 => self * 66u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL67 => self * 67u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL68 => self * 68u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL69 => self * 69u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL70 => self * 70u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL71 => self * 71u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL72 => self * 72u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL73 => self * 73u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL74 => self * 74u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL75 => self * 75u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL76 => self * 76u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL77 => self * 77u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL78 => self * 78u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL79 => self * 79u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL80 => self * 80u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL81 => self * 81u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL82 => self * 82u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL83 => self * 83u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL84 => self * 84u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL85 => self * 85u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL86 => self * 86u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL87 => self * 87u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL88 => self * 88u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL89 => self * 89u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL90 => self * 90u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL91 => self * 91u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL92 => self * 92u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL93 => self * 93u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL94 => self * 94u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL95 => self * 95u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL96 => self * 96u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL97 => self * 97u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL98 => self * 98u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL99 => self * 99u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL100 => self * 100u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL101 => self * 101u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL102 => self * 102u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL103 => self * 103u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL104 => self * 104u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL105 => self * 105u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL106 => self * 106u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL107 => self * 107u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL108 => self * 108u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL109 => self * 109u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL110 => self * 110u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL111 => self * 111u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL112 => self * 112u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL113 => self * 113u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL114 => self * 114u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL115 => self * 115u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL116 => self * 116u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL117 => self * 117u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL118 => self * 118u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL119 => self * 119u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL120 => self * 120u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL121 => self * 121u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL122 => self * 122u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL123 => self * 123u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL124 => self * 124u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL125 => self * 125u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL126 => self * 126u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL127 => self * 127u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL128 => self * 128u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL129 => self * 129u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL130 => self * 130u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL131 => self * 131u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL132 => self * 132u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL133 => self * 133u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL134 => self * 134u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL135 => self * 135u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL136 => self * 136u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL137 => self * 137u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL138 => self * 138u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL139 => self * 139u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL140 => self * 140u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL141 => self * 141u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL142 => self * 142u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL143 => self * 143u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL144 => self * 144u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL145 => self * 145u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL146 => self * 146u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL147 => self * 147u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL148 => self * 148u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL149 => self * 149u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL150 => self * 150u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL151 => self * 151u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL152 => self * 152u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL153 => self * 153u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL154 => self * 154u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL155 => self * 155u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL156 => self * 156u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL157 => self * 157u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL158 => self * 158u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL159 => self * 159u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL160 => self * 160u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL161 => self * 161u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL162 => self * 162u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL163 => self * 163u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL164 => self * 164u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL165 => self * 165u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL166 => self * 166u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL167 => self * 167u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL168 => self * 168u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL169 => self * 169u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL170 => self * 170u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL171 => self * 171u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL172 => self * 172u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL173 => self * 173u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL174 => self * 174u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL175 => self * 175u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL176 => self * 176u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL177 => self * 177u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL178 => self * 178u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL179 => self * 179u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL180 => self * 180u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL181 => self * 181u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL182 => self * 182u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL183 => self * 183u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL184 => self * 184u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL185 => self * 185u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL186 => self * 186u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL187 => self * 187u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL188 => self * 188u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL189 => self * 189u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL190 => self * 190u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL191 => self * 191u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL192 => self * 192u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL193 => self * 193u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL194 => self * 194u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL195 => self * 195u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL196 => self * 196u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL197 => self * 197u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL198 => self * 198u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL199 => self * 199u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL200 => self * 200u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL201 => self * 201u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL202 => self * 202u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL203 => self * 203u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL204 => self * 204u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL205 => self * 205u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL206 => self * 206u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL207 => self * 207u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL208 => self * 208u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL209 => self * 209u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL210 => self * 210u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL211 => self * 211u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL212 => self * 212u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL213 => self * 213u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL214 => self * 214u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL215 => self * 215u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL216 => self * 216u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL217 => self * 217u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL218 => self * 218u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL219 => self * 219u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL220 => self * 220u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL221 => self * 221u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL222 => self * 222u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL223 => self * 223u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL224 => self * 224u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL225 => self * 225u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL226 => self * 226u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL227 => self * 227u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL228 => self * 228u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL229 => self * 229u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL230 => self * 230u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL231 => self * 231u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL232 => self * 232u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL233 => self * 233u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL234 => self * 234u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL235 => self * 235u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL236 => self * 236u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL237 => self * 237u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL238 => self * 238u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL239 => self * 239u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL240 => self * 240u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL241 => self * 241u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL242 => self * 242u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL243 => self * 243u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL244 => self * 244u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL245 => self * 245u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL246 => self * 246u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL247 => self * 247u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL248 => self * 248u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL249 => self * 249u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL250 => self * 250u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL251 => self * 251u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL252 => self * 252u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL253 => self * 253u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL254 => self * 254u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL255 => self * 255u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL256 => self * 256u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL257 => self * 257u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL258 => self * 258u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL259 => self * 259u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL260 => self * 260u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL261 => self * 261u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL262 => self * 262u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL263 => self * 263u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL264 => self * 264u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL265 => self * 265u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL266 => self * 266u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL267 => self * 267u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL268 => self * 268u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL269 => self * 269u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL270 => self * 270u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL271 => self * 271u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL272 => self * 272u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL273 => self * 273u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL274 => self * 274u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL275 => self * 275u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL276 => self * 276u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL277 => self * 277u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL278 => self * 278u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL279 => self * 279u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL280 => self * 280u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL281 => self * 281u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL282 => self * 282u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL283 => self * 283u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL284 => self * 284u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL285 => self * 285u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL286 => self * 286u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL287 => self * 287u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL288 => self * 288u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL289 => self * 289u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL290 => self * 290u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL291 => self * 291u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL292 => self * 292u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL293 => self * 293u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL294 => self * 294u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL295 => self * 295u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL296 => self * 296u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL297 => self * 297u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL298 => self * 298u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL299 => self * 299u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL300 => self * 300u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL301 => self * 301u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL302 => self * 302u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL303 => self * 303u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL304 => self * 304u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL305 => self * 305u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL306 => self * 306u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL307 => self * 307u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL308 => self * 308u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL309 => self * 309u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL310 => self * 310u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL311 => self * 311u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL312 => self * 312u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL313 => self * 313u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL314 => self * 314u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL315 => self * 315u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL316 => self * 316u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL317 => self * 317u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL318 => self * 318u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL319 => self * 319u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL320 => self * 320u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL321 => self * 321u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL322 => self * 322u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL323 => self * 323u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL324 => self * 324u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL325 => self * 325u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL326 => self * 326u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL327 => self * 327u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL328 => self * 328u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL329 => self * 329u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL330 => self * 330u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL331 => self * 331u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL332 => self * 332u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL333 => self * 333u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL334 => self * 334u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL335 => self * 335u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL336 => self * 336u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL337 => self * 337u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL338 => self * 338u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL339 => self * 339u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL340 => self * 340u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL341 => self * 341u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL342 => self * 342u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL343 => self * 343u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL344 => self * 344u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL345 => self * 345u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL346 => self * 346u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL347 => self * 347u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL348 => self * 348u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL349 => self * 349u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL350 => self * 350u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL351 => self * 351u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL352 => self * 352u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL353 => self * 353u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL354 => self * 354u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL355 => self * 355u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL356 => self * 356u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL357 => self * 357u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL358 => self * 358u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL359 => self * 359u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL360 => self * 360u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL361 => self * 361u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL362 => self * 362u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL363 => self * 363u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL364 => self * 364u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL365 => self * 365u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL366 => self * 366u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL367 => self * 367u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL368 => self * 368u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL369 => self * 369u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL370 => self * 370u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL371 => self * 371u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL372 => self * 372u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL373 => self * 373u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL374 => self * 374u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL375 => self * 375u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL376 => self * 376u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL377 => self * 377u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL378 => self * 378u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL379 => self * 379u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL380 => self * 380u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL381 => self * 381u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL382 => self * 382u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL383 => self * 383u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL384 => self * 384u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL385 => self * 385u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL386 => self * 386u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL387 => self * 387u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL388 => self * 388u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL389 => self * 389u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL390 => self * 390u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL391 => self * 391u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL392 => self * 392u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL393 => self * 393u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL394 => self * 394u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL395 => self * 395u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL396 => self * 396u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL397 => self * 397u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL398 => self * 398u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL399 => self * 399u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL400 => self * 400u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL401 => self * 401u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL402 => self * 402u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL403 => self * 403u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL404 => self * 404u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL405 => self * 405u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL406 => self * 406u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL407 => self * 407u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL408 => self * 408u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL409 => self * 409u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL410 => self * 410u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL411 => self * 411u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL412 => self * 412u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL413 => self * 413u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL414 => self * 414u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL415 => self * 415u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL416 => self * 416u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL417 => self * 417u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL418 => self * 418u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL419 => self * 419u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL420 => self * 420u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL421 => self * 421u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL422 => self * 422u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL423 => self * 423u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL424 => self * 424u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL425 => self * 425u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL426 => self * 426u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL427 => self * 427u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL428 => self * 428u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL429 => self * 429u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL430 => self * 430u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL431 => self * 431u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL432 => self * 432u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllp> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllp) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllp::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllp::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllp::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllp::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllp> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllp) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllp::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllq> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllq) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllq::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllq::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllq::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllq::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllq::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllq::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllq::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Pllq::DIV9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Pllq::DIV10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Pllq::DIV11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Pllq::DIV12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Pllq::DIV13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Pllq::DIV14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Pllq::DIV15 => self * 1u32 / 15u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllq> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllq) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllq::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV15 => self * 15u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllr> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllr) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllr::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllr::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllr::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllr::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllr::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllr::DIV7 => self * 1u32 / 7u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllr> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllr) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllr::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV7 => self * 7u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod ADC123_COMMON {}
    pub mod ADC2 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod ADC3 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod CAN1 {
        pub type RX0 = crate::interrupt::typelevel::CAN1_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN1_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN1_SCE;
        pub type TX = crate::interrupt::typelevel::CAN1_TX;
    }
    pub mod CAN2 {
        pub type RX0 = crate::interrupt::typelevel::CAN2_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN2_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN2_SCE;
        pub type TX = crate::interrupt::typelevel::CAN2_TX;
    }
    pub mod CAN3 {
        pub type RX0 = crate::interrupt::typelevel::CAN3_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN3_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN3_SCE;
        pub type TX = crate::interrupt::typelevel::CAN3_TX;
    }
    pub mod CEC {
        pub type GLOBAL = crate::interrupt::typelevel::CEC;
    }
    pub mod CRC {}
    pub mod CRYP {
        pub type GLOBAL = crate::interrupt::typelevel::CRYP;
    }
    pub mod DAC1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod DBGMCU {}
    pub mod DCMI {
        pub type GLOBAL = crate::interrupt::typelevel::DCMI;
    }
    pub mod DFSDM1 {}
    pub mod DMA1 {
        pub type CH0 = crate::interrupt::typelevel::DMA1_STREAM0;
        pub type CH1 = crate::interrupt::typelevel::DMA1_STREAM1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_STREAM2;
        pub type CH3 = crate::interrupt::typelevel::DMA1_STREAM3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_STREAM4;
        pub type CH5 = crate::interrupt::typelevel::DMA1_STREAM5;
        pub type CH6 = crate::interrupt::typelevel::DMA1_STREAM6;
        pub type CH7 = crate::interrupt::typelevel::DMA1_STREAM7;
    }
    pub mod DMA2 {
        pub type CH0 = crate::interrupt::typelevel::DMA2_STREAM0;
        pub type CH1 = crate::interrupt::typelevel::DMA2_STREAM1;
        pub type CH2 = crate::interrupt::typelevel::DMA2_STREAM2;
        pub type CH3 = crate::interrupt::typelevel::DMA2_STREAM3;
        pub type CH4 = crate::interrupt::typelevel::DMA2_STREAM4;
        pub type CH5 = crate::interrupt::typelevel::DMA2_STREAM5;
        pub type CH6 = crate::interrupt::typelevel::DMA2_STREAM6;
        pub type CH7 = crate::interrupt::typelevel::DMA2_STREAM7;
    }
    pub mod DMA2D {
        pub type GLOBAL = crate::interrupt::typelevel::DMA2D;
    }
    pub mod ETH {
        pub type GLOBAL = crate::interrupt::typelevel::ETH;
        pub type WKUP = crate::interrupt::typelevel::ETH_WKUP;
    }
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI9_5;
    }
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod FMC {
        pub type GLOBAL = crate::interrupt::typelevel::FMC;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOE {}
    pub mod GPIOF {}
    pub mod GPIOG {}
    pub mod GPIOH {}
    pub mod GPIOI {}
    pub mod GPIOJ {}
    pub mod GPIOK {}
    pub mod HASH {
        pub type GLOBAL = crate::interrupt::typelevel::HASH_RNG;
    }
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1_ER;
        pub type EV = crate::interrupt::typelevel::I2C1_EV;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2_ER;
        pub type EV = crate::interrupt::typelevel::I2C2_EV;
    }
    pub mod I2C3 {
        pub type ER = crate::interrupt::typelevel::I2C3_ER;
        pub type EV = crate::interrupt::typelevel::I2C3_EV;
    }
    pub mod I2C4 {
        pub type ER = crate::interrupt::typelevel::I2C4_ER;
        pub type EV = crate::interrupt::typelevel::I2C4_EV;
    }
    pub mod IWDG {}
    pub mod JPEG {
        pub type GLOBAL = crate::interrupt::typelevel::JPEG;
    }
    pub mod LPTIM1 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM1;
    }
    pub mod LTDC {
        pub type ER = crate::interrupt::typelevel::LTDC_ER;
        pub type GLOBAL = crate::interrupt::typelevel::LTDC;
    }
    pub mod MDIOS {
        pub type GLOBAL = crate::interrupt::typelevel::MDIOS;
        pub type WKUP = crate::interrupt::typelevel::MDIOS;
    }
    pub mod PWR {}
    pub mod QUADSPI {
        pub type GLOBAL = crate::interrupt::typelevel::QUADSPI;
    }
    pub mod RCC {
        pub type GLOBAL = crate::interrupt::typelevel::RCC;
    }
    pub mod RNG {
        pub type GLOBAL = crate::interrupt::typelevel::HASH_RNG;
    }
    pub mod RTC {
        pub type ALARM = crate::interrupt::typelevel::RTC_ALARM;
        pub type STAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type TAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type WKUP = crate::interrupt::typelevel::RTC_WKUP;
    }
    pub mod SAI1 {
        pub type A = crate::interrupt::typelevel::SAI1;
        pub type B = crate::interrupt::typelevel::SAI1;
    }
    pub mod SAI2 {
        pub type A = crate::interrupt::typelevel::SAI2;
        pub type B = crate::interrupt::typelevel::SAI2;
    }
    pub mod SDMMC1 {
        pub type GLOBAL = crate::interrupt::typelevel::SDMMC1;
    }
    pub mod SDMMC2 {
        pub type GLOBAL = crate::interrupt::typelevel::SDMMC2;
    }
    pub mod SPDIFRX1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPDIF_RX;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod SPI3 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI3;
    }
    pub mod SPI4 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI4;
    }
    pub mod SPI5 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI5;
    }
    pub mod SPI6 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI6;
    }
    pub mod SYSCFG {}
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM10;
    }
    pub mod TIM10 {
        pub type BRK = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type CC = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type COM = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type TRG = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM10;
    }
    pub mod TIM11 {
        pub type BRK = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type CC = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type UP = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
    }
    pub mod TIM12 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type CC = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type COM = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type TRG = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type UP = crate::interrupt::typelevel::TIM8_BRK_TIM12;
    }
    pub mod TIM13 {
        pub type BRK = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type CC = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type COM = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type TRG = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type UP = crate::interrupt::typelevel::TIM8_UP_TIM13;
    }
    pub mod TIM14 {
        pub type BRK = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type CC = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type UP = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3;
        pub type CC = crate::interrupt::typelevel::TIM3;
        pub type COM = crate::interrupt::typelevel::TIM3;
        pub type TRG = crate::interrupt::typelevel::TIM3;
        pub type UP = crate::interrupt::typelevel::TIM3;
    }
    pub mod TIM4 {
        pub type BRK = crate::interrupt::typelevel::TIM4;
        pub type CC = crate::interrupt::typelevel::TIM4;
        pub type COM = crate::interrupt::typelevel::TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM4;
        pub type UP = crate::interrupt::typelevel::TIM4;
    }
    pub mod TIM5 {
        pub type BRK = crate::interrupt::typelevel::TIM5;
        pub type CC = crate::interrupt::typelevel::TIM5;
        pub type COM = crate::interrupt::typelevel::TIM5;
        pub type TRG = crate::interrupt::typelevel::TIM5;
        pub type UP = crate::interrupt::typelevel::TIM5;
    }
    pub mod TIM6 {
        pub type BRK = crate::interrupt::typelevel::TIM6_DAC;
        pub type CC = crate::interrupt::typelevel::TIM6_DAC;
        pub type COM = crate::interrupt::typelevel::TIM6_DAC;
        pub type TRG = crate::interrupt::typelevel::TIM6_DAC;
        pub type UP = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod TIM7 {
        pub type BRK = crate::interrupt::typelevel::TIM7;
        pub type CC = crate::interrupt::typelevel::TIM7;
        pub type COM = crate::interrupt::typelevel::TIM7;
        pub type TRG = crate::interrupt::typelevel::TIM7;
        pub type UP = crate::interrupt::typelevel::TIM7;
    }
    pub mod TIM8 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type CC = crate::interrupt::typelevel::TIM8_CC;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type UP = crate::interrupt::typelevel::TIM8_UP_TIM13;
    }
    pub mod TIM9 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type CC = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type COM = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type TRG = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type UP = crate::interrupt::typelevel::TIM1_BRK_TIM9;
    }
    pub mod UART4 {
        pub type GLOBAL = crate::interrupt::typelevel::UART4;
    }
    pub mod UART5 {
        pub type GLOBAL = crate::interrupt::typelevel::UART5;
    }
    pub mod UART7 {
        pub type GLOBAL = crate::interrupt::typelevel::UART7;
    }
    pub mod UART8 {
        pub type GLOBAL = crate::interrupt::typelevel::UART8;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2;
    }
    pub mod USART3 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3;
    }
    pub mod USART6 {
        pub type GLOBAL = crate::interrupt::typelevel::USART6;
    }
    pub mod USB_OTG_FS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_FS;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_FS;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_FS;
        pub type WKUP = crate::interrupt::typelevel::OTG_FS_WKUP;
    }
    pub mod USB_OTG_HS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_HS_EP1_IN;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_HS_EP1_OUT;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_HS;
        pub type WKUP = crate::interrupt::typelevel::OTG_HS_WKUP;
    }
    pub mod VREFINTCAL {}
    pub mod WWDG {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH0, 0u8);
dma_channel_impl!(DMA1_CH1, 1u8);
dma_channel_impl!(DMA1_CH2, 2u8);
dma_channel_impl!(DMA1_CH3, 3u8);
dma_channel_impl!(DMA1_CH4, 4u8);
dma_channel_impl!(DMA1_CH5, 5u8);
dma_channel_impl!(DMA1_CH6, 6u8);
dma_channel_impl!(DMA1_CH7, 7u8);
dma_channel_impl!(DMA2_CH0, 8u8);
dma_channel_impl!(DMA2_CH1, 9u8);
dma_channel_impl!(DMA2_CH2, 10u8);
dma_channel_impl!(DMA2_CH3, 11u8);
dma_channel_impl!(DMA2_CH4, 12u8);
dma_channel_impl!(DMA2_CH5, 13u8);
dma_channel_impl!(DMA2_CH6, 14u8);
dma_channel_impl!(DMA2_CH7, 15u8);
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM0() {
    <crate::peripherals::DMA1_CH0 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM1() {
    <crate::peripherals::DMA1_CH1 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM2() {
    <crate::peripherals::DMA1_CH2 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM3() {
    <crate::peripherals::DMA1_CH3 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM4() {
    <crate::peripherals::DMA1_CH4 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM5() {
    <crate::peripherals::DMA1_CH5 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM6() {
    <crate::peripherals::DMA1_CH6 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_STREAM7() {
    <crate::peripherals::DMA1_CH7 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM0() {
    <crate::peripherals::DMA2_CH0 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM1() {
    <crate::peripherals::DMA2_CH1 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM2() {
    <crate::peripherals::DMA2_CH2 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM3() {
    <crate::peripherals::DMA2_CH3 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM4() {
    <crate::peripherals::DMA2_CH4 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM5() {
    <crate::peripherals::DMA2_CH5 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM6() {
    <crate::peripherals::DMA2_CH6 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA2_STREAM7() {
    <crate::peripherals::DMA2_CH7 as crate::dma::ChannelInterrupt>::on_irq();
}
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 0usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 1usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 2usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 3usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 4usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 5usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 6usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 7usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 0usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 1usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 2usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 3usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 4usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 5usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 6usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 7usize,
    },
];
pub fn gpio_block(n: usize) -> crate::pac::gpio::Gpio {
    {
        unsafe {
            {
                crate::pac::gpio::Gpio::from_ptr((1073872896usize + 1024usize * n) as _)
            }
        }
    }
}
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 2097152usize;
pub const WRITE_SIZE: usize = 16usize;
