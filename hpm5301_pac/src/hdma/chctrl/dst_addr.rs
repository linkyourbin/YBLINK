///Register `DstAddr` reader
pub type R = crate::R<DstAddrSpec>;
///Register `DstAddr` writer
pub type W = crate::W<DstAddrSpec>;
///Field `DSTADDRL` reader - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered.
pub type DstaddrlR = crate::FieldReader<u32>;
///Field `DSTADDRL` writer - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered.
pub type DstaddrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered.
    #[inline(always)]
    pub fn dstaddrl(&self) -> DstaddrlR {
        DstaddrlR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered.
    #[inline(always)]
    pub fn dstaddrl(&mut self) -> DstaddrlW<'_, DstAddrSpec> {
        DstaddrlW::new(self, 0)
    }
}
/**Channel &index0 Destination Address Low Part Register

You can [`read`](crate::Reg::read) this register and get [`dst_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DstAddrSpec;
impl crate::RegisterSpec for DstAddrSpec {
    type Ux = u32;
}
///`read()` method returns [`dst_addr::R`](R) reader structure
impl crate::Readable for DstAddrSpec {}
///`write(|w| ..)` method takes [`dst_addr::W`](W) writer structure
impl crate::Writable for DstAddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DstAddr to value 0
impl crate::Resettable for DstAddrSpec {}
