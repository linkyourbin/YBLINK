///Register `CFG_PROT` reader
pub type R = crate::R<CfgProtSpec>;
///Register `CFG_PROT` writer
pub type W = crate::W<CfgProtSpec>;
///Field `UPD_PSD` reader - The password of unlocking register update
pub type UpdPsdR = crate::FieldReader<u16>;
///Field `UPD_PSD` writer - The password of unlocking register update
pub type UpdPsdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UPD_OT_TIME` reader - The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle
pub type UpdOtTimeR = crate::FieldReader;
///Field `UPD_OT_TIME` writer - The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle
pub type UpdOtTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:15 - The password of unlocking register update
    #[inline(always)]
    pub fn upd_psd(&self) -> UpdPsdR {
        UpdPsdR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:19 - The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle
    #[inline(always)]
    pub fn upd_ot_time(&self) -> UpdOtTimeR {
        UpdOtTimeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:15 - The password of unlocking register update
    #[inline(always)]
    pub fn upd_psd(&mut self) -> UpdPsdW<'_, CfgProtSpec> {
        UpdPsdW::new(self, 0)
    }
    ///Bits 16:19 - The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle
    #[inline(always)]
    pub fn upd_ot_time(&mut self) -> UpdOtTimeW<'_, CfgProtSpec> {
        UpdOtTimeW::new(self, 16)
    }
}
/**ctrl register protection register

You can [`read`](crate::Reg::read) this register and get [`cfg_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfgProtSpec;
impl crate::RegisterSpec for CfgProtSpec {
    type Ux = u32;
}
///`read()` method returns [`cfg_prot::R`](R) reader structure
impl crate::Readable for CfgProtSpec {}
///`write(|w| ..)` method takes [`cfg_prot::W`](W) writer structure
impl crate::Writable for CfgProtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFG_PROT to value 0
impl crate::Resettable for CfgProtSpec {}
