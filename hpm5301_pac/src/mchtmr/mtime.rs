///Register `MTIME` reader
pub type R = crate::R<MtimeSpec>;
///Register `MTIME` writer
pub type W = crate::W<MtimeSpec>;
///Field `MTIME` reader - Machine time
pub type MtimeR = crate::FieldReader<u64>;
///Field `MTIME` writer - Machine time
pub type MtimeW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    ///Bits 0:63 - Machine time
    #[inline(always)]
    pub fn mtime(&self) -> MtimeR {
        MtimeR::new(self.bits)
    }
}
impl W {
    ///Bits 0:63 - Machine time
    #[inline(always)]
    pub fn mtime(&mut self) -> MtimeW<'_, MtimeSpec> {
        MtimeW::new(self, 0)
    }
}
/**Machine Time

You can [`read`](crate::Reg::read) this register and get [`mtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MtimeSpec;
impl crate::RegisterSpec for MtimeSpec {
    type Ux = u64;
}
///`read()` method returns [`mtime::R`](R) reader structure
impl crate::Readable for MtimeSpec {}
///`write(|w| ..)` method takes [`mtime::W`](W) writer structure
impl crate::Writable for MtimeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTIME to value 0x0002_0210
impl crate::Resettable for MtimeSpec {
    const RESET_VALUE: u64 = 0x0002_0210;
}
