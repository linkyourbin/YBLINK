///Register `BURSTSIZE` reader
pub type R = crate::R<BurstsizeSpec>;
///Register `BURSTSIZE` writer
pub type W = crate::W<BurstsizeSpec>;
///Field `RXPBURST` reader - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory.
pub type RxpburstR = crate::FieldReader;
///Field `RXPBURST` writer - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory.
pub type RxpburstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TXPBURST` reader - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus.
pub type TxpburstR = crate::FieldReader;
///Field `TXPBURST` writer - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus.
pub type TxpburstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory.
    #[inline(always)]
    pub fn rxpburst(&self) -> RxpburstR {
        RxpburstR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus.
    #[inline(always)]
    pub fn txpburst(&self) -> TxpburstR {
        TxpburstR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory.
    #[inline(always)]
    pub fn rxpburst(&mut self) -> RxpburstW<'_, BurstsizeSpec> {
        RxpburstW::new(self, 0)
    }
    ///Bits 8:15 - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus.
    #[inline(always)]
    pub fn txpburst(&mut self) -> TxpburstW<'_, BurstsizeSpec> {
        TxpburstW::new(self, 8)
    }
}
/**Programmable Burst Size Register

You can [`read`](crate::Reg::read) this register and get [`burstsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`burstsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BurstsizeSpec;
impl crate::RegisterSpec for BurstsizeSpec {
    type Ux = u32;
}
///`read()` method returns [`burstsize::R`](R) reader structure
impl crate::Readable for BurstsizeSpec {}
///`write(|w| ..)` method takes [`burstsize::W`](W) writer structure
impl crate::Writable for BurstsizeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BURSTSIZE to value 0
impl crate::Resettable for BurstsizeSpec {}
