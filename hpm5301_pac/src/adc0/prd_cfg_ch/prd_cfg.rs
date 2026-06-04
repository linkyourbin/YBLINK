///Register `prd_cfg` reader
pub type R = crate::R<PrdCfgSpec>;
///Register `prd_cfg` writer
pub type W = crate::W<PrdCfgSpec>;
///Field `PRD` reader - conver period, with prescale. Set to 0 means disable current channel
pub type PrdR = crate::FieldReader;
///Field `PRD` writer - conver period, with prescale. Set to 0 means disable current channel
pub type PrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PRESCALE` reader - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx
pub type PrescaleR = crate::FieldReader;
///Field `PRESCALE` writer - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:7 - conver period, with prescale. Set to 0 means disable current channel
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:12 - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:7 - conver period, with prescale. Set to 0 means disable current channel
    #[inline(always)]
    pub fn prd(&mut self) -> PrdW<'_, PrdCfgSpec> {
        PrdW::new(self, 0)
    }
    ///Bits 8:12 - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<'_, PrdCfgSpec> {
        PrescaleW::new(self, 8)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`prd_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prd_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PrdCfgSpec;
impl crate::RegisterSpec for PrdCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`prd_cfg::R`](R) reader structure
impl crate::Readable for PrdCfgSpec {}
///`write(|w| ..)` method takes [`prd_cfg::W`](W) writer structure
impl crate::Writable for PrdCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets prd_cfg to value 0
impl crate::Resettable for PrdCfgSpec {}
