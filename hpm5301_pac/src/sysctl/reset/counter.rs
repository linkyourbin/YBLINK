///Register `counter` reader
pub type R = crate::R<CounterSpec>;
///Register `counter` writer
pub type W = crate::W<CounterSpec>;
///Field `COUNTER` reader - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
pub type CounterR = crate::FieldReader<u32>;
///Field `COUNTER` writer - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn counter(&mut self) -> CounterW<'_, CounterSpec> {
        CounterW::new(self, 0)
    }
}
/**Reset Setting

You can [`read`](crate::Reg::read) this register and get [`counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CounterSpec;
impl crate::RegisterSpec for CounterSpec {
    type Ux = u32;
}
///`read()` method returns [`counter::R`](R) reader structure
impl crate::Readable for CounterSpec {}
///`write(|w| ..)` method takes [`counter::W`](W) writer structure
impl crate::Writable for CounterSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets counter to value 0
impl crate::Resettable for CounterSpec {}
