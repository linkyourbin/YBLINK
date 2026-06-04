///Register `VALUE` reader
pub type R = crate::R<ValueSpec>;
///Register `VALUE` writer
pub type W = crate::W<ValueSpec>;
///Field `INPUT` reader - GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin
pub type InputR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin
    #[inline(always)]
    pub fn input(&self) -> InputR {
        InputR::new(self.bits)
    }
}
impl W {}
/**GPIO input value

You can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
///`read()` method returns [`value::R`](R) reader structure
impl crate::Readable for ValueSpec {}
///`write(|w| ..)` method takes [`value::W`](W) writer structure
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VALUE to value 0
impl crate::Resettable for ValueSpec {}
