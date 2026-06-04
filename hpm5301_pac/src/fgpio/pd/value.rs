///Register `VALUE` reader
pub type R = crate::R<ValueSpec>;
///Register `VALUE` writer
pub type W = crate::W<ValueSpec>;
///Field `IRQ_DUAL` reader - GPIO dual edge interrupt enable 0: single edge interrupt 1: dual edge interrupt enable
pub type IrqDualR = crate::BitReader;
///Field `IRQ_DUAL` writer - GPIO dual edge interrupt enable 0: single edge interrupt 1: dual edge interrupt enable
pub type IrqDualW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIO dual edge interrupt enable 0: single edge interrupt 1: dual edge interrupt enable
    #[inline(always)]
    pub fn irq_dual(&self) -> IrqDualR {
        IrqDualR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIO dual edge interrupt enable 0: single edge interrupt 1: dual edge interrupt enable
    #[inline(always)]
    pub fn irq_dual(&mut self) -> IrqDualW<'_, ValueSpec> {
        IrqDualW::new(self, 0)
    }
}
/**GPIO dual edge interrupt enable value

You can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
///`read()` method returns [`value::R`](R) reader structure
impl crate::Readable for ValueSpec {}
///`write(|w| ..)` method takes [`value::W`](W) writer structure
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VALUE to value 0
impl crate::Resettable for ValueSpec {}
