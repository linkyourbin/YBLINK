///Register `Cmd` reader
pub type R = crate::R<CmdSpec>;
///Register `Cmd` writer
pub type W = crate::W<CmdSpec>;
///Field `CMD` reader - SPI Command
pub type CmdR = crate::FieldReader;
///Field `CMD` writer - SPI Command
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - SPI Command
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - SPI Command
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CmdSpec> {
        CmdW::new(self, 0)
    }
}
/**Command Register

You can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
///`read()` method returns [`cmd::R`](R) reader structure
impl crate::Readable for CmdSpec {}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Cmd to value 0
impl crate::Resettable for CmdSpec {}
