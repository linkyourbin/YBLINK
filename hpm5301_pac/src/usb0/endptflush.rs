///Register `ENDPTFLUSH` reader
pub type R = crate::R<EndptflushSpec>;
///Register `ENDPTFLUSH` writer
pub type W = crate::W<EndptflushSpec>;
///Field `FERB` reader - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\[N\] - Endpoint #N, N is in 0..7
pub type FerbR = crate::FieldReader<u16>;
///Field `FERB` writer - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\[N\] - Endpoint #N, N is in 0..7
pub type FerbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `FETB` reader - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\[N\] - Endpoint #N, N is in 0..7
pub type FetbR = crate::FieldReader<u16>;
///Field `FETB` writer - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\[N\] - Endpoint #N, N is in 0..7
pub type FetbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn ferb(&self) -> FerbR {
        FerbR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn fetb(&self) -> FetbR {
        FetbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn ferb(&mut self) -> FerbW<'_, EndptflushSpec> {
        FerbW::new(self, 0)
    }
    ///Bits 16:31 - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn fetb(&mut self) -> FetbW<'_, EndptflushSpec> {
        FetbW::new(self, 16)
    }
}
/**Endpoint Flush Register

You can [`read`](crate::Reg::read) this register and get [`endptflush::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptflush::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EndptflushSpec;
impl crate::RegisterSpec for EndptflushSpec {
    type Ux = u32;
}
///`read()` method returns [`endptflush::R`](R) reader structure
impl crate::Readable for EndptflushSpec {}
///`write(|w| ..)` method takes [`endptflush::W`](W) writer structure
impl crate::Writable for EndptflushSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTFLUSH to value 0
impl crate::Resettable for EndptflushSpec {}
