///Register `IRQ_ENABLE` reader
pub type R = crate::R<IrqEnableSpec>;
///Register `IRQ_ENABLE` writer
pub type W = crate::W<IrqEnableSpec>;
///Field `ENABLE` reader - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled
pub type EnableR = crate::FieldReader;
///Field `ENABLE` writer - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, IrqEnableSpec> {
        EnableW::new(self, 0)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IrqEnableSpec;
impl crate::RegisterSpec for IrqEnableSpec {
    type Ux = u32;
}
///`read()` method returns [`irq_enable::R`](R) reader structure
impl crate::Readable for IrqEnableSpec {}
///`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure
impl crate::Writable for IrqEnableSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_ENABLE to value 0
impl crate::Resettable for IrqEnableSpec {}
