///Register `AGE` reader
pub type R = crate::R<AgeSpec>;
///Register `AGE` writer
pub type W = crate::W<AgeSpec>;
///Field `AGE` reader - age of T register in 24MHz clock cycles
pub type AgeR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - age of T register in 24MHz clock cycles
    #[inline(always)]
    pub fn age(&self) -> AgeR {
        AgeR::new(self.bits)
    }
}
impl W {}
/**Sample age

You can [`read`](crate::Reg::read) this register and get [`age::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`age::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgeSpec;
impl crate::RegisterSpec for AgeSpec {
    type Ux = u32;
}
///`read()` method returns [`age::R`](R) reader structure
impl crate::Readable for AgeSpec {}
///`write(|w| ..)` method takes [`age::W`](W) writer structure
impl crate::Writable for AgeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGE to value 0
impl crate::Resettable for AgeSpec {}
