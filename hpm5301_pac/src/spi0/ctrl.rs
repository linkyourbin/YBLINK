///Register `Ctrl` reader
pub type R = crate::R<CtrlSpec>;
///Register `Ctrl` writer
pub type W = crate::W<CtrlSpec>;
///Field `SPIRST` reader - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
pub type SpirstR = crate::BitReader;
///Field `SPIRST` writer - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
pub type SpirstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFORST` reader - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
pub type RxfiforstR = crate::BitReader;
///Field `RXFIFORST` writer - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
pub type RxfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFORST` reader - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
pub type TxfiforstR = crate::BitReader;
///Field `TXFIFORST` writer - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
pub type TxfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDMAEN` reader - RX DMA enable
pub type RxdmaenR = crate::BitReader;
///Field `RXDMAEN` writer - RX DMA enable
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - TX DMA enable
pub type TxdmaenR = crate::BitReader;
///Field `TXDMAEN` writer - TX DMA enable
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXTHRES` reader - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold.
pub type RxthresR = crate::FieldReader;
///Field `RXTHRES` writer - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold.
pub type RxthresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TXTHRES` reader - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold.
pub type TxthresR = crate::FieldReader;
///Field `TXTHRES` writer - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold.
pub type TxthresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CS_EN` reader - No description available
pub type CsEnR = crate::FieldReader;
///Field `CS_EN` writer - No description available
pub type CsEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
    #[inline(always)]
    pub fn spirst(&self) -> SpirstR {
        SpirstR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
    #[inline(always)]
    pub fn rxfiforst(&self) -> RxfiforstR {
        RxfiforstR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
    #[inline(always)]
    pub fn txfiforst(&self) -> TxfiforstR {
        TxfiforstR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RX DMA enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX DMA enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:15 - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold.
    #[inline(always)]
    pub fn rxthres(&self) -> RxthresR {
        RxthresR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold.
    #[inline(always)]
    pub fn txthres(&self) -> TxthresR {
        TxthresR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - No description available
    #[inline(always)]
    pub fn cs_en(&self) -> CsEnR {
        CsEnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
    #[inline(always)]
    pub fn spirst(&mut self) -> SpirstW<'_, CtrlSpec> {
        SpirstW::new(self, 0)
    }
    ///Bit 1 - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
    #[inline(always)]
    pub fn rxfiforst(&mut self) -> RxfiforstW<'_, CtrlSpec> {
        RxfiforstW::new(self, 1)
    }
    ///Bit 2 - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes.
    #[inline(always)]
    pub fn txfiforst(&mut self) -> TxfiforstW<'_, CtrlSpec> {
        TxfiforstW::new(self, 2)
    }
    ///Bit 3 - RX DMA enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RxdmaenW<'_, CtrlSpec> {
        RxdmaenW::new(self, 3)
    }
    ///Bit 4 - TX DMA enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<'_, CtrlSpec> {
        TxdmaenW::new(self, 4)
    }
    ///Bits 8:15 - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold.
    #[inline(always)]
    pub fn rxthres(&mut self) -> RxthresW<'_, CtrlSpec> {
        RxthresW::new(self, 8)
    }
    ///Bits 16:23 - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold.
    #[inline(always)]
    pub fn txthres(&mut self) -> TxthresW<'_, CtrlSpec> {
        TxthresW::new(self, 16)
    }
    ///Bits 24:27 - No description available
    #[inline(always)]
    pub fn cs_en(&mut self) -> CsEnW<'_, CtrlSpec> {
        CsEnW::new(self, 24)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CtrlSpec {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Ctrl to value 0
impl crate::Resettable for CtrlSpec {}
