///Register `VALUE` reader
pub type R = crate::R<ValueSpec>;
///Register `VALUE` writer
pub type W = crate::W<ValueSpec>;
///Field `LINK` reader - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed
pub type LinkR = crate::FieldReader<u32>;
///Field `LINK` writer - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, ValueSpec> {
        LinkW::new(self, 0)
    }
}
/**Group setting

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
