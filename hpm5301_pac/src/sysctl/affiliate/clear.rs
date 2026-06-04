///Register `CLEAR` reader
pub type R = crate::R<ClearSpec>;
///Register `CLEAR` writer
pub type W = crate::W<ClearSpec>;
///Field `LINK` reader - Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0
pub type LinkR = crate::FieldReader;
///Field `LINK` writer - Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, ClearSpec> {
        LinkW::new(self, 0)
    }
}
/**Affiliate of Group

You can [`read`](crate::Reg::read) this register and get [`clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u32;
}
///`read()` method returns [`clear::R`](R) reader structure
impl crate::Readable for ClearSpec {}
///`write(|w| ..)` method takes [`clear::W`](W) writer structure
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLEAR to value 0
impl crate::Resettable for ClearSpec {}
