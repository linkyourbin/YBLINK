///Register `lf_wait` reader
pub type R = crate::R<LfWaitSpec>;
///Register `lf_wait` writer
pub type W = crate::W<LfWaitSpec>;
///Field `WAIT` reader - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
pub type WaitR = crate::FieldReader<u32>;
///Field `WAIT` writer - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
pub type WaitW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<'_, LfWaitSpec> {
        WaitW::new(self, 0)
    }
}
/**Power Setting

You can [`read`](crate::Reg::read) this register and get [`lf_wait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lf_wait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LfWaitSpec;
impl crate::RegisterSpec for LfWaitSpec {
    type Ux = u32;
}
///`read()` method returns [`lf_wait::R`](R) reader structure
impl crate::Readable for LfWaitSpec {}
///`write(|w| ..)` method takes [`lf_wait::W`](W) writer structure
impl crate::Writable for LfWaitSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets lf_wait to value 0xff
impl crate::Resettable for LfWaitSpec {
    const RESET_VALUE: u32 = 0xff;
}
