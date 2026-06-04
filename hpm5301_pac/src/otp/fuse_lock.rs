///Register `FUSE_LOCK[%s]` reader
pub type R = crate::R<FuseLockSpec>;
///Register `FUSE_LOCK[%s]` writer
pub type W = crate::W<FuseLockSpec>;
///Field `LOCK` reader - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked
pub type LockR = crate::FieldReader<u32>;
///Field `LOCK` writer - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, FuseLockSpec> {
        LockW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`fuse_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fuse_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FuseLockSpec;
impl crate::RegisterSpec for FuseLockSpec {
    type Ux = u32;
}
///`read()` method returns [`fuse_lock::R`](R) reader structure
impl crate::Readable for FuseLockSpec {}
///`write(|w| ..)` method takes [`fuse_lock::W`](W) writer structure
impl crate::Writable for FuseLockSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FUSE_LOCK[%s] to value 0
impl crate::Resettable for FuseLockSpec {}
