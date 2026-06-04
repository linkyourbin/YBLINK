///Register `TOGGLE` reader
pub type R = crate::R<ToggleSpec>;
///Register `TOGGLE` writer
pub type W = crate::W<ToggleSpec>;
///Field `IRQ_EN` reader - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable
pub type IrqEnR = crate::FieldReader<u32>;
///Field `IRQ_EN` writer - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable
pub type IrqEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable
    #[inline(always)]
    pub fn irq_en(&self) -> IrqEnR {
        IrqEnR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable
    #[inline(always)]
    pub fn irq_en(&mut self) -> IrqEnW<'_, ToggleSpec> {
        IrqEnW::new(self, 0)
    }
}
/**GPIO interrupt enable toggle

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
