///Register `LOWER_LIM_IRQ` reader
pub type R = crate::R<LowerLimIrqSpec>;
///Register `LOWER_LIM_IRQ` writer
pub type W = crate::W<LowerLimIrqSpec>;
///Field `T` reader - Minimum temperature for compare
pub type TR = crate::FieldReader<u32>;
///Field `T` writer - Minimum temperature for compare
pub type TW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Minimum temperature for compare
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Minimum temperature for compare
    #[inline(always)]
    pub fn t(&mut self) -> TW<'_, LowerLimIrqSpec> {
        TW::new(self, 0)
    }
}
/**Minimum temperature to interrupt

You can [`read`](crate::Reg::read) this register and get [`lower_lim_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lower_lim_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LowerLimIrqSpec;
impl crate::RegisterSpec for LowerLimIrqSpec {
    type Ux = u32;
}
///`read()` method returns [`lower_lim_irq::R`](R) reader structure
impl crate::Readable for LowerLimIrqSpec {}
///`write(|w| ..)` method takes [`lower_lim_irq::W`](W) writer structure
impl crate::Writable for LowerLimIrqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOWER_LIM_IRQ to value 0
impl crate::Resettable for LowerLimIrqSpec {}
