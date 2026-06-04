///Register `VALUE` reader
pub type R = crate::R<ValueSpec>;
///Register `VALUE` writer
pub type W = crate::W<ValueSpec>;
///Field `LINK` reader - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3
pub type LinkR = crate::FieldReader;
///Field `LINK` writer - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, ValueSpec> {
        LinkW::new(self, 0)
    }
}
/**Affiliate of Group

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
