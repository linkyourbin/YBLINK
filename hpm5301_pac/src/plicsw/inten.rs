///Register `INTEN` reader
pub type R = crate::R<IntenSpec>;
///Register `INTEN` writer
pub type W = crate::W<IntenSpec>;
///Field `INTERRUPT` reader - enable software interrupt
pub type InterruptR = crate::BitReader;
///Field `INTERRUPT` writer - enable software interrupt
pub type InterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable software interrupt
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - enable software interrupt
    #[inline(always)]
    pub fn interrupt(&mut self) -> InterruptW<'_, IntenSpec> {
        InterruptW::new(self, 0)
    }
}
/**Interrupt enable

You can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
///`read()` method returns [`inten::R`](R) reader structure
impl crate::Readable for IntenSpec {}
///`write(|w| ..)` method takes [`inten::W`](W) writer structure
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTEN to value 0
impl crate::Resettable for IntenSpec {}
