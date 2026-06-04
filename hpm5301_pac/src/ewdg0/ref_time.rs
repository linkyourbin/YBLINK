///Register `REF_TIME` reader
pub type R = crate::R<RefTimeSpec>;
///Register `REF_TIME` writer
pub type W = crate::W<RefTimeSpec>;
///Field `REFRESH_PERIOD` reader - The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
pub type RefreshPeriodR = crate::FieldReader<u16>;
///Field `REFRESH_PERIOD` writer - The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
pub type RefreshPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
    #[inline(always)]
    pub fn refresh_period(&self) -> RefreshPeriodR {
        RefreshPeriodR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
    #[inline(always)]
    pub fn refresh_period(&mut self) -> RefreshPeriodW<'_, RefTimeSpec> {
        RefreshPeriodW::new(self, 0)
    }
}
/**Refresh period value

You can [`read`](crate::Reg::read) this register and get [`ref_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RefTimeSpec;
impl crate::RegisterSpec for RefTimeSpec {
    type Ux = u32;
}
///`read()` method returns [`ref_time::R`](R) reader structure
impl crate::Readable for RefTimeSpec {}
///`write(|w| ..)` method takes [`ref_time::W`](W) writer structure
impl crate::Writable for RefTimeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REF_TIME to value 0
impl crate::Resettable for RefTimeSpec {}
