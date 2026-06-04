///Register `ret_wait` reader
pub type R = crate::R<RetWaitSpec>;
///Register `ret_wait` writer
pub type W = crate::W<RetWaitSpec>;
///Field `WAIT` reader - wait time for memory retention mode transition, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
pub type WaitR = crate::FieldReader<u32>;
///Field `WAIT` writer - wait time for memory retention mode transition, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
pub type WaitW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - wait time for memory retention mode transition, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - wait time for memory retention mode transition, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<'_, RetWaitSpec> {
        WaitW::new(self, 0)
    }
}
/**Power Setting

You can [`read`](crate::Reg::read) this register and get [`ret_wait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret_wait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RetWaitSpec;
impl crate::RegisterSpec for RetWaitSpec {
    type Ux = u32;
}
///`read()` method returns [`ret_wait::R`](R) reader structure
impl crate::Readable for RetWaitSpec {}
///`write(|w| ..)` method takes [`ret_wait::W`](W) writer structure
impl crate::Writable for RetWaitSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ret_wait to value 0x0f
impl crate::Resettable for RetWaitSpec {
    const RESET_VALUE: u32 = 0x0f;
}
