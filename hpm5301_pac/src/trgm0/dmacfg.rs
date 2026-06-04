///Register `DMACFG[%s]` reader
pub type R = crate::R<DmacfgSpec>;
///Register `DMACFG[%s]` writer
pub type W = crate::W<DmacfgSpec>;
///Field `DMASRCSEL` reader - This field selects one of the DMA requests as the DMA request output.
pub type DmasrcselR = crate::FieldReader;
///Field `DMASRCSEL` writer - This field selects one of the DMA requests as the DMA request output.
pub type DmasrcselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DMAMUX_EN` reader - No description available
pub type DmamuxEnR = crate::BitReader;
///Field `DMAMUX_EN` writer - No description available
pub type DmamuxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - This field selects one of the DMA requests as the DMA request output.
    #[inline(always)]
    pub fn dmasrcsel(&self) -> DmasrcselR {
        DmasrcselR::new((self.bits & 0x3f) as u8)
    }
    ///Bit 31 - No description available
    #[inline(always)]
    pub fn dmamux_en(&self) -> DmamuxEnR {
        DmamuxEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - This field selects one of the DMA requests as the DMA request output.
    #[inline(always)]
    pub fn dmasrcsel(&mut self) -> DmasrcselW<'_, DmacfgSpec> {
        DmasrcselW::new(self, 0)
    }
    ///Bit 31 - No description available
    #[inline(always)]
    pub fn dmamux_en(&mut self) -> DmamuxEnW<'_, DmacfgSpec> {
        DmamuxEnW::new(self, 31)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`dmacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacfgSpec;
impl crate::RegisterSpec for DmacfgSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacfg::R`](R) reader structure
impl crate::Readable for DmacfgSpec {}
///`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure
impl crate::Writable for DmacfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACFG[%s] to value 0
impl crate::Resettable for DmacfgSpec {}
