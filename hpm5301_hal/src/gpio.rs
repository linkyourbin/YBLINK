use core::{convert::Infallible, marker::PhantomData};

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

use crate::{Peri, PeripheralType, pac, peripherals};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

impl Level {
    pub const fn is_high(self) -> bool {
        matches!(self, Self::High)
    }

    pub const fn is_low(self) -> bool {
        matches!(self, Self::Low)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Speed {
    #[default]
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Speed {
    const fn spd_bits(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
            Self::VeryHigh => 3,
        }
    }

    const fn fast_slew(self) -> bool {
        matches!(self, Self::High | Self::VeryHigh)
    }

    const fn drive_strength(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Medium => 2,
            Self::High => 4,
            Self::VeryHigh => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pull {
    #[default]
    None,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    A,
    B,
    X,
    Y,
}

impl Port {
    const fn gpio_index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::X => 13,
            Self::Y => 14,
        }
    }
}

pub trait Pin: PeripheralType + Sized + 'static {
    const PORT: Port;
    const NUMBER: u8;
    const IOC_INDEX: usize;

    fn degrade() -> AnyPin {
        AnyPin {
            port: Self::PORT,
            number: Self::NUMBER,
            ioc_index: Self::IOC_INDEX,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnyPin {
    port: Port,
    number: u8,
    ioc_index: usize,
}

impl PeripheralType for AnyPin {}

impl AnyPin {
    pub const fn port(&self) -> Port {
        self.port
    }

    pub const fn number(&self) -> u8 {
        self.number
    }
}

pub trait IntoAnyPin<'d> {
    fn into_any_pin(self) -> AnyPin;
}

impl<'d, T: Pin> IntoAnyPin<'d> for Peri<'d, T> {
    fn into_any_pin(self) -> AnyPin {
        let _ = self.into_inner();
        T::degrade()
    }
}

impl<'d> IntoAnyPin<'d> for Peri<'d, AnyPin> {
    fn into_any_pin(self) -> AnyPin {
        self.into_inner()
    }
}

impl<'d, T: Pin> From<Peri<'d, T>> for Peri<'d, AnyPin> {
    fn from(pin: Peri<'d, T>) -> Self {
        Peri::new(pin.into_any_pin())
    }
}

pub struct Output<'d> {
    pin: AnyPin,
    _lifetime: PhantomData<&'d mut AnyPin>,
}

impl<'d> Output<'d> {
    pub fn new(pin: impl IntoAnyPin<'d>, initial_output: Level, speed: Speed) -> Self {
        let pin = pin.into_any_pin();
        configure_as_gpio(pin, speed, Pull::None, false);

        let mut output = Self {
            pin,
            _lifetime: PhantomData,
        };
        output.set_level(initial_output);
        set_output_enable(pin, true);
        output
    }

    pub fn set_high(&mut self) {
        write_pin(self.pin, true);
    }

    pub fn set_low(&mut self) {
        write_pin(self.pin, false);
    }

    pub fn set_level(&mut self, level: Level) {
        write_pin(self.pin, level.is_high());
    }

    pub fn toggle(&mut self) {
        toggle_pin(self.pin);
    }

    pub fn is_set_high(&self) -> bool {
        output_level(self.pin)
    }

    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }
}

pub struct Input<'d> {
    pin: AnyPin,
    _lifetime: PhantomData<&'d mut AnyPin>,
}

impl<'d> Input<'d> {
    pub fn new(pin: impl IntoAnyPin<'d>, pull: Pull) -> Self {
        let pin = pin.into_any_pin();
        configure_as_gpio(pin, Speed::Low, pull, false);
        set_output_enable(pin, false);

        Self {
            pin,
            _lifetime: PhantomData,
        }
    }

    pub fn is_high(&self) -> bool {
        input_level(self.pin)
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    pub fn get_level(&self) -> Level {
        if self.is_high() {
            Level::High
        } else {
            Level::Low
        }
    }
}

pub struct Flex<'d> {
    pin: AnyPin,
    _lifetime: PhantomData<&'d mut AnyPin>,
}

impl<'d> Flex<'d> {
    pub fn new(pin: impl IntoAnyPin<'d>) -> Self {
        let pin = pin.into_any_pin();
        configure_as_gpio(pin, Speed::Low, Pull::None, false);

        Self {
            pin,
            _lifetime: PhantomData,
        }
    }

    pub fn set_as_output(&mut self, speed: Speed) {
        configure_as_gpio(self.pin, speed, Pull::None, false);
        set_output_enable(self.pin, true);
    }

    pub fn set_as_input(&mut self, pull: Pull) {
        configure_as_gpio(self.pin, Speed::Low, pull, false);
        set_output_enable(self.pin, false);
    }

    pub fn set_high(&mut self) {
        write_pin(self.pin, true);
    }

    pub fn set_low(&mut self) {
        write_pin(self.pin, false);
    }

    pub fn set_level(&mut self, level: Level) {
        write_pin(self.pin, level.is_high());
    }

    pub fn toggle(&mut self) {
        toggle_pin(self.pin);
    }

    pub fn is_high(&self) -> bool {
        input_level(self.pin)
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    pub fn is_set_high(&self) -> bool {
        output_level(self.pin)
    }
}

impl ErrorType for Output<'_> {
    type Error = Infallible;
}

impl OutputPin for Output<'_> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Output::set_low(self);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Output::set_high(self);
        Ok(())
    }
}

impl StatefulOutputPin for Output<'_> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_set_low(self))
    }
}

impl ErrorType for Input<'_> {
    type Error = Infallible;
}

impl InputPin for Input<'_> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_low(self))
    }
}

impl ErrorType for Flex<'_> {
    type Error = Infallible;
}

impl OutputPin for Flex<'_> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Flex::set_low(self);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Flex::set_high(self);
        Ok(())
    }
}

impl StatefulOutputPin for Flex<'_> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!Flex::is_set_high(self))
    }
}

impl InputPin for Flex<'_> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_low(self))
    }
}

fn configure_as_gpio(pin: AnyPin, speed: Speed, pull: Pull, open_drain: bool) {
    let ioc = unsafe { &*pac::Ioc::ptr() };
    let pad = ioc.pad(pin.ioc_index);

    pad.func_ctl().write(|w| unsafe {
        w.alt_select()
            .bits(0)
            .analog()
            .clear_bit()
            .loop_back()
            .clear_bit()
    });

    pad.pad_ctl().modify(|_, w| unsafe {
        w.ds()
            .bits(speed.drive_strength())
            .spd()
            .bits(speed.spd_bits())
            .sr()
            .bit(speed.fast_slew())
            .od()
            .bit(open_drain)
            .ke()
            .clear_bit()
            .pe()
            .bit(!matches!(pull, Pull::None))
            .ps()
            .bit(matches!(pull, Pull::Up))
            .prs()
            .bits(0)
            .hys()
            .bit(!matches!(pull, Pull::None))
    });
}

fn gpio() -> &'static pac::fgpio::RegisterBlock {
    unsafe { &*pac::Gpio0::ptr() }
}

fn bit(pin: AnyPin) -> u32 {
    1u32 << pin.number
}

fn set_output_enable(pin: AnyPin, enable: bool) {
    let oe = gpio().oe(pin.port.gpio_index());
    unsafe {
        if enable {
            oe.set().write_with_zero(|w| w.bits(bit(pin)));
        } else {
            oe.clear().write_with_zero(|w| w.bits(bit(pin)));
        }
    }
}

fn write_pin(pin: AnyPin, high: bool) {
    let output = gpio().do_(pin.port.gpio_index());
    unsafe {
        if high {
            output.set().write_with_zero(|w| w.bits(bit(pin)));
        } else {
            output.clear().write_with_zero(|w| w.bits(bit(pin)));
        }
    }
}

fn toggle_pin(pin: AnyPin) {
    unsafe {
        gpio()
            .do_(pin.port.gpio_index())
            .toggle()
            .write_with_zero(|w| w.bits(bit(pin)));
    }
}

fn output_level(pin: AnyPin) -> bool {
    gpio().do_(pin.port.gpio_index()).value().read().bits() & bit(pin) != 0
}

fn input_level(pin: AnyPin) -> bool {
    gpio().di(pin.port.gpio_index()).value().read().bits() & bit(pin) != 0
}

macro_rules! impl_pin {
    ($name:ident, $port:ident, $number:expr, $ioc_index:expr) => {
        impl Pin for peripherals::$name {
            const PORT: Port = Port::$port;
            const NUMBER: u8 = $number;
            const IOC_INDEX: usize = $ioc_index;
        }
    };
}

impl_pin!(PA00, A, 0, 0);
impl_pin!(PA01, A, 1, 1);
impl_pin!(PA02, A, 2, 2);
impl_pin!(PA03, A, 3, 3);
impl_pin!(PA04, A, 4, 4);
impl_pin!(PA05, A, 5, 5);
impl_pin!(PA06, A, 6, 6);
impl_pin!(PA07, A, 7, 7);
impl_pin!(PA08, A, 8, 8);
impl_pin!(PA09, A, 9, 9);
impl_pin!(PA10, A, 10, 10);
impl_pin!(PA11, A, 11, 11);
impl_pin!(PA12, A, 12, 12);
impl_pin!(PA13, A, 13, 13);
impl_pin!(PA14, A, 14, 14);
impl_pin!(PA15, A, 15, 15);
impl_pin!(PA16, A, 16, 16);
impl_pin!(PA17, A, 17, 17);
impl_pin!(PA18, A, 18, 18);
impl_pin!(PA19, A, 19, 19);
impl_pin!(PA20, A, 20, 20);
impl_pin!(PA21, A, 21, 21);
impl_pin!(PA22, A, 22, 22);
impl_pin!(PA23, A, 23, 23);
impl_pin!(PA24, A, 24, 24);
impl_pin!(PA25, A, 25, 25);
impl_pin!(PA26, A, 26, 26);
impl_pin!(PA27, A, 27, 27);
impl_pin!(PA28, A, 28, 28);
impl_pin!(PA29, A, 29, 29);
impl_pin!(PA30, A, 30, 30);
impl_pin!(PA31, A, 31, 31);

impl_pin!(PB00, B, 0, 32);
impl_pin!(PB01, B, 1, 33);
impl_pin!(PB02, B, 2, 34);
impl_pin!(PB03, B, 3, 35);
impl_pin!(PB04, B, 4, 36);
impl_pin!(PB05, B, 5, 37);
impl_pin!(PB06, B, 6, 38);
impl_pin!(PB07, B, 7, 39);
impl_pin!(PB08, B, 8, 40);
impl_pin!(PB09, B, 9, 41);
impl_pin!(PB10, B, 10, 42);
impl_pin!(PB11, B, 11, 43);
impl_pin!(PB12, B, 12, 44);
impl_pin!(PB13, B, 13, 45);
impl_pin!(PB14, B, 14, 46);
impl_pin!(PB15, B, 15, 47);
impl_pin!(PB16, B, 16, 48);
impl_pin!(PB17, B, 17, 49);
impl_pin!(PB18, B, 18, 50);
impl_pin!(PB19, B, 19, 51);
impl_pin!(PB20, B, 20, 52);
impl_pin!(PB21, B, 21, 53);
impl_pin!(PB22, B, 22, 54);
impl_pin!(PB23, B, 23, 55);
impl_pin!(PB24, B, 24, 56);
impl_pin!(PB25, B, 25, 57);
impl_pin!(PB26, B, 26, 58);
impl_pin!(PB27, B, 27, 59);
impl_pin!(PB28, B, 28, 60);
impl_pin!(PB29, B, 29, 61);
impl_pin!(PB30, B, 30, 62);
impl_pin!(PB31, B, 31, 63);

impl_pin!(PX00, X, 0, 416);
impl_pin!(PX01, X, 1, 417);
impl_pin!(PX02, X, 2, 418);
impl_pin!(PX03, X, 3, 419);
impl_pin!(PX04, X, 4, 420);
impl_pin!(PX05, X, 5, 421);
impl_pin!(PX06, X, 6, 422);
impl_pin!(PX07, X, 7, 423);

impl_pin!(PY00, Y, 0, 448);
impl_pin!(PY01, Y, 1, 449);
impl_pin!(PY02, Y, 2, 450);
impl_pin!(PY03, Y, 3, 451);
impl_pin!(PY04, Y, 4, 452);
impl_pin!(PY05, Y, 5, 453);
impl_pin!(PY06, Y, 6, 454);
impl_pin!(PY07, Y, 7, 455);
