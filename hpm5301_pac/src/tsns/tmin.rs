///Register `TMIN` reader
pub type R = crate::R<TminSpec>;
///Register `TMIN` writer
pub type W = crate::W<TminSpec>;
///Field `T` reader - minimum temperature ever found
pub type TR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - minimum temperature ever found
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(self.bits)
    }
}
impl W {}
/**Minimum Temperature

You can [`read`](crate::Reg::read) this register and get [`tmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TminSpec;
impl crate::RegisterSpec for TminSpec {
    type Ux = u32;
}
///`read()` method returns [`tmin::R`](R) reader structure
impl crate::Readable for TminSpec {}
///`write(|w| ..)` method takes [`tmin::W`](W) writer structure
impl crate::Writable for TminSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMIN to value 0x007f_ffff
impl crate::Resettable for TminSpec {
    const RESET_VALUE: u32 = 0x007f_ffff;
}
