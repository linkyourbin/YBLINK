///Register `UPPER_LIM_IRQ` reader
pub type R = crate::R<UpperLimIrqSpec>;
///Register `UPPER_LIM_IRQ` writer
pub type W = crate::W<UpperLimIrqSpec>;
///Field `T` reader - Maximum temperature for compare
pub type TR = crate::FieldReader<u32>;
///Field `T` writer - Maximum temperature for compare
pub type TW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Maximum temperature for compare
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Maximum temperature for compare
    #[inline(always)]
    pub fn t(&mut self) -> TW<'_, UpperLimIrqSpec> {
        TW::new(self, 0)
    }
}
/**Maximum temperature to interrupt

You can [`read`](crate::Reg::read) this register and get [`upper_lim_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upper_lim_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UpperLimIrqSpec;
impl crate::RegisterSpec for UpperLimIrqSpec {
    type Ux = u32;
}
///`read()` method returns [`upper_lim_irq::R`](R) reader structure
impl crate::Readable for UpperLimIrqSpec {}
///`write(|w| ..)` method takes [`upper_lim_irq::W`](W) writer structure
impl crate::Writable for UpperLimIrqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UPPER_LIM_IRQ to value 0
impl crate::Resettable for UpperLimIrqSpec {}
