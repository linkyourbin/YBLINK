///Register `SrcAddr` reader
pub type R = crate::R<SrcAddrSpec>;
///Register `SrcAddr` writer
pub type W = crate::W<SrcAddrSpec>;
///Field `SRCADDRL` reader - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered.
pub type SrcaddrlR = crate::FieldReader<u32>;
///Field `SRCADDRL` writer - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered.
pub type SrcaddrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered.
    #[inline(always)]
    pub fn srcaddrl(&self) -> SrcaddrlR {
        SrcaddrlR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered.
    #[inline(always)]
    pub fn srcaddrl(&mut self) -> SrcaddrlW<'_, SrcAddrSpec> {
        SrcaddrlW::new(self, 0)
    }
}
/**Channel &index0 Source Address Low Part Register

You can [`read`](crate::Reg::read) this register and get [`src_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SrcAddrSpec;
impl crate::RegisterSpec for SrcAddrSpec {
    type Ux = u32;
}
///`read()` method returns [`src_addr::R`](R) reader structure
impl crate::Readable for SrcAddrSpec {}
///`write(|w| ..)` method takes [`src_addr::W`](W) writer structure
impl crate::Writable for SrcAddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SrcAddr to value 0
impl crate::Resettable for SrcAddrSpec {}
