///Register `ChEN` reader
pub type R = crate::R<ChEnSpec>;
///Register `ChEN` writer
pub type W = crate::W<ChEnSpec>;
///Field `CHEN` reader - Alias of the Enable field of all ChnCtrl registers
pub type ChenR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Alias of the Enable field of all ChnCtrl registers
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new(self.bits)
    }
}
impl W {}
/**Channel Enable Register

You can [`read`](crate::Reg::read) this register and get [`ch_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ChEnSpec;
impl crate::RegisterSpec for ChEnSpec {
    type Ux = u32;
}
///`read()` method returns [`ch_en::R`](R) reader structure
impl crate::Readable for ChEnSpec {}
///`write(|w| ..)` method takes [`ch_en::W`](W) writer structure
impl crate::Writable for ChEnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ChEN to value 0
impl crate::Resettable for ChEnSpec {}
