///Register `PENDING` reader
pub type R = crate::R<PendingSpec>;
///Register `PENDING` writer
pub type W = crate::W<PendingSpec>;
///Field `INTERRUPT` reader - writing 1 to trigger software interrupt
pub type InterruptR = crate::BitReader;
///Field `INTERRUPT` writer - writing 1 to trigger software interrupt
pub type InterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - writing 1 to trigger software interrupt
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - writing 1 to trigger software interrupt
    #[inline(always)]
    pub fn interrupt(&mut self) -> InterruptW<'_, PendingSpec> {
        InterruptW::new(self, 1)
    }
}
/**Pending status

You can [`read`](crate::Reg::read) this register and get [`pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PendingSpec;
impl crate::RegisterSpec for PendingSpec {
    type Ux = u32;
}
///`read()` method returns [`pending::R`](R) reader structure
impl crate::Readable for PendingSpec {}
///`write(|w| ..)` method takes [`pending::W`](W) writer structure
impl crate::Writable for PendingSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PENDING to value 0
impl crate::Resettable for PendingSpec {}
