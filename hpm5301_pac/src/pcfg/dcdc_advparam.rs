///Register `DCDC_ADVPARAM` reader
pub type R = crate::R<DcdcAdvparamSpec>;
///Register `DCDC_ADVPARAM` writer
pub type W = crate::W<DcdcAdvparamSpec>;
///Field `MAX_DUT` reader - maximum duty cycle
pub type MaxDutR = crate::FieldReader;
///Field `MAX_DUT` writer - maximum duty cycle
pub type MaxDutW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `MIN_DUT` reader - minimum duty cycle
pub type MinDutR = crate::FieldReader;
///Field `MIN_DUT` writer - minimum duty cycle
pub type MinDutW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - maximum duty cycle
    #[inline(always)]
    pub fn max_dut(&self) -> MaxDutR {
        MaxDutR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - minimum duty cycle
    #[inline(always)]
    pub fn min_dut(&self) -> MinDutR {
        MinDutR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - maximum duty cycle
    #[inline(always)]
    pub fn max_dut(&mut self) -> MaxDutW<'_, DcdcAdvparamSpec> {
        MaxDutW::new(self, 0)
    }
    ///Bits 8:14 - minimum duty cycle
    #[inline(always)]
    pub fn min_dut(&mut self) -> MinDutW<'_, DcdcAdvparamSpec> {
        MinDutW::new(self, 8)
    }
}
/**DCDC advance parameter

You can [`read`](crate::Reg::read) this register and get [`dcdc_advparam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_advparam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcAdvparamSpec;
impl crate::RegisterSpec for DcdcAdvparamSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_advparam::R`](R) reader structure
impl crate::Readable for DcdcAdvparamSpec {}
///`write(|w| ..)` method takes [`dcdc_advparam::W`](W) writer structure
impl crate::Writable for DcdcAdvparamSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_ADVPARAM to value 0x6e1c
impl crate::Resettable for DcdcAdvparamSpec {
    const RESET_VALUE: u32 = 0x6e1c;
}
