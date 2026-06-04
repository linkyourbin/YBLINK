///Register `GCR` reader
pub type R = crate::R<GcrSpec>;
///Register `GCR` writer
pub type W = crate::W<GcrSpec>;
///Field `TRGOPEN` reader - The bitfield enable the TRGM outputs.
pub type TrgopenR = crate::FieldReader;
///Field `TRGOPEN` writer - The bitfield enable the TRGM outputs.
pub type TrgopenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The bitfield enable the TRGM outputs.
    #[inline(always)]
    pub fn trgopen(&self) -> TrgopenR {
        TrgopenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - The bitfield enable the TRGM outputs.
    #[inline(always)]
    pub fn trgopen(&mut self) -> TrgopenW<'_, GcrSpec> {
        TrgopenW::new(self, 0)
    }
}
/**General Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GcrSpec;
impl crate::RegisterSpec for GcrSpec {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GcrSpec {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GcrSpec {}
