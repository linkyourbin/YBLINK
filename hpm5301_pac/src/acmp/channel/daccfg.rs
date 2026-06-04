///Register `daccfg` reader
pub type R = crate::R<DaccfgSpec>;
///Register `daccfg` writer
pub type W = crate::W<DaccfgSpec>;
///Field `DACCFG` reader - 8bit DAC digital value
pub type DaccfgR = crate::FieldReader;
///Field `DACCFG` writer - 8bit DAC digital value
pub type DaccfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - 8bit DAC digital value
    #[inline(always)]
    pub fn daccfg(&self) -> DaccfgR {
        DaccfgR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - 8bit DAC digital value
    #[inline(always)]
    pub fn daccfg(&mut self) -> DaccfgW<'_, DaccfgSpec> {
        DaccfgW::new(self, 0)
    }
}
/**DAC configure register

You can [`read`](crate::Reg::read) this register and get [`daccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DaccfgSpec;
impl crate::RegisterSpec for DaccfgSpec {
    type Ux = u32;
}
///`read()` method returns [`daccfg::R`](R) reader structure
impl crate::Readable for DaccfgSpec {}
///`write(|w| ..)` method takes [`daccfg::W`](W) writer structure
impl crate::Writable for DaccfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets daccfg to value 0
impl crate::Resettable for DaccfgSpec {}
