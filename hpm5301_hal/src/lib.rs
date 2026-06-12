#![no_std]

use core::marker::PhantomData;

pub use hpm5301_pac as pac;

pub mod embassy;
pub mod gpio;
pub mod mode;
pub mod sysctl;
pub mod time;
pub mod timer;
pub mod usb;

/// Owned peripheral token, following Embassy HAL's `Peri<'d, T>` style.
pub struct Peri<'d, T: PeripheralType> {
    inner: T,
    _lifetime: PhantomData<&'d mut T>,
}

impl<'d, T: PeripheralType> Peri<'d, T> {
    pub(crate) const fn new(inner: T) -> Self {
        Self {
            inner,
            _lifetime: PhantomData,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

/// Marker implemented by all HAL-owned peripheral tokens.
pub trait PeripheralType {}

/// Raw PAC access for peripheral tokens.
///
/// HAL drivers use this internally. Users can also use it as an escape hatch
/// while a high-level driver is still missing.
pub trait RawPeripheral: PeripheralType {
    type Raw;

    /// Steal the matching PAC peripheral.
    ///
    /// # Safety
    ///
    /// The caller must ensure register access is coordinated with any HAL
    /// driver that owns the same peripheral token.
    unsafe fn steal_raw() -> Self::Raw;
}

impl<'d, T: RawPeripheral> Peri<'d, T> {
    /// Return the raw PAC register accessor for this token.
    ///
    /// # Safety
    ///
    /// This bypasses the HAL's type-state API. Do not use the raw peripheral
    /// concurrently with a HAL driver for the same token.
    pub unsafe fn raw(&self) -> T::Raw {
        unsafe { T::steal_raw() }
    }
}

macro_rules! define_peripherals {
    ($($name:ident),+ $(,)?) => {
        pub mod peripherals {
            $(
                #[allow(non_camel_case_types)]
                pub struct $name;

                impl crate::PeripheralType for $name {}
            )+
        }

        #[allow(non_snake_case)]
        pub struct Peripherals {
            $(pub $name: Peri<'static, peripherals::$name>,)+
        }

        impl Peripherals {
            /// Returns all HAL peripheral tokens without touching hardware.
            ///
            /// # Safety
            ///
            /// The caller must guarantee this is called at most once.
            pub unsafe fn steal() -> Self {
                Self {
                    $($name: Peri::new(peripherals::$name),)+
                }
            }
        }
    };
}

define_peripherals!(
    FGPIO, GPIO0, PGPIO, PLIC, MCHTMR, PLICSW, GPTMR0, GPTMR1, PTMR, UART0, UART1, UART2, UART3,
    PUART, I2C0, I2C1, I2C2, I2C3, SPI0, SPI1, SPI2, SPI3, CRC, TSNS, MBX0A, MBX0B, EWDG0, EWDG1,
    PEWDG, DMAMUX, HDMA, GPIOM, SYNT, TRGM0, USB0, SEC, MON, OTP, KEYM, ADC0, ACMP, SYSCTL, IOC,
    PIOC, PLLCTLV2, PPOR, PCFG, PGPR0, PGPR1, PDGO, HDMA_CH0, HDMA_CH1, HDMA_CH2, HDMA_CH3,
    HDMA_CH4, HDMA_CH5, HDMA_CH6, HDMA_CH7, HDMA_CH8, HDMA_CH9, HDMA_CH10, HDMA_CH11, HDMA_CH12,
    HDMA_CH13, HDMA_CH14, HDMA_CH15, HDMA_CH16, HDMA_CH17, HDMA_CH18, HDMA_CH19, HDMA_CH20,
    HDMA_CH21, HDMA_CH22, HDMA_CH23, HDMA_CH24, HDMA_CH25, HDMA_CH26, HDMA_CH27, HDMA_CH28,
    HDMA_CH29, HDMA_CH30, HDMA_CH31, PA00, PA01, PA02, PA03, PA04, PA05, PA06, PA07, PA08, PA09,
    PA10, PA11, PA12, PA13, PA14, PA15, PA16, PA17, PA18, PA19, PA20, PA21, PA22, PA23, PA24, PA25,
    PA26, PA27, PA28, PA29, PA30, PA31, PB00, PB01, PB02, PB03, PB04, PB05, PB06, PB07, PB08, PB09,
    PB10, PB11, PB12, PB13, PB14, PB15, PB16, PB17, PB18, PB19, PB20, PB21, PB22, PB23, PB24, PB25,
    PB26, PB27, PB28, PB29, PB30, PB31, PX00, PX01, PX02, PX03, PX04, PX05, PX06, PX07, PY00, PY01,
    PY02, PY03, PY04, PY05, PY06, PY07,
);

macro_rules! impl_raw_peripheral {
    ($token:ident, $raw:ident) => {
        impl RawPeripheral for peripherals::$token {
            type Raw = pac::$raw;

            unsafe fn steal_raw() -> Self::Raw {
                unsafe { pac::$raw::steal() }
            }
        }
    };
}

impl_raw_peripheral!(FGPIO, Fgpio);
impl_raw_peripheral!(GPIO0, Gpio0);
impl_raw_peripheral!(PGPIO, Pgpio);
impl_raw_peripheral!(PLIC, Plic);
impl_raw_peripheral!(MCHTMR, Mchtmr);
impl_raw_peripheral!(PLICSW, Plicsw);
impl_raw_peripheral!(GPTMR0, Gptmr0);
impl_raw_peripheral!(GPTMR1, Gptmr1);
impl_raw_peripheral!(PTMR, Ptmr);
impl_raw_peripheral!(UART0, Uart0);
impl_raw_peripheral!(UART1, Uart1);
impl_raw_peripheral!(UART2, Uart2);
impl_raw_peripheral!(UART3, Uart3);
impl_raw_peripheral!(PUART, Puart);
impl_raw_peripheral!(I2C0, I2c0);
impl_raw_peripheral!(I2C1, I2c1);
impl_raw_peripheral!(I2C2, I2c2);
impl_raw_peripheral!(I2C3, I2c3);
impl_raw_peripheral!(SPI0, Spi0);
impl_raw_peripheral!(SPI1, Spi1);
impl_raw_peripheral!(SPI2, Spi2);
impl_raw_peripheral!(SPI3, Spi3);
impl_raw_peripheral!(CRC, Crc);
impl_raw_peripheral!(TSNS, Tsns);
impl_raw_peripheral!(MBX0A, Mbx0a);
impl_raw_peripheral!(MBX0B, Mbx0b);
impl_raw_peripheral!(EWDG0, Ewdg0);
impl_raw_peripheral!(EWDG1, Ewdg1);
impl_raw_peripheral!(PEWDG, Pewdg);
impl_raw_peripheral!(DMAMUX, Dmamux);
impl_raw_peripheral!(HDMA, Hdma);
impl_raw_peripheral!(GPIOM, Gpiom);
impl_raw_peripheral!(SYNT, Synt);
impl_raw_peripheral!(TRGM0, Trgm0);
impl_raw_peripheral!(USB0, Usb0);
impl_raw_peripheral!(SEC, Sec);
impl_raw_peripheral!(MON, Mon);
impl_raw_peripheral!(OTP, Otp);
impl_raw_peripheral!(KEYM, Keym);
impl_raw_peripheral!(ADC0, Adc0);
impl_raw_peripheral!(ACMP, Acmp);
impl_raw_peripheral!(SYSCTL, Sysctl);
impl_raw_peripheral!(IOC, Ioc);
impl_raw_peripheral!(PIOC, Pioc);
impl_raw_peripheral!(PLLCTLV2, Pllctlv2);
impl_raw_peripheral!(PPOR, Ppor);
impl_raw_peripheral!(PCFG, Pcfg);
impl_raw_peripheral!(PGPR0, Pgpr0);
impl_raw_peripheral!(PGPR1, Pgpr1);
impl_raw_peripheral!(PDGO, Pdgo);

#[derive(Default)]
pub struct Config {
    pub sysctl: sysctl::Config,
}

pub fn init(config: Config) -> Peripherals {
    let _pac = pac::Peripherals::take().expect("hpm5301_hal init called more than once");
    sysctl::init(config.sysctl);
    enable_instruction_cache();
    time::init();
    embassy::init();

    unsafe { Peripherals::steal() }
}

fn enable_instruction_cache() {
    // D-cache stays off until all USB/DMA buffers have explicit coherency handling.
    unsafe {
        andes_riscv::l1c::ic_enable();
    }
}

/// Return peripheral tokens without initializing clocks or the HAL timebase.
///
/// # Safety
///
/// Call at most once, and only when the caller initializes required hardware.
pub unsafe fn uninited() -> Peripherals {
    unsafe { Peripherals::steal() }
}
