///Register `PERIODICLISTBASE` reader
pub type R = crate::R<Union154PeriodiclistbaseSpec>;
///Register `PERIODICLISTBASE` writer
pub type W = crate::W<Union154PeriodiclistbaseSpec>;
///Field `BASEADR` reader - BASEADR Base Address (Low). These bits correspond to memory address signals \[31:12\], respectively. Only used by the host controller.
pub type BaseadrR = crate::FieldReader<u32>;
///Field `BASEADR` writer - BASEADR Base Address (Low). These bits correspond to memory address signals \[31:12\], respectively. Only used by the host controller.
pub type BaseadrW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 12:31 - BASEADR Base Address (Low). These bits correspond to memory address signals \[31:12\], respectively. Only used by the host controller.
    #[inline(always)]
    pub fn baseadr(&self) -> BaseadrR {
        BaseadrR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    ///Bits 12:31 - BASEADR Base Address (Low). These bits correspond to memory address signals \[31:12\], respectively. Only used by the host controller.
    #[inline(always)]
    pub fn baseadr(&mut self) -> BaseadrW<'_, Union154PeriodiclistbaseSpec> {
        BaseadrW::new(self, 12)
    }
}
/**Frame List Base Address Register

You can [`read`](crate::Reg::read) this register and get [`union_154_periodiclistbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_154_periodiclistbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union154PeriodiclistbaseSpec;
impl crate::RegisterSpec for Union154PeriodiclistbaseSpec {
    type Ux = u32;
}
///`read()` method returns [`union_154_periodiclistbase::R`](R) reader structure
impl crate::Readable for Union154PeriodiclistbaseSpec {}
///`write(|w| ..)` method takes [`union_154_periodiclistbase::W`](W) writer structure
impl crate::Writable for Union154PeriodiclistbaseSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PERIODICLISTBASE to value 0
impl crate::Resettable for Union154PeriodiclistbaseSpec {}
