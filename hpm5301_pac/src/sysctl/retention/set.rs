///Register `SET` reader
pub type R = crate::R<SetSpec>;
///Register `SET` writer
pub type W = crate::W<SetSpec>;
///Field `LINK` reader - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: keep
pub type LinkR = crate::FieldReader<u16>;
///Field `LINK` writer - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: keep
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: keep
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: keep
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, SetSpec> {
        LinkW::new(self, 0)
    }
}
/**Retention Control

You can [`read`](crate::Reg::read) this register and get [`set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SetSpec;
impl crate::RegisterSpec for SetSpec {
    type Ux = u32;
}
///`read()` method returns [`set::R`](R) reader structure
impl crate::Readable for SetSpec {}
///`write(|w| ..)` method takes [`set::W`](W) writer structure
impl crate::Writable for SetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SET to value 0
impl crate::Resettable for SetSpec {}
