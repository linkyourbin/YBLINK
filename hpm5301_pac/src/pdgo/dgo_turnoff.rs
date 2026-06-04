///Register `DGO_TURNOFF` reader
pub type R = crate::R<DgoTurnoffSpec>;
///Register `DGO_TURNOFF` writer
pub type W = crate::W<DgoTurnoffSpec>;
///Field `COUNTER` writer - trunoff counter, counter stops when it counts down to 0, the trunoff occurs when the counter value is 1.
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - trunoff counter, counter stops when it counts down to 0, the trunoff occurs when the counter value is 1.
    #[inline(always)]
    pub fn counter(&mut self) -> CounterW<'_, DgoTurnoffSpec> {
        CounterW::new(self, 0)
    }
}
/**trunoff control

You can [`read`](crate::Reg::read) this register and get [`dgo_turnoff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgo_turnoff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DgoTurnoffSpec;
impl crate::RegisterSpec for DgoTurnoffSpec {
    type Ux = u32;
}
///`read()` method returns [`dgo_turnoff::R`](R) reader structure
impl crate::Readable for DgoTurnoffSpec {}
///`write(|w| ..)` method takes [`dgo_turnoff::W`](W) writer structure
impl crate::Writable for DgoTurnoffSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DGO_TURNOFF to value 0
impl crate::Resettable for DgoTurnoffSpec {}
