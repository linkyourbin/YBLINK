///Register `LSR` reader
pub type R = crate::R<LsrSpec>;
///Register `LSR` writer
pub type W = crate::W<LsrSpec>;
///Field `DR` reader - Data ready. This bit is set when there are incoming received data in the Receiver Buffer Register (RBR). It is cleared when all of the received data are read.
pub type DrR = crate::BitReader;
///Field `OE` reader - Overrun error This bit indicates that data in the Receiver Buffer Register (RBR) is overrun.
pub type OeR = crate::BitReader;
///Field `PE` reader - Parity error This bit is set when the received parity does not match with the parity selected in the LCR\[5:4\]. It is cleared when this register is read. In the FIFO mode, this bit indicates the parity error for the received data at the top of the RXFIFO.
pub type PeR = crate::BitReader;
///Field `FE` reader - Framing error This bit is set when the received STOP bit is not HIGH. It is cleared when this register is read. In the FIFO mode, this bit indicates the framing error for the received data at the top of the RXFIFO.
pub type FeR = crate::BitReader;
///Field `LBREAK` reader - Line break This bit is set when the uart_sin input signal was held LOWfor longer than the time for a full-word transmission. A full-word transmission is the transmission of the START, data, parity, and STOP bits. It is cleared when this register is read. In the FIFO mode, this bit indicates the line break for the received data at the top of the RXFIFO.
pub type LbreakR = crate::BitReader;
///Field `THRE` reader - Transmitter Holding Register empty This bit is 1 when the THR (TXFIFO in the FIFO mode) is empty. Otherwise, it is zero. If the THRE interrupt is enabled, an interrupt is triggered when THRE becomes 1.
pub type ThreR = crate::BitReader;
///Field `TEMT` reader - Transmitter empty This bit is 1 when the THR (TXFIFO in the FIFO mode) and the Transmitter Shift Register (TSR) are both empty. Otherwise, it is zero.
pub type TemtR = crate::BitReader;
///Field `ERRF` reader - Error in RXFIFO In the FIFO mode, this bit is set when there is at least one parity error, framing error, or line break associated with data in the RXFIFO. It is cleared when this register is read and there is no more error for the rest of data in the RXFIFO.
pub type ErrfR = crate::BitReader;
///Field `TFIFO_NUM` reader - data bytes in txfifo not sent
pub type TfifoNumR = crate::FieldReader;
///Field `RFIFO_NUM` reader - data bytes in rxfifo not read
pub type RfifoNumR = crate::FieldReader;
///Field `TXIDLE` reader - txidle after timeout, clear after tx idle condition not match
pub type TxidleR = crate::BitReader;
///Field `RXIDLE` reader - rxidle after timeout, clear after rx idle condition not match
pub type RxidleR = crate::BitReader;
impl R {
    ///Bit 0 - Data ready. This bit is set when there are incoming received data in the Receiver Buffer Register (RBR). It is cleared when all of the received data are read.
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overrun error This bit indicates that data in the Receiver Buffer Register (RBR) is overrun.
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Parity error This bit is set when the received parity does not match with the parity selected in the LCR\[5:4\]. It is cleared when this register is read. In the FIFO mode, this bit indicates the parity error for the received data at the top of the RXFIFO.
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Framing error This bit is set when the received STOP bit is not HIGH. It is cleared when this register is read. In the FIFO mode, this bit indicates the framing error for the received data at the top of the RXFIFO.
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Line break This bit is set when the uart_sin input signal was held LOWfor longer than the time for a full-word transmission. A full-word transmission is the transmission of the START, data, parity, and STOP bits. It is cleared when this register is read. In the FIFO mode, this bit indicates the line break for the received data at the top of the RXFIFO.
    #[inline(always)]
    pub fn lbreak(&self) -> LbreakR {
        LbreakR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmitter Holding Register empty This bit is 1 when the THR (TXFIFO in the FIFO mode) is empty. Otherwise, it is zero. If the THRE interrupt is enabled, an interrupt is triggered when THRE becomes 1.
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmitter empty This bit is 1 when the THR (TXFIFO in the FIFO mode) and the Transmitter Shift Register (TSR) are both empty. Otherwise, it is zero.
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Error in RXFIFO In the FIFO mode, this bit is set when there is at least one parity error, framing error, or line break associated with data in the RXFIFO. It is cleared when this register is read and there is no more error for the rest of data in the RXFIFO.
    #[inline(always)]
    pub fn errf(&self) -> ErrfR {
        ErrfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - data bytes in txfifo not sent
    #[inline(always)]
    pub fn tfifo_num(&self) -> TfifoNumR {
        TfifoNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - data bytes in rxfifo not read
    #[inline(always)]
    pub fn rfifo_num(&self) -> RfifoNumR {
        RfifoNumR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 30 - txidle after timeout, clear after tx idle condition not match
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - rxidle after timeout, clear after rx idle condition not match
    #[inline(always)]
    pub fn rxidle(&self) -> RxidleR {
        RxidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
/**Line Status Register

You can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
///`read()` method returns [`lsr::R`](R) reader structure
impl crate::Readable for LsrSpec {}
///`write(|w| ..)` method takes [`lsr::W`](W) writer structure
impl crate::Writable for LsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LSR to value 0
impl crate::Resettable for LsrSpec {}
