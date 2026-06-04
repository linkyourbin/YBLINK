///Register `MTIMECMP` reader
pub type R = crate::R<MtimecmpSpec>;
///Register `MTIMECMP` writer
pub type W = crate::W<MtimecmpSpec>;
///Field `MTIMECMP` reader - Machine time compare
pub type MtimecmpR = crate::FieldReader<u64>;
///Field `MTIMECMP` writer - Machine time compare
pub type MtimecmpW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    ///Bits 0:63 - Machine time compare
    #[inline(always)]
    pub fn mtimecmp(&self) -> MtimecmpR {
        MtimecmpR::new(self.bits)
    }
}
impl W {
    ///Bits 0:63 - Machine time compare
    #[inline(always)]
    pub fn mtimecmp(&mut self) -> MtimecmpW<'_, MtimecmpSpec> {
        MtimecmpW::new(self, 0)
    }
}
/**Machine Time Compare

You can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MtimecmpSpec;
impl crate::RegisterSpec for MtimecmpSpec {
    type Ux = u64;
}
///`read()` method returns [`mtimecmp::R`](R) reader structure
impl crate::Readable for MtimecmpSpec {}
///`write(|w| ..)` method takes [`mtimecmp::W`](W) writer structure
impl crate::Writable for MtimecmpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTIMECMP to value 0x0002_0210
impl crate::Resettable for MtimecmpSpec {
    const RESET_VALUE: u64 = 0x0002_0210;
}
