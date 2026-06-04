///Register `SET` reader
pub type R = crate::R<SetSpec>;
///Register `SET` writer
pub type W = crate::W<SetSpec>;
///Field `IRQ_POL` reader - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge
pub type IrqPolR = crate::FieldReader<u32>;
///Field `IRQ_POL` writer - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge
pub type IrqPolW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge
    #[inline(always)]
    pub fn irq_pol(&self) -> IrqPolR {
        IrqPolR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge
    #[inline(always)]
    pub fn irq_pol(&mut self) -> IrqPolW<'_, SetSpec> {
        IrqPolW::new(self, 0)
    }
}
/**GPIO interrupt polarity set

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
