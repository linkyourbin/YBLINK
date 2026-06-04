///Register `CAPDTY` reader
pub type R = crate::R<CapdtySpec>;
///Register `CAPDTY` writer
pub type W = crate::W<CapdtySpec>;
///Field `MEAS_HIGH` reader - This register contains the input signal duty cycle when channel is configured to input capture measure mode.
pub type MeasHighR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register contains the input signal duty cycle when channel is configured to input capture measure mode.
    #[inline(always)]
    pub fn meas_high(&self) -> MeasHighR {
        MeasHighR::new(self.bits)
    }
}
impl W {}
/**PWM duty cycle measure register

You can [`read`](crate::Reg::read) this register and get [`capdty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capdty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CapdtySpec;
impl crate::RegisterSpec for CapdtySpec {
    type Ux = u32;
}
///`read()` method returns [`capdty::R`](R) reader structure
impl crate::Readable for CapdtySpec {}
///`write(|w| ..)` method takes [`capdty::W`](W) writer structure
impl crate::Writable for CapdtySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAPDTY to value 0
impl crate::Resettable for CapdtySpec {}
