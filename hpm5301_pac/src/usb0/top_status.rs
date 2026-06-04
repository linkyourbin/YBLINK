///Register `TOP_STATUS` reader
pub type R = crate::R<TopStatusSpec>;
///Register `TOP_STATUS` writer
pub type W = crate::W<TopStatusSpec>;
///Field `WAKEUP_INT_STATUS` reader - No description available
pub type WakeupIntStatusR = crate::BitReader;
///Field `WAKEUP_INT_STATUS` writer - No description available
pub type WakeupIntStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - No description available
    #[inline(always)]
    pub fn wakeup_int_status(&self) -> WakeupIntStatusR {
        WakeupIntStatusR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 31 - No description available
    #[inline(always)]
    pub fn wakeup_int_status(&mut self) -> WakeupIntStatusW<'_, TopStatusSpec> {
        WakeupIntStatusW::new(self, 31)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`top_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TopStatusSpec;
impl crate::RegisterSpec for TopStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`top_status::R`](R) reader structure
impl crate::Readable for TopStatusSpec {}
///`write(|w| ..)` method takes [`top_status::W`](W) writer structure
impl crate::Writable for TopStatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TOP_STATUS to value 0
impl crate::Resettable for TopStatusSpec {}
