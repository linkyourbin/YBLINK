///Register `FUSE[%s]` reader
pub type R = crate::R<FuseSpec>;
///Register `FUSE[%s]` writer
pub type W = crate::W<FuseSpec>;
///Field `FUSE` reader - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)
pub type FuseR = crate::FieldReader<u32>;
///Field `FUSE` writer - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)
pub type FuseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)
    #[inline(always)]
    pub fn fuse(&self) -> FuseR {
        FuseR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)
    #[inline(always)]
    pub fn fuse(&mut self) -> FuseW<'_, FuseSpec> {
        FuseW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`fuse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fuse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FuseSpec;
impl crate::RegisterSpec for FuseSpec {
    type Ux = u32;
}
///`read()` method returns [`fuse::R`](R) reader structure
impl crate::Readable for FuseSpec {}
///`write(|w| ..)` method takes [`fuse::W`](W) writer structure
impl crate::Writable for FuseSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FUSE[%s] to value 0
impl crate::Resettable for FuseSpec {}
