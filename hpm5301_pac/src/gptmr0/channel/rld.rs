///Register `RLD` reader
pub type R = crate::R<RldSpec>;
///Register `RLD` writer
pub type W = crate::W<RldSpec>;
///Field `RLD` reader - reload value
pub type RldR = crate::FieldReader<u32>;
///Field `RLD` writer - reload value
pub type RldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reload value
    #[inline(always)]
    pub fn rld(&self) -> RldR {
        RldR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - reload value
    #[inline(always)]
    pub fn rld(&mut self) -> RldW<'_, RldSpec> {
        RldW::new(self, 0)
    }
}
/**Reload register

You can [`read`](crate::Reg::read) this register and get [`rld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RldSpec;
impl crate::RegisterSpec for RldSpec {
    type Ux = u32;
}
///`read()` method returns [`rld::R`](R) reader structure
impl crate::Readable for RldSpec {}
///`write(|w| ..)` method takes [`rld::W`](W) writer structure
impl crate::Writable for RldSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RLD to value 0xffff_ffff
impl crate::Resettable for RldSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
