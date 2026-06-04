///Register `PENDING[%s]` reader
pub type R = crate::R<PendingSpec>;
///Register `PENDING[%s]` writer
pub type W = crate::W<PendingSpec>;
///Field `INTERRUPT` reader - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit.
pub type InterruptR = crate::FieldReader<u32>;
///Field `INTERRUPT` writer - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit.
pub type InterruptW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit.
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit.
    #[inline(always)]
    pub fn interrupt(&mut self) -> InterruptW<'_, PendingSpec> {
        InterruptW::new(self, 0)
    }
}
/**no description available

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
///`reset()` method sets PENDING[%s] to value 0
impl crate::Resettable for PendingSpec {}
