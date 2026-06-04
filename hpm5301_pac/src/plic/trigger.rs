///Register `TRIGGER[%s]` reader
pub type R = crate::R<TriggerSpec>;
///Register `TRIGGER[%s]` writer
pub type W = crate::W<TriggerSpec>;
///Field `INTERRUPT` reader - The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt
pub type InterruptR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new(self.bits)
    }
}
impl W {}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TriggerSpec;
impl crate::RegisterSpec for TriggerSpec {
    type Ux = u32;
}
///`read()` method returns [`trigger::R`](R) reader structure
impl crate::Readable for TriggerSpec {}
///`write(|w| ..)` method takes [`trigger::W`](W) writer structure
impl crate::Writable for TriggerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRIGGER[%s] to value 0
impl crate::Resettable for TriggerSpec {}
