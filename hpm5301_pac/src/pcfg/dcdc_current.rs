///Register `DCDC_CURRENT` reader
pub type R = crate::R<DcdcCurrentSpec>;
///Register `DCDC_CURRENT` writer
pub type W = crate::W<DcdcCurrentSpec>;
///Field `LEVEL` reader - DCDC current level, current level is num * 50mA
pub type LevelR = crate::FieldReader;
///Field `VALID` reader - Current level valid 0: data is invalid 1: data is valid
pub type ValidR = crate::BitReader;
///Field `ESTI_EN` reader - enable current measure
pub type EstiEnR = crate::BitReader;
///Field `ESTI_EN` writer - enable current measure
pub type EstiEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - DCDC current level, current level is num * 50mA
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Current level valid 0: data is invalid 1: data is valid
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - enable current measure
    #[inline(always)]
    pub fn esti_en(&self) -> EstiEnR {
        EstiEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 15 - enable current measure
    #[inline(always)]
    pub fn esti_en(&mut self) -> EstiEnW<'_, DcdcCurrentSpec> {
        EstiEnW::new(self, 15)
    }
}
/**DCDC current estimation

You can [`read`](crate::Reg::read) this register and get [`dcdc_current::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_current::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcCurrentSpec;
impl crate::RegisterSpec for DcdcCurrentSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_current::R`](R) reader structure
impl crate::Readable for DcdcCurrentSpec {}
///`write(|w| ..)` method takes [`dcdc_current::W`](W) writer structure
impl crate::Writable for DcdcCurrentSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_CURRENT to value 0
impl crate::Resettable for DcdcCurrentSpec {}
