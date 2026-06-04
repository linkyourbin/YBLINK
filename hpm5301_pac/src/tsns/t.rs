///Register `T` reader
pub type R = crate::R<TSpec>;
///Register `T` writer
pub type W = crate::W<TSpec>;
///Field `T` reader - Signed number of temperature in 256 x celsius degree
pub type TR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Signed number of temperature in 256 x celsius degree
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(self.bits)
    }
}
impl W {}
/**Temperature

You can [`read`](crate::Reg::read) this register and get [`t::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSpec;
impl crate::RegisterSpec for TSpec {
    type Ux = u32;
}
///`read()` method returns [`t::R`](R) reader structure
impl crate::Readable for TSpec {}
///`write(|w| ..)` method takes [`t::W`](W) writer structure
impl crate::Writable for TSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets T to value 0
impl crate::Resettable for TSpec {}
