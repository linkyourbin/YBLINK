///Register `DGO_CTR0` reader
pub type R = crate::R<DgoCtr0Spec>;
///Register `DGO_CTR0` writer
pub type W = crate::W<DgoCtr0Spec>;
///Field `RETENTION` reader - dgo register status retenion
pub type RetentionR = crate::BitReader;
///Field `RETENTION` writer - dgo register status retenion
pub type RetentionW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - dgo register status retenion
    #[inline(always)]
    pub fn retention(&self) -> RetentionR {
        RetentionR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - dgo register status retenion
    #[inline(always)]
    pub fn retention(&mut self) -> RetentionW<'_, DgoCtr0Spec> {
        RetentionW::new(self, 16)
    }
}
/**control register 0

You can [`read`](crate::Reg::read) this register and get [`dgo_ctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_ctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoCtr0Spec;
impl crate::RegisterSpec for DgoCtr0Spec {
    type Ux = u32;
}
///`read()` method returns [`dgo_ctr0::R`](R) reader structure
impl crate::Readable for DgoCtr0Spec {}
///`write(|w| ..)` method takes [`dgo_ctr0::W`](W) writer structure
impl crate::Writable for DgoCtr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_CTR0 to value 0
impl crate::Resettable for DgoCtr0Spec {}
