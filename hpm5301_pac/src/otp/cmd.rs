///Register `CMD` reader
pub type R = crate::R<CmdSpec>;
///Register `CMD` writer
pub type W = crate::W<CmdSpec>;
///Field `CMD` reader - command to access fure array "BLOW" will update fuse word at ADDR to value hold in DATA "READ" will fetch fuse value in at ADDR to DATA register
pub type CmdR = crate::FieldReader<u32>;
///Field `CMD` writer - command to access fure array "BLOW" will update fuse word at ADDR to value hold in DATA "READ" will fetch fuse value in at ADDR to DATA register
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - command to access fure array "BLOW" will update fuse word at ADDR to value hold in DATA "READ" will fetch fuse value in at ADDR to DATA register
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - command to access fure array "BLOW" will update fuse word at ADDR to value hold in DATA "READ" will fetch fuse value in at ADDR to DATA register
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CmdSpec> {
        CmdW::new(self, 0)
    }
}
/**CMD

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
///`reset()` method sets CMD to value 0
impl crate::Resettable for CmdSpec {}
