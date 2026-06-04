///Register `INTEN[%s]` reader
pub type R = crate::R<IntenSpec>;
///Register `INTEN[%s]` writer
pub type W = crate::W<IntenSpec>;
///Field `INTERRUPT` reader - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit.
pub type InterruptR = crate::FieldReader<u32>;
///Field `INTERRUPT` writer - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit.
pub type InterruptW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit.
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit.
    #[inline(always)]
    pub fn interrupt(&mut self) -> InterruptW<'_, IntenSpec> {
        InterruptW::new(self, 0)
    }
}
/**no description available

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
///`reset()` method sets INTEN[%s] to value 0
impl crate::Resettable for IntenSpec {}
