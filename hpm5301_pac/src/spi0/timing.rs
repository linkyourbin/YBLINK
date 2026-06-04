///Register `Timing` reader
pub type R = crate::R<TimingSpec>;
///Register `Timing` writer
pub type W = crate::W<TimingSpec>;
///Field `SCLK_DIV` reader - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency.
pub type SclkDivR = crate::FieldReader;
///Field `SCLK_DIV` writer - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency.
pub type SclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CSHT` reader - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2
pub type CshtR = crate::FieldReader;
///Field `CSHT` writer - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2
pub type CshtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CS2SCLK` reader - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2
pub type Cs2sclkR = crate::FieldReader;
///Field `CS2SCLK` writer - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2
pub type Cs2sclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency.
    #[inline(always)]
    pub fn sclk_div(&self) -> SclkDivR {
        SclkDivR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2
    #[inline(always)]
    pub fn csht(&self) -> CshtR {
        CshtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2
    #[inline(always)]
    pub fn cs2sclk(&self) -> Cs2sclkR {
        Cs2sclkR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    ///Bits 0:7 - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency.
    #[inline(always)]
    pub fn sclk_div(&mut self) -> SclkDivW<'_, TimingSpec> {
        SclkDivW::new(self, 0)
    }
    ///Bits 8:11 - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2
    #[inline(always)]
    pub fn csht(&mut self) -> CshtW<'_, TimingSpec> {
        CshtW::new(self, 8)
    }
    ///Bits 12:13 - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2
    #[inline(always)]
    pub fn cs2sclk(&mut self) -> Cs2sclkW<'_, TimingSpec> {
        Cs2sclkW::new(self, 12)
    }
}
/**Interface Timing Register

You can [`read`](crate::Reg::read) this register and get [`timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TimingSpec;
impl crate::RegisterSpec for TimingSpec {
    type Ux = u32;
}
///`read()` method returns [`timing::R`](R) reader structure
impl crate::Readable for TimingSpec {}
///`write(|w| ..)` method takes [`timing::W`](W) writer structure
impl crate::Writable for TimingSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Timing to value 0
impl crate::Resettable for TimingSpec {}
