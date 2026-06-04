///Register `TRACK_TARGET` reader
pub type R = crate::R<TrackTargetSpec>;
///Register `TRACK_TARGET` writer
pub type W = crate::W<TrackTargetSpec>;
///Field `TARGET` reader - Target frequency multiplier of divided source
pub type TargetR = crate::FieldReader<u16>;
///Field `TARGET` writer - Target frequency multiplier of divided source
pub type TargetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PRE_DIV` reader - Divider for reference source
pub type PreDivR = crate::FieldReader<u16>;
///Field `PRE_DIV` writer - Divider for reference source
pub type PreDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Target frequency multiplier of divided source
    #[inline(always)]
    pub fn target(&self) -> TargetR {
        TargetR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Divider for reference source
    #[inline(always)]
    pub fn pre_div(&self) -> PreDivR {
        PreDivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Target frequency multiplier of divided source
    #[inline(always)]
    pub fn target(&mut self) -> TargetW<'_, TrackTargetSpec> {
        TargetW::new(self, 0)
    }
    ///Bits 16:31 - Divider for reference source
    #[inline(always)]
    pub fn pre_div(&mut self) -> PreDivW<'_, TrackTargetSpec> {
        PreDivW::new(self, 16)
    }
}
/**RC 24M track target

You can [`read`](crate::Reg::read) this register and get [`track_target::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`track_target::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrackTargetSpec;
impl crate::RegisterSpec for TrackTargetSpec {
    type Ux = u32;
}
///`read()` method returns [`track_target::R`](R) reader structure
impl crate::Readable for TrackTargetSpec {}
///`write(|w| ..)` method takes [`track_target::W`](W) writer structure
impl crate::Writable for TrackTargetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRACK_TARGET to value 0
impl crate::Resettable for TrackTargetSpec {}
