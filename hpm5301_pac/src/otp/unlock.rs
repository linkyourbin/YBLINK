///Register `UNLOCK` reader
pub type R = crate::R<UnlockSpec>;
///Register `UNLOCK` writer
pub type W = crate::W<UnlockSpec>;
///Field `UNLOCK` reader - unlock word for fuse array operation write "OPEN" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly
pub type UnlockR = crate::FieldReader<u32>;
///Field `UNLOCK` writer - unlock word for fuse array operation write "OPEN" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly
pub type UnlockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - unlock word for fuse array operation write "OPEN" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - unlock word for fuse array operation write "OPEN" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly
    #[inline(always)]
    pub fn unlock(&mut self) -> UnlockW<'_, UnlockSpec> {
        UnlockW::new(self, 0)
    }
}
/**UNLOCK

You can [`read`](crate::Reg::read) this register and get [`unlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UnlockSpec;
impl crate::RegisterSpec for UnlockSpec {
    type Ux = u32;
}
///`read()` method returns [`unlock::R`](R) reader structure
impl crate::Readable for UnlockSpec {}
///`write(|w| ..)` method takes [`unlock::W`](W) writer structure
impl crate::Writable for UnlockSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UNLOCK to value 0
impl crate::Resettable for UnlockSpec {}
