///Register `LLPointer` reader
pub type R = crate::R<LlpointerSpec>;
///Register `LLPointer` writer
pub type W = crate::W<LlpointerSpec>;
///Field `LLPOINTERL` reader - Low part of the pointer to the next descriptor. The pointer must be double word aligned.
pub type LlpointerlR = crate::FieldReader<u32>;
///Field `LLPOINTERL` writer - Low part of the pointer to the next descriptor. The pointer must be double word aligned.
pub type LlpointerlW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 3:31 - Low part of the pointer to the next descriptor. The pointer must be double word aligned.
    #[inline(always)]
    pub fn llpointerl(&self) -> LlpointerlR {
        LlpointerlR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    ///Bits 3:31 - Low part of the pointer to the next descriptor. The pointer must be double word aligned.
    #[inline(always)]
    pub fn llpointerl(&mut self) -> LlpointerlW<'_, LlpointerSpec> {
        LlpointerlW::new(self, 3)
    }
}
/**Channel &index0 Linked List Pointer Low Part Register

You can [`read`](crate::Reg::read) this register and get [`llpointer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llpointer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LlpointerSpec;
impl crate::RegisterSpec for LlpointerSpec {
    type Ux = u32;
}
///`read()` method returns [`llpointer::R`](R) reader structure
impl crate::Readable for LlpointerSpec {}
///`write(|w| ..)` method takes [`llpointer::W`](W) writer structure
impl crate::Writable for LlpointerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LLPointer to value 0
impl crate::Resettable for LlpointerSpec {}
