///Register `SCG_CTRL` reader
pub type R = crate::R<ScgCtrlSpec>;
///Register `SCG_CTRL` writer
pub type W = crate::W<ScgCtrlSpec>;
///Field `SCG` reader - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart
pub type ScgR = crate::FieldReader<u32>;
///Field `SCG` writer - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart
pub type ScgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart
    #[inline(always)]
    pub fn scg(&self) -> ScgR {
        ScgR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart
    #[inline(always)]
    pub fn scg(&mut self) -> ScgW<'_, ScgCtrlSpec> {
        ScgW::new(self, 0)
    }
}
/**Clock gate control in PMIC

You can [`read`](crate::Reg::read) this register and get [`scg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScgCtrlSpec;
impl crate::RegisterSpec for ScgCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`scg_ctrl::R`](R) reader structure
impl crate::Readable for ScgCtrlSpec {}
///`write(|w| ..)` method takes [`scg_ctrl::W`](W) writer structure
impl crate::Writable for ScgCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCG_CTRL to value 0xffff_ffff
impl crate::Resettable for ScgCtrlSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
