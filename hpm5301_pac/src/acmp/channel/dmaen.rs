///Register `dmaen` reader
pub type R = crate::R<DmaenSpec>;
///Register `dmaen` writer
pub type W = crate::W<DmaenSpec>;
///Field `REDGEN` reader - Output rising edge flag DMA request enable bit.
pub type RedgenR = crate::BitReader;
///Field `REDGEN` writer - Output rising edge flag DMA request enable bit.
pub type RedgenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEDGEN` reader - Output falling edge flag DMA request enable bit.
pub type FedgenR = crate::BitReader;
///Field `FEDGEN` writer - Output falling edge flag DMA request enable bit.
pub type FedgenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Output rising edge flag DMA request enable bit.
    #[inline(always)]
    pub fn redgen(&self) -> RedgenR {
        RedgenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output falling edge flag DMA request enable bit.
    #[inline(always)]
    pub fn fedgen(&self) -> FedgenR {
        FedgenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Output rising edge flag DMA request enable bit.
    #[inline(always)]
    pub fn redgen(&mut self) -> RedgenW<'_, DmaenSpec> {
        RedgenW::new(self, 0)
    }
    ///Bit 1 - Output falling edge flag DMA request enable bit.
    #[inline(always)]
    pub fn fedgen(&mut self) -> FedgenW<'_, DmaenSpec> {
        FedgenW::new(self, 1)
    }
}
/**DMA request enable register

You can [`read`](crate::Reg::read) this register and get [`dmaen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmaenSpec;
impl crate::RegisterSpec for DmaenSpec {
    type Ux = u32;
}
///`read()` method returns [`dmaen::R`](R) reader structure
impl crate::Readable for DmaenSpec {}
///`write(|w| ..)` method takes [`dmaen::W`](W) writer structure
impl crate::Writable for DmaenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets dmaen to value 0
impl crate::Resettable for DmaenSpec {}
