///Register `BANDGAP` reader
pub type R = crate::R<BandgapSpec>;
///Register `BANDGAP` writer
pub type W = crate::W<BandgapSpec>;
///Field `VBG_P50_TRIM` reader - Banggap 1.0V output trim value
pub type VbgP50TrimR = crate::FieldReader;
///Field `VBG_P50_TRIM` writer - Banggap 1.0V output trim value
pub type VbgP50TrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `VBG_P65_TRIM` reader - Banggap 1.0V output trim value
pub type VbgP65TrimR = crate::FieldReader;
///Field `VBG_P65_TRIM` writer - Banggap 1.0V output trim value
pub type VbgP65TrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `VBG_1P0_TRIM` reader - Banggap 1.0V output trim value
pub type Vbg1p0TrimR = crate::FieldReader;
///Field `VBG_1P0_TRIM` writer - Banggap 1.0V output trim value
pub type Vbg1p0TrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `VBG_TRIMMED` reader - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed
pub type VbgTrimmedR = crate::BitReader;
///Field `VBG_TRIMMED` writer - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed
pub type VbgTrimmedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Banggap 1.0V output trim value
    #[inline(always)]
    pub fn vbg_p50_trim(&self) -> VbgP50TrimR {
        VbgP50TrimR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Banggap 1.0V output trim value
    #[inline(always)]
    pub fn vbg_p65_trim(&self) -> VbgP65TrimR {
        VbgP65TrimR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Banggap 1.0V output trim value
    #[inline(always)]
    pub fn vbg_1p0_trim(&self) -> Vbg1p0TrimR {
        Vbg1p0TrimR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed
    #[inline(always)]
    pub fn vbg_trimmed(&self) -> VbgTrimmedR {
        VbgTrimmedR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Banggap 1.0V output trim value
    #[inline(always)]
    pub fn vbg_p50_trim(&mut self) -> VbgP50TrimW<'_, BandgapSpec> {
        VbgP50TrimW::new(self, 0)
    }
    ///Bits 8:12 - Banggap 1.0V output trim value
    #[inline(always)]
    pub fn vbg_p65_trim(&mut self) -> VbgP65TrimW<'_, BandgapSpec> {
        VbgP65TrimW::new(self, 8)
    }
    ///Bits 16:20 - Banggap 1.0V output trim value
    #[inline(always)]
    pub fn vbg_1p0_trim(&mut self) -> Vbg1p0TrimW<'_, BandgapSpec> {
        Vbg1p0TrimW::new(self, 16)
    }
    ///Bit 31 - Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed
    #[inline(always)]
    pub fn vbg_trimmed(&mut self) -> VbgTrimmedW<'_, BandgapSpec> {
        VbgTrimmedW::new(self, 31)
    }
}
/**BANGGAP control

You can [`read`](crate::Reg::read) this register and get [`bandgap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bandgap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BandgapSpec;
impl crate::RegisterSpec for BandgapSpec {
    type Ux = u32;
}
///`read()` method returns [`bandgap::R`](R) reader structure
impl crate::Readable for BandgapSpec {}
///`write(|w| ..)` method takes [`bandgap::W`](W) writer structure
impl crate::Writable for BandgapSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BANDGAP to value 0x0010_1010
impl crate::Resettable for BandgapSpec {
    const RESET_VALUE: u32 = 0x0010_1010;
}
