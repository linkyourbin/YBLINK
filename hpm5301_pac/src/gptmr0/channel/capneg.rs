///Register `CAPNEG` reader
pub type R = crate::R<CapnegSpec>;
///Register `CAPNEG` writer
pub type W = crate::W<CapnegSpec>;
///Field `CAPNEG` reader - This register contains the counter value captured at input signal falling edge
pub type CapnegR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register contains the counter value captured at input signal falling edge
    #[inline(always)]
    pub fn capneg(&self) -> CapnegR {
        CapnegR::new(self.bits)
    }
}
impl W {}
/**Capture falling edge register

You can [`read`](crate::Reg::read) this register and get [`capneg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capneg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CapnegSpec;
impl crate::RegisterSpec for CapnegSpec {
    type Ux = u32;
}
///`read()` method returns [`capneg::R`](R) reader structure
impl crate::Readable for CapnegSpec {}
///`write(|w| ..)` method takes [`capneg::W`](W) writer structure
impl crate::Writable for CapnegSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAPNEG to value 0
impl crate::Resettable for CapnegSpec {}
