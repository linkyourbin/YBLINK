///Register `CAPPRD` reader
pub type R = crate::R<CapprdSpec>;
///Register `CAPPRD` writer
pub type W = crate::W<CapprdSpec>;
///Field `CAPPRD` reader - This register contains the input signal period when channel is configured to input capture measure mode.
pub type CapprdR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register contains the input signal period when channel is configured to input capture measure mode.
    #[inline(always)]
    pub fn capprd(&self) -> CapprdR {
        CapprdR::new(self.bits)
    }
}
impl W {}
/**PWM period measure register

You can [`read`](crate::Reg::read) this register and get [`capprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CapprdSpec;
impl crate::RegisterSpec for CapprdSpec {
    type Ux = u32;
}
///`read()` method returns [`capprd::R`](R) reader structure
impl crate::Readable for CapprdSpec {}
///`write(|w| ..)` method takes [`capprd::W`](W) writer structure
impl crate::Writable for CapprdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAPPRD to value 0
impl crate::Resettable for CapprdSpec {}
