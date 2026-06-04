///Register `FCRR` reader
pub type R = crate::R<FcrrSpec>;
///Register `FCRR` writer
pub type W = crate::W<FcrrSpec>;
///Field `FIFOE` reader - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles.
pub type FifoeR = crate::BitReader;
///Field `FIFOE` writer - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles.
pub type FifoeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIFORST` writer - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared.
pub type RfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFIFORST` writer - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared.
pub type TfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAE` reader - DMA enable 0: Disable 1: Enable
pub type DmaeR = crate::BitReader;
///Field `DMAE` writer - DMA enable 0: Disable 1: Enable
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFIFOT` reader - Transmitter FIFO trigger level
pub type TfifotR = crate::FieldReader;
///Field `TFIFOT` writer - Transmitter FIFO trigger level
pub type TfifotW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RFIFOT` reader - Receiver FIFO trigger level
pub type RfifotR = crate::FieldReader;
///Field `RFIFOT` writer - Receiver FIFO trigger level
pub type RfifotW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RFIFOT4` reader - rxfifo threshold(0 for 1byte, 0xF for 32bytes). Uart will send rx_dma_req if data in fifo reachs the threshold, also will set the rxdata irq if enabled
pub type Rfifot4R = crate::FieldReader;
///Field `RFIFOT4` writer - rxfifo threshold(0 for 1byte, 0xF for 32bytes). Uart will send rx_dma_req if data in fifo reachs the threshold, also will set the rxdata irq if enabled
pub type Rfifot4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TFIFOT4` reader - txfifo threshold(0 for 1byte, 0xF for 32bytes), uart will send tx_dma_req when data in fifo is less than threshold.
pub type Tfifot4R = crate::FieldReader;
///Field `TFIFOT4` writer - txfifo threshold(0 for 1byte, 0xF for 32bytes), uart will send tx_dma_req when data in fifo is less than threshold.
pub type Tfifot4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `FIFOT4EN` reader - set to use new 4bit fifo threshold(TFIFOT4 and RFIFOT4) clr to use 2bit(TFIFOT and RFIFOT)
pub type Fifot4enR = crate::BitReader;
///Field `FIFOT4EN` writer - set to use new 4bit fifo threshold(TFIFOT4 and RFIFOT4) clr to use 2bit(TFIFOT and RFIFOT)
pub type Fifot4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles.
    #[inline(always)]
    pub fn fifoe(&self) -> FifoeR {
        FifoeR::new((self.bits & 1) != 0)
    }
    ///Bit 3 - DMA enable 0: Disable 1: Enable
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Transmitter FIFO trigger level
    #[inline(always)]
    pub fn tfifot(&self) -> TfifotR {
        TfifotR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Receiver FIFO trigger level
    #[inline(always)]
    pub fn rfifot(&self) -> RfifotR {
        RfifotR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:12 - rxfifo threshold(0 for 1byte, 0xF for 32bytes). Uart will send rx_dma_req if data in fifo reachs the threshold, also will set the rxdata irq if enabled
    #[inline(always)]
    pub fn rfifot4(&self) -> Rfifot4R {
        Rfifot4R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - txfifo threshold(0 for 1byte, 0xF for 32bytes), uart will send tx_dma_req when data in fifo is less than threshold.
    #[inline(always)]
    pub fn tfifot4(&self) -> Tfifot4R {
        Tfifot4R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - set to use new 4bit fifo threshold(TFIFOT4 and RFIFOT4) clr to use 2bit(TFIFOT and RFIFOT)
    #[inline(always)]
    pub fn fifot4en(&self) -> Fifot4enR {
        Fifot4enR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles.
    #[inline(always)]
    pub fn fifoe(&mut self) -> FifoeW<'_, FcrrSpec> {
        FifoeW::new(self, 0)
    }
    ///Bit 1 - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared.
    #[inline(always)]
    pub fn rfiforst(&mut self) -> RfiforstW<'_, FcrrSpec> {
        RfiforstW::new(self, 1)
    }
    ///Bit 2 - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared.
    #[inline(always)]
    pub fn tfiforst(&mut self) -> TfiforstW<'_, FcrrSpec> {
        TfiforstW::new(self, 2)
    }
    ///Bit 3 - DMA enable 0: Disable 1: Enable
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<'_, FcrrSpec> {
        DmaeW::new(self, 3)
    }
    ///Bits 4:5 - Transmitter FIFO trigger level
    #[inline(always)]
    pub fn tfifot(&mut self) -> TfifotW<'_, FcrrSpec> {
        TfifotW::new(self, 4)
    }
    ///Bits 6:7 - Receiver FIFO trigger level
    #[inline(always)]
    pub fn rfifot(&mut self) -> RfifotW<'_, FcrrSpec> {
        RfifotW::new(self, 6)
    }
    ///Bits 8:12 - rxfifo threshold(0 for 1byte, 0xF for 32bytes). Uart will send rx_dma_req if data in fifo reachs the threshold, also will set the rxdata irq if enabled
    #[inline(always)]
    pub fn rfifot4(&mut self) -> Rfifot4W<'_, FcrrSpec> {
        Rfifot4W::new(self, 8)
    }
    ///Bits 16:20 - txfifo threshold(0 for 1byte, 0xF for 32bytes), uart will send tx_dma_req when data in fifo is less than threshold.
    #[inline(always)]
    pub fn tfifot4(&mut self) -> Tfifot4W<'_, FcrrSpec> {
        Tfifot4W::new(self, 16)
    }
    ///Bit 23 - set to use new 4bit fifo threshold(TFIFOT4 and RFIFOT4) clr to use 2bit(TFIFOT and RFIFOT)
    #[inline(always)]
    pub fn fifot4en(&mut self) -> Fifot4enW<'_, FcrrSpec> {
        Fifot4enW::new(self, 23)
    }
}
/**FIFO Control Register config

You can [`read`](crate::Reg::read) this register and get [`fcrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FcrrSpec;
impl crate::RegisterSpec for FcrrSpec {
    type Ux = u32;
}
///`read()` method returns [`fcrr::R`](R) reader structure
impl crate::Readable for FcrrSpec {}
///`write(|w| ..)` method takes [`fcrr::W`](W) writer structure
impl crate::Writable for FcrrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCRR to value 0
impl crate::Resettable for FcrrSpec {}
