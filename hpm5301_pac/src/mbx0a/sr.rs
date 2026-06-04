///Register `SR` reader
pub type R = crate::R<SrSpec>;
///Register `SR` writer
pub type W = crate::W<SrSpec>;
///Field `RWMV` reader - RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG.
pub type RwmvR = crate::BitReader;
///Field `TWME` reader - TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet.
pub type TwmeR = crate::BitReader;
///Field `RFMF` reader - RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet.
pub type RfmfR = crate::BitReader;
///Field `RFMA` reader - RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already.
pub type RfmaR = crate::BitReader;
///Field `TFME` reader - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet.
pub type TfmeR = crate::BitReader;
///Field `TFME` writer - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet.
pub type TfmeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFMA` reader - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)
pub type TfmaR = crate::BitReader;
///Field `TFMA` writer - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)
pub type TfmaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EW2RO` writer - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
pub type Ew2roW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EAIVA` writer - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
pub type EaivaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWTFF` writer - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
pub type EwtffW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRFE` writer - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
pub type ErrfeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWTRF` writer - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
pub type EwtrfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRRE` writer - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
pub type ErrreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFEC` reader - TX FIFO empty message word count
pub type TfecR = crate::FieldReader;
///Field `RFVC` reader - RX FIFO valid message count
pub type RfvcR = crate::FieldReader;
impl R {
    ///Bit 0 - RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG.
    #[inline(always)]
    pub fn rwmv(&self) -> RwmvR {
        RwmvR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet.
    #[inline(always)]
    pub fn twme(&self) -> TwmeR {
        TwmeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet.
    #[inline(always)]
    pub fn rfmf(&self) -> RfmfR {
        RfmfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already.
    #[inline(always)]
    pub fn rfma(&self) -> RfmaR {
        RfmaR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet.
    #[inline(always)]
    pub fn tfme(&self) -> TfmeR {
        TfmeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)
    #[inline(always)]
    pub fn tfma(&self) -> TfmaR {
        TfmaR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:19 - TX FIFO empty message word count
    #[inline(always)]
    pub fn tfec(&self) -> TfecR {
        TfecR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - RX FIFO valid message count
    #[inline(always)]
    pub fn rfvc(&self) -> RfvcR {
        RfvcR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 6 - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet.
    #[inline(always)]
    pub fn tfme(&mut self) -> TfmeW<'_, SrSpec> {
        TfmeW::new(self, 6)
    }
    ///Bit 7 - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)
    #[inline(always)]
    pub fn tfma(&mut self) -> TfmaW<'_, SrSpec> {
        TfmaW::new(self, 7)
    }
    ///Bit 8 - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
    #[inline(always)]
    pub fn ew2ro(&mut self) -> Ew2roW<'_, SrSpec> {
        Ew2roW::new(self, 8)
    }
    ///Bit 9 - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
    #[inline(always)]
    pub fn eaiva(&mut self) -> EaivaW<'_, SrSpec> {
        EaivaW::new(self, 9)
    }
    ///Bit 10 - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
    #[inline(always)]
    pub fn ewtff(&mut self) -> EwtffW<'_, SrSpec> {
        EwtffW::new(self, 10)
    }
    ///Bit 11 - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
    #[inline(always)]
    pub fn errfe(&mut self) -> ErrfeW<'_, SrSpec> {
        ErrfeW::new(self, 11)
    }
    ///Bit 12 - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
    #[inline(always)]
    pub fn ewtrf(&mut self) -> EwtrfW<'_, SrSpec> {
        EwtrfW::new(self, 12)
    }
    ///Bit 13 - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen.
    #[inline(always)]
    pub fn errre(&mut self) -> ErrreW<'_, SrSpec> {
        ErrreW::new(self, 13)
    }
}
/**Status Registers

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SrSpec {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0xe2
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xe2;
}
