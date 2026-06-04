///Register `TOGGLE` reader
pub type R = crate::R<ToggleSpec>;
///Register `TOGGLE` writer
pub type W = crate::W<ToggleSpec>;
///Field `IRQ_TYPE` reader - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge
pub type IrqTypeR = crate::FieldReader<u32>;
///Field `IRQ_TYPE` writer - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge
pub type IrqTypeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge
    #[inline(always)]
    pub fn irq_type(&self) -> IrqTypeR {
        IrqTypeR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge
    #[inline(always)]
    pub fn irq_type(&mut self) -> IrqTypeW<'_, ToggleSpec> {
        IrqTypeW::new(self, 0)
    }
}
/**GPIO interrupt type toggle

You can [`read`](crate::Reg::read) this register and get [`toggle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toggle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ToggleSpec;
impl crate::RegisterSpec for ToggleSpec {
    type Ux = u32;
}
///`read()` method returns [`toggle::R`](R) reader structure
impl crate::Readable for ToggleSpec {}
///`write(|w| ..)` method takes [`toggle::W`](W) writer structure
impl crate::Writable for ToggleSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TOGGLE to value 0
impl crate::Resettable for ToggleSpec {}
