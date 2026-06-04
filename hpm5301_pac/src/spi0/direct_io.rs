///Register `DirectIO` reader
pub type R = crate::R<DirectIoSpec>;
///Register `DirectIO` writer
pub type W = crate::W<DirectIoSpec>;
///Field `CS_I` reader - Status of the SPI CS (chip select) signal
pub type CsIR = crate::BitReader;
///Field `SCLK_I` reader - Status of the SPI SCLK signal
pub type SclkIR = crate::BitReader;
///Field `MOSI_I` reader - Status of the SPI MOSI signal
pub type MosiIR = crate::BitReader;
///Field `MISO_I` reader - Status of the SPI MISO signal
pub type MisoIR = crate::BitReader;
///Field `WP_I` reader - Status of the SPI Flash write protect signal
pub type WpIR = crate::BitReader;
///Field `HOLD_I` reader - Status of the SPI Flash hold signal
pub type HoldIR = crate::BitReader;
///Field `CS_O` reader - Output value for the SPI CS (chip select) signal
pub type CsOR = crate::BitReader;
///Field `CS_O` writer - Output value for the SPI CS (chip select) signal
pub type CsOW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCLK_O` reader - Output value for the SPI SCLK signal
pub type SclkOR = crate::BitReader;
///Field `SCLK_O` writer - Output value for the SPI SCLK signal
pub type SclkOW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOSI_O` reader - Output value for the SPI MOSI signal
pub type MosiOR = crate::BitReader;
///Field `MOSI_O` writer - Output value for the SPI MOSI signal
pub type MosiOW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MISO_O` reader - Output value for the SPI MISO signal
pub type MisoOR = crate::BitReader;
///Field `MISO_O` writer - Output value for the SPI MISO signal
pub type MisoOW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP_O` reader - Output value for the SPI Flash write protect signal
pub type WpOR = crate::BitReader;
///Field `WP_O` writer - Output value for the SPI Flash write protect signal
pub type WpOW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD_O` reader - Output value for the SPI Flash hold signal
pub type HoldOR = crate::BitReader;
///Field `HOLD_O` writer - Output value for the SPI Flash hold signal
pub type HoldOW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS_OE` reader - Output enable for SPI CS (chip select) signal
pub type CsOeR = crate::BitReader;
///Field `CS_OE` writer - Output enable for SPI CS (chip select) signal
pub type CsOeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCLK_OE` reader - Output enable for the SPI SCLK signal
pub type SclkOeR = crate::BitReader;
///Field `SCLK_OE` writer - Output enable for the SPI SCLK signal
pub type SclkOeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOSI_OE` reader - Output enable for the SPI MOSI signal
pub type MosiOeR = crate::BitReader;
///Field `MOSI_OE` writer - Output enable for the SPI MOSI signal
pub type MosiOeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MISO_OE` reader - Output enable fo the SPI MISO signal
pub type MisoOeR = crate::BitReader;
///Field `MISO_OE` writer - Output enable fo the SPI MISO signal
pub type MisoOeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP_OE` reader - Output enable for the SPI Flash write protect signal
pub type WpOeR = crate::BitReader;
///Field `WP_OE` writer - Output enable for the SPI Flash write protect signal
pub type WpOeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD_OE` reader - Output enable for the SPI Flash hold signal
pub type HoldOeR = crate::BitReader;
///Field `HOLD_OE` writer - Output enable for the SPI Flash hold signal
pub type HoldOeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRECTIOEN` reader - Enable Direct IO 0x0: Disable 0x1: Enable
pub type DirectioenR = crate::BitReader;
///Field `DIRECTIOEN` writer - Enable Direct IO 0x0: Disable 0x1: Enable
pub type DirectioenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Status of the SPI CS (chip select) signal
    #[inline(always)]
    pub fn cs_i(&self) -> CsIR {
        CsIR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Status of the SPI SCLK signal
    #[inline(always)]
    pub fn sclk_i(&self) -> SclkIR {
        SclkIR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Status of the SPI MOSI signal
    #[inline(always)]
    pub fn mosi_i(&self) -> MosiIR {
        MosiIR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Status of the SPI MISO signal
    #[inline(always)]
    pub fn miso_i(&self) -> MisoIR {
        MisoIR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Status of the SPI Flash write protect signal
    #[inline(always)]
    pub fn wp_i(&self) -> WpIR {
        WpIR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Status of the SPI Flash hold signal
    #[inline(always)]
    pub fn hold_i(&self) -> HoldIR {
        HoldIR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Output value for the SPI CS (chip select) signal
    #[inline(always)]
    pub fn cs_o(&self) -> CsOR {
        CsOR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output value for the SPI SCLK signal
    #[inline(always)]
    pub fn sclk_o(&self) -> SclkOR {
        SclkOR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output value for the SPI MOSI signal
    #[inline(always)]
    pub fn mosi_o(&self) -> MosiOR {
        MosiOR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output value for the SPI MISO signal
    #[inline(always)]
    pub fn miso_o(&self) -> MisoOR {
        MisoOR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output value for the SPI Flash write protect signal
    #[inline(always)]
    pub fn wp_o(&self) -> WpOR {
        WpOR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output value for the SPI Flash hold signal
    #[inline(always)]
    pub fn hold_o(&self) -> HoldOR {
        HoldOR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Output enable for SPI CS (chip select) signal
    #[inline(always)]
    pub fn cs_oe(&self) -> CsOeR {
        CsOeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Output enable for the SPI SCLK signal
    #[inline(always)]
    pub fn sclk_oe(&self) -> SclkOeR {
        SclkOeR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Output enable for the SPI MOSI signal
    #[inline(always)]
    pub fn mosi_oe(&self) -> MosiOeR {
        MosiOeR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output enable fo the SPI MISO signal
    #[inline(always)]
    pub fn miso_oe(&self) -> MisoOeR {
        MisoOeR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output enable for the SPI Flash write protect signal
    #[inline(always)]
    pub fn wp_oe(&self) -> WpOeR {
        WpOeR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Output enable for the SPI Flash hold signal
    #[inline(always)]
    pub fn hold_oe(&self) -> HoldOeR {
        HoldOeR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Enable Direct IO 0x0: Disable 0x1: Enable
    #[inline(always)]
    pub fn directioen(&self) -> DirectioenR {
        DirectioenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - Output value for the SPI CS (chip select) signal
    #[inline(always)]
    pub fn cs_o(&mut self) -> CsOW<'_, DirectIoSpec> {
        CsOW::new(self, 8)
    }
    ///Bit 9 - Output value for the SPI SCLK signal
    #[inline(always)]
    pub fn sclk_o(&mut self) -> SclkOW<'_, DirectIoSpec> {
        SclkOW::new(self, 9)
    }
    ///Bit 10 - Output value for the SPI MOSI signal
    #[inline(always)]
    pub fn mosi_o(&mut self) -> MosiOW<'_, DirectIoSpec> {
        MosiOW::new(self, 10)
    }
    ///Bit 11 - Output value for the SPI MISO signal
    #[inline(always)]
    pub fn miso_o(&mut self) -> MisoOW<'_, DirectIoSpec> {
        MisoOW::new(self, 11)
    }
    ///Bit 12 - Output value for the SPI Flash write protect signal
    #[inline(always)]
    pub fn wp_o(&mut self) -> WpOW<'_, DirectIoSpec> {
        WpOW::new(self, 12)
    }
    ///Bit 13 - Output value for the SPI Flash hold signal
    #[inline(always)]
    pub fn hold_o(&mut self) -> HoldOW<'_, DirectIoSpec> {
        HoldOW::new(self, 13)
    }
    ///Bit 16 - Output enable for SPI CS (chip select) signal
    #[inline(always)]
    pub fn cs_oe(&mut self) -> CsOeW<'_, DirectIoSpec> {
        CsOeW::new(self, 16)
    }
    ///Bit 17 - Output enable for the SPI SCLK signal
    #[inline(always)]
    pub fn sclk_oe(&mut self) -> SclkOeW<'_, DirectIoSpec> {
        SclkOeW::new(self, 17)
    }
    ///Bit 18 - Output enable for the SPI MOSI signal
    #[inline(always)]
    pub fn mosi_oe(&mut self) -> MosiOeW<'_, DirectIoSpec> {
        MosiOeW::new(self, 18)
    }
    ///Bit 19 - Output enable fo the SPI MISO signal
    #[inline(always)]
    pub fn miso_oe(&mut self) -> MisoOeW<'_, DirectIoSpec> {
        MisoOeW::new(self, 19)
    }
    ///Bit 20 - Output enable for the SPI Flash write protect signal
    #[inline(always)]
    pub fn wp_oe(&mut self) -> WpOeW<'_, DirectIoSpec> {
        WpOeW::new(self, 20)
    }
    ///Bit 21 - Output enable for the SPI Flash hold signal
    #[inline(always)]
    pub fn hold_oe(&mut self) -> HoldOeW<'_, DirectIoSpec> {
        HoldOeW::new(self, 21)
    }
    ///Bit 24 - Enable Direct IO 0x0: Disable 0x1: Enable
    #[inline(always)]
    pub fn directioen(&mut self) -> DirectioenW<'_, DirectIoSpec> {
        DirectioenW::new(self, 24)
    }
}
/**Direct IO Control Register

You can [`read`](crate::Reg::read) this register and get [`direct_io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct_io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DirectIoSpec;
impl crate::RegisterSpec for DirectIoSpec {
    type Ux = u32;
}
///`read()` method returns [`direct_io::R`](R) reader structure
impl crate::Readable for DirectIoSpec {}
///`write(|w| ..)` method takes [`direct_io::W`](W) writer structure
impl crate::Writable for DirectIoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DirectIO to value 0x3100
impl crate::Resettable for DirectIoSpec {
    const RESET_VALUE: u32 = 0x3100;
}
