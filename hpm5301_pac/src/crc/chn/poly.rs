///Register `poly` reader
pub type R = crate::R<PolySpec>;
///Register `poly` writer
pub type W = crate::W<PolySpec>;
///Field `POLY` reader - poly setting
pub type PolyR = crate::FieldReader<u32>;
///Field `POLY` writer - poly setting
pub type PolyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - poly setting
    #[inline(always)]
    pub fn poly(&self) -> PolyR {
        PolyR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - poly setting
    #[inline(always)]
    pub fn poly(&mut self) -> PolyW<'_, PolySpec> {
        PolyW::new(self, 0)
    }
}
/**chn&index0 poly

You can [`read`](crate::Reg::read) this register and get [`poly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PolySpec;
impl crate::RegisterSpec for PolySpec {
    type Ux = u32;
}
///`read()` method returns [`poly::R`](R) reader structure
impl crate::Readable for PolySpec {}
///`write(|w| ..)` method takes [`poly::W`](W) writer structure
impl crate::Writable for PolySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets poly to value 0
impl crate::Resettable for PolySpec {}
