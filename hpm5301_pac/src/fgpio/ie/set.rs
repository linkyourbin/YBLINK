///Register `SET` reader
pub type R = crate::R<SetSpec>;
///Register `SET` writer
pub type W = crate::W<SetSpec>;
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
    pub fn irq_en(&mut self) -> IrqEnW<'_, SetSpec> {
        IrqEnW::new(self, 0)
    }
}
/**GPIO interrupt enable set

You can [`read`](crate::Reg::read) this register and get [`set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SetSpec;
impl crate::RegisterSpec for SetSpec {
    type Ux = u32;
}
///`read()` method returns [`set::R`](R) reader structure
impl crate::Readable for SetSpec {}
///`write(|w| ..)` method takes [`set::W`](W) writer structure
impl crate::Writable for SetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SET to value 0
impl crate::Resettable for SetSpec {}
