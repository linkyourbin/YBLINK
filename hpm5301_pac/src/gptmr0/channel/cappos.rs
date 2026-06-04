///Register `CAPPOS` reader
pub type R = crate::R<CapposSpec>;
///Register `CAPPOS` writer
pub type W = crate::W<CapposSpec>;
///Field `CAPPOS` reader - This register contains the counter value captured at input signal rising edge
pub type CapposR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register contains the counter value captured at input signal rising edge
    #[inline(always)]
    pub fn cappos(&self) -> CapposR {
        CapposR::new(self.bits)
    }
}
impl W {}
/**Capture rising edge register

You can [`read`](crate::Reg::read) this register and get [`cappos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cappos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CapposSpec;
impl crate::RegisterSpec for CapposSpec {
    type Ux = u32;
}
///`read()` method returns [`cappos::R`](R) reader structure
impl crate::Readable for CapposSpec {}
///`write(|w| ..)` method takes [`cappos::W`](W) writer structure
impl crate::Writable for CapposSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAPPOS to value 0
impl crate::Resettable for CapposSpec {}
