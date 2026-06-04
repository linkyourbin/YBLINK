///Register `TOGGLE` reader
pub type R = crate::R<ToggleSpec>;
///Register `TOGGLE` writer
pub type W = crate::W<ToggleSpec>;
///Field `IRQ_DUAL` reader - GPIO dual edge interrupt enable toggle 0: keep original edge interrupt type 1: change original edge interrupt type to another one.
pub type IrqDualR = crate::BitReader;
///Field `IRQ_DUAL` writer - GPIO dual edge interrupt enable toggle 0: keep original edge interrupt type 1: change original edge interrupt type to another one.
pub type IrqDualW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIO dual edge interrupt enable toggle 0: keep original edge interrupt type 1: change original edge interrupt type to another one.
    #[inline(always)]
    pub fn irq_dual(&self) -> IrqDualR {
        IrqDualR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIO dual edge interrupt enable toggle 0: keep original edge interrupt type 1: change original edge interrupt type to another one.
    #[inline(always)]
    pub fn irq_dual(&mut self) -> IrqDualW<'_, ToggleSpec> {
        IrqDualW::new(self, 0)
    }
}
/**GPIO dual edge interrupt enable toggle

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
