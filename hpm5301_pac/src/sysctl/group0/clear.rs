///Register `CLEAR` reader
pub type R = crate::R<ClearSpec>;
///Register `CLEAR` writer
pub type W = crate::W<ClearSpec>;
///Field `LINK` reader - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed
pub type LinkR = crate::FieldReader<u32>;
///Field `LINK` writer - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, ClearSpec> {
        LinkW::new(self, 0)
    }
}
/**Group setting

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
