///Register `ADC16_PARAMS[%s]` reader
pub type R = crate::R<Adc16ParamsSpec>;
///Register `ADC16_PARAMS[%s]` writer
pub type W = crate::W<Adc16ParamsSpec>;
///Field `PARAM_VAL` reader - No description available
pub type ParamValR = crate::FieldReader<u16>;
///Field `PARAM_VAL` writer - No description available
pub type ParamValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - No description available
    #[inline(always)]
    pub fn param_val(&self) -> ParamValR {
        ParamValR::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - No description available
    #[inline(always)]
    pub fn param_val(&mut self) -> ParamValW<'_, Adc16ParamsSpec> {
        ParamValW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`adc16_params::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc16_params::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adc16ParamsSpec;
impl crate::RegisterSpec for Adc16ParamsSpec {
    type Ux = u16;
}
///`read()` method returns [`adc16_params::R`](R) reader structure
impl crate::Readable for Adc16ParamsSpec {}
///`write(|w| ..)` method takes [`adc16_params::W`](W) writer structure
impl crate::Writable for Adc16ParamsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC16_PARAMS[%s] to value 0
impl crate::Resettable for Adc16ParamsSpec {}
