///Register `FCR` reader
pub type R = crate::R<Union28FcrSpec>;
///Register `FCR` writer
pub type W = crate::W<Union28FcrSpec>;
///Field `FIFOE` writer - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles.
pub type FifoeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIFORST` writer - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared.
pub type RfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFIFORST` writer - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared.
pub type TfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAE` writer - DMA enable 0: Disable 1: Enable
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFIFOT` writer - Transmitter FIFO trigger level(0 for 1byte, 0x3 for 4bytes), uart will send tx_dma_req when data in fifo is less than threshold.
pub type TfifotW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RFIFOT` writer - Receiver FIFO trigger level(0 for 1byte, 0x3 for 4bytes). Uart will send rx_dma_req if data in fifo reachs the threshold
pub type RfifotW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    ///Bit 0 - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles.
    #[inline(always)]
    pub fn fifoe(&mut self) -> FifoeW<'_, Union28FcrSpec> {
        FifoeW::new(self, 0)
    }
    ///Bit 1 - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared.
    #[inline(always)]
    pub fn rfiforst(&mut self) -> RfiforstW<'_, Union28FcrSpec> {
        RfiforstW::new(self, 1)
    }
    ///Bit 2 - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared.
    #[inline(always)]
    pub fn tfiforst(&mut self) -> TfiforstW<'_, Union28FcrSpec> {
        TfiforstW::new(self, 2)
    }
    ///Bit 3 - DMA enable 0: Disable 1: Enable
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<'_, Union28FcrSpec> {
        DmaeW::new(self, 3)
    }
    ///Bits 4:5 - Transmitter FIFO trigger level(0 for 1byte, 0x3 for 4bytes), uart will send tx_dma_req when data in fifo is less than threshold.
    #[inline(always)]
    pub fn tfifot(&mut self) -> TfifotW<'_, Union28FcrSpec> {
        TfifotW::new(self, 4)
    }
    ///Bits 6:7 - Receiver FIFO trigger level(0 for 1byte, 0x3 for 4bytes). Uart will send rx_dma_req if data in fifo reachs the threshold
    #[inline(always)]
    pub fn rfifot(&mut self) -> RfifotW<'_, Union28FcrSpec> {
        RfifotW::new(self, 6)
    }
}
/**FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`union_28_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_28_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union28FcrSpec;
impl crate::RegisterSpec for Union28FcrSpec {
    type Ux = u32;
}
///`read()` method returns [`union_28_fcr::R`](R) reader structure
impl crate::Readable for Union28FcrSpec {}
///`write(|w| ..)` method takes [`union_28_fcr::W`](W) writer structure
impl crate::Writable for Union28FcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for Union28FcrSpec {}
