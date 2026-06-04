///Register `CLEAR` reader
pub type R = crate::R<ClearSpec>;
///Register `CLEAR` writer
pub type W = crate::W<ClearSpec>;
///Field `IRQ_DUAL` reader - GPIO dual edge interrupt enable clear 0: keep original edge interrupt type 1: single edge interrupt enable
pub type IrqDualR = crate::BitReader;
///Field `IRQ_DUAL` writer - GPIO dual edge interrupt enable clear 0: keep original edge interrupt type 1: single edge interrupt enable
pub type IrqDualW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIO dual edge interrupt enable clear 0: keep original edge interrupt type 1: single edge interrupt enable
    #[inline(always)]
    pub fn irq_dual(&self) -> IrqDualR {
        IrqDualR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIO dual edge interrupt enable clear 0: keep original edge interrupt type 1: single edge interrupt enable
    #[inline(always)]
    pub fn irq_dual(&mut self) -> IrqDualW<'_, ClearSpec> {
        IrqDualW::new(self, 0)
    }
}
/**GPIO dual edge interrupt enable clear

You can [`read`](crate::Reg::read) this register and get [`clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u32;
}
///`read()` method returns [`clear::R`](R) reader structure
impl crate::Readable for ClearSpec {}
///`write(|w| ..)` method takes [`clear::W`](W) writer structure
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLEAR to value 0
impl crate::Resettable for ClearSpec {}
