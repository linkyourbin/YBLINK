///Register `CNTUPTVAL` reader
pub type R = crate::R<CntuptvalSpec>;
///Register `CNTUPTVAL` writer
pub type W = crate::W<CntuptvalSpec>;
///Field `CNTUPTVAL` reader - counter will be set to this value when software write cntupt bit in CR
pub type CntuptvalR = crate::FieldReader<u32>;
///Field `CNTUPTVAL` writer - counter will be set to this value when software write cntupt bit in CR
pub type CntuptvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - counter will be set to this value when software write cntupt bit in CR
    #[inline(always)]
    pub fn cntuptval(&self) -> CntuptvalR {
        CntuptvalR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - counter will be set to this value when software write cntupt bit in CR
    #[inline(always)]
    pub fn cntuptval(&mut self) -> CntuptvalW<'_, CntuptvalSpec> {
        CntuptvalW::new(self, 0)
    }
}
/**Counter update value register

You can [`read`](crate::Reg::read) this register and get [`cntuptval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntuptval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CntuptvalSpec;
impl crate::RegisterSpec for CntuptvalSpec {
    type Ux = u32;
}
///`read()` method returns [`cntuptval::R`](R) reader structure
impl crate::Readable for CntuptvalSpec {}
///`write(|w| ..)` method takes [`cntuptval::W`](W) writer structure
impl crate::Writable for CntuptvalSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTUPTVAL to value 0
impl crate::Resettable for CntuptvalSpec {}
