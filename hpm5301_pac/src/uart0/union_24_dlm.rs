///Register `DLM` reader
pub type R = crate::R<Union24DlmSpec>;
///Register `DLM` writer
pub type W = crate::W<Union24DlmSpec>;
///Field `DLM` reader - Most significant byte of the Divisor Latch
pub type DlmR = crate::FieldReader;
///Field `DLM` writer - Most significant byte of the Divisor Latch
pub type DlmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Most significant byte of the Divisor Latch
    #[inline(always)]
    pub fn dlm(&self) -> DlmR {
        DlmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Most significant byte of the Divisor Latch
    #[inline(always)]
    pub fn dlm(&mut self) -> DlmW<'_, Union24DlmSpec> {
        DlmW::new(self, 0)
    }
}
/**Divisor Latch MSB (when DLAB = 1)

You can [`read`](crate::Reg::read) this register and get [`union_24_dlm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_24_dlm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union24DlmSpec;
impl crate::RegisterSpec for Union24DlmSpec {
    type Ux = u32;
}
///`read()` method returns [`union_24_dlm::R`](R) reader structure
impl crate::Readable for Union24DlmSpec {}
///`write(|w| ..)` method takes [`union_24_dlm::W`](W) writer structure
impl crate::Writable for Union24DlmSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLM to value 0
impl crate::Resettable for Union24DlmSpec {}
