///Register `ENDPTSTAT` reader
pub type R = crate::R<EndptstatSpec>;
///Register `ENDPTSTAT` writer
pub type W = crate::W<EndptstatSpec>;
///Field `ERBR` reader - ERBR Endpoint Receive Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to a one by the hardware as a response to receiving a command from a corresponding bit in the ENDPRIME register. There is always a delay between setting a bit in the ENDPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ERBR\[N\] - Endpoint #N, N is in 0..7
pub type ErbrR = crate::FieldReader<u16>;
///Field `ETBR` reader - ETBR Endpoint Transmit Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to one by the hardware as a response to receiving a command from a corresponding bit in the ENDPTPRIME register. There is always a delay between setting a bit in the ENDPTPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ETBR\[N\] - Endpoint #N, N is in 0..7
pub type EtbrR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - ERBR Endpoint Receive Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to a one by the hardware as a response to receiving a command from a corresponding bit in the ENDPRIME register. There is always a delay between setting a bit in the ENDPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ERBR\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn erbr(&self) -> ErbrR {
        ErbrR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - ETBR Endpoint Transmit Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to one by the hardware as a response to receiving a command from a corresponding bit in the ENDPTPRIME register. There is always a delay between setting a bit in the ENDPTPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ETBR\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn etbr(&self) -> EtbrR {
        EtbrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
/**Endpoint Status Register

You can [`read`](crate::Reg::read) this register and get [`endptstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EndptstatSpec;
impl crate::RegisterSpec for EndptstatSpec {
    type Ux = u32;
}
///`read()` method returns [`endptstat::R`](R) reader structure
impl crate::Readable for EndptstatSpec {}
///`write(|w| ..)` method takes [`endptstat::W`](W) writer structure
impl crate::Writable for EndptstatSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTSTAT to value 0
impl crate::Resettable for EndptstatSpec {}
