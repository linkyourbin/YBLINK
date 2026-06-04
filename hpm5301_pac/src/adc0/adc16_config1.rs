///Register `adc16_config1` reader
pub type R = crate::R<Adc16Config1Spec>;
///Register `adc16_config1` writer
pub type W = crate::W<Adc16Config1Spec>;
///Field `COV_END_CNT` reader - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1).
pub type CovEndCntR = crate::FieldReader;
///Field `COV_END_CNT` writer - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1).
pub type CovEndCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 8:12 - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1).
    #[inline(always)]
    pub fn cov_end_cnt(&self) -> CovEndCntR {
        CovEndCntR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 8:12 - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1).
    #[inline(always)]
    pub fn cov_end_cnt(&mut self) -> CovEndCntW<'_, Adc16Config1Spec> {
        CovEndCntW::new(self, 8)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`adc16_config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc16_config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adc16Config1Spec;
impl crate::RegisterSpec for Adc16Config1Spec {
    type Ux = u32;
}
///`read()` method returns [`adc16_config1::R`](R) reader structure
impl crate::Readable for Adc16Config1Spec {}
///`write(|w| ..)` method takes [`adc16_config1::W`](W) writer structure
impl crate::Writable for Adc16Config1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets adc16_config1 to value 0x0100
impl crate::Resettable for Adc16Config1Spec {
    const RESET_VALUE: u32 = 0x0100;
}
