///Register `STATUS` reader
pub type R = crate::R<StatusSpec>;
///Register `STATUS` writer
pub type W = crate::W<StatusSpec>;
///Field `FLAG` reader - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected
pub type FlagR = crate::BitReader;
///Field `FLAG` writer - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected
pub type FlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, StatusSpec> {
        FlagW::new(self, 0)
    }
}
/**Glitch and clock monitor status

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for StatusSpec {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for StatusSpec {}
