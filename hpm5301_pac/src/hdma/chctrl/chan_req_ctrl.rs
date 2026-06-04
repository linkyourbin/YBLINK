///Register `ChanReqCtrl` reader
pub type R = crate::R<ChanReqCtrlSpec>;
///Register `ChanReqCtrl` writer
pub type W = crate::W<ChanReqCtrlSpec>;
///Field `DSTREQSEL` reader - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to.
pub type DstreqselR = crate::FieldReader;
///Field `DSTREQSEL` writer - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to.
pub type DstreqselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SRCREQSEL` reader - Source DMA request select. Select the request/ack handshake pair that the source device is connected to.
pub type SrcreqselR = crate::FieldReader;
///Field `SRCREQSEL` writer - Source DMA request select. Select the request/ack handshake pair that the source device is connected to.
pub type SrcreqselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 16:20 - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to.
    #[inline(always)]
    pub fn dstreqsel(&self) -> DstreqselR {
        DstreqselR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - Source DMA request select. Select the request/ack handshake pair that the source device is connected to.
    #[inline(always)]
    pub fn srcreqsel(&self) -> SrcreqselR {
        SrcreqselR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 16:20 - Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to.
    #[inline(always)]
    pub fn dstreqsel(&mut self) -> DstreqselW<'_, ChanReqCtrlSpec> {
        DstreqselW::new(self, 16)
    }
    ///Bits 24:28 - Source DMA request select. Select the request/ack handshake pair that the source device is connected to.
    #[inline(always)]
    pub fn srcreqsel(&mut self) -> SrcreqselW<'_, ChanReqCtrlSpec> {
        SrcreqselW::new(self, 24)
    }
}
/**Channel &index0 DMA Request Control Register

You can [`read`](crate::Reg::read) this register and get [`chan_req_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_req_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ChanReqCtrlSpec;
impl crate::RegisterSpec for ChanReqCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`chan_req_ctrl::R`](R) reader structure
impl crate::Readable for ChanReqCtrlSpec {}
///`write(|w| ..)` method takes [`chan_req_ctrl::W`](W) writer structure
impl crate::Writable for ChanReqCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ChanReqCtrl to value 0
impl crate::Resettable for ChanReqCtrlSpec {}
