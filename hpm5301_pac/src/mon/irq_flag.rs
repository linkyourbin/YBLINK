///Register `IRQ_FLAG` reader
pub type R = crate::R<IrqFlagSpec>;
///Register `IRQ_FLAG` writer
pub type W = crate::W<IrqFlagSpec>;
///Field `FLAG` reader - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened
pub type FlagR = crate::FieldReader;
///Field `FLAG` writer - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened
pub type FlagW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, IrqFlagSpec> {
        FlagW::new(self, 0)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`irq_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IrqFlagSpec;
impl crate::RegisterSpec for IrqFlagSpec {
    type Ux = u32;
}
///`read()` method returns [`irq_flag::R`](R) reader structure
impl crate::Readable for IrqFlagSpec {}
///`write(|w| ..)` method takes [`irq_flag::W`](W) writer structure
impl crate::Writable for IrqFlagSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_FLAG to value 0
impl crate::Resettable for IrqFlagSpec {}
