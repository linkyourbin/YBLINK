///Register `TMAX` reader
pub type R = crate::R<TmaxSpec>;
///Register `TMAX` writer
pub type W = crate::W<TmaxSpec>;
///Field `T` reader - maximum temperature ever found
pub type TR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - maximum temperature ever found
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(self.bits)
    }
}
impl W {}
/**Maximum Temperature

You can [`read`](crate::Reg::read) this register and get [`tmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TmaxSpec;
impl crate::RegisterSpec for TmaxSpec {
    type Ux = u32;
}
///`read()` method returns [`tmax::R`](R) reader structure
impl crate::Readable for TmaxSpec {}
///`write(|w| ..)` method takes [`tmax::W`](W) writer structure
impl crate::Writable for TmaxSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMAX to value 0xff80_0000
impl crate::Resettable for TmaxSpec {
    const RESET_VALUE: u32 = 0xff80_0000;
}
