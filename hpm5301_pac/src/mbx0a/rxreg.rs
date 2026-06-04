///Register `RXREG` reader
pub type R = crate::R<RxregSpec>;
///Register `RXREG` writer
pub type W = crate::W<RxregSpec>;
///Field `RXREG` reader - Receive word message from other core.
pub type RxregR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Receive word message from other core.
    #[inline(always)]
    pub fn rxreg(&self) -> RxregR {
        RxregR::new(self.bits)
    }
}
impl W {}
/**Receive word message from other core.

You can [`read`](crate::Reg::read) this register and get [`rxreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RxregSpec;
impl crate::RegisterSpec for RxregSpec {
    type Ux = u32;
}
///`read()` method returns [`rxreg::R`](R) reader structure
impl crate::Readable for RxregSpec {}
///`write(|w| ..)` method takes [`rxreg::W`](W) writer structure
impl crate::Writable for RxregSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXREG to value 0
impl crate::Resettable for RxregSpec {}
