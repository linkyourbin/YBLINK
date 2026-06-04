///Register `SET` reader
pub type R = crate::R<SetSpec>;
///Register `SET` writer
pub type W = crate::W<SetSpec>;
///Field `IRQ_DUAL` reader - GPIO dual edge interrupt enable set 0: keep original edge interrupt type 1: dual edge interrupt enable
pub type IrqDualR = crate::BitReader;
///Field `IRQ_DUAL` writer - GPIO dual edge interrupt enable set 0: keep original edge interrupt type 1: dual edge interrupt enable
pub type IrqDualW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIO dual edge interrupt enable set 0: keep original edge interrupt type 1: dual edge interrupt enable
    #[inline(always)]
    pub fn irq_dual(&self) -> IrqDualR {
        IrqDualR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIO dual edge interrupt enable set 0: keep original edge interrupt type 1: dual edge interrupt enable
    #[inline(always)]
    pub fn irq_dual(&mut self) -> IrqDualW<'_, SetSpec> {
        IrqDualW::new(self, 0)
    }
}
/**GPIO dual edge interrupt enable set

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
