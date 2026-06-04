///Register `IntEn` reader
pub type R = crate::R<IntEnSpec>;
///Register `IntEn` writer
pub type W = crate::W<IntEnSpec>;
///Field `FIFOEMPTY` reader - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty.
pub type FifoemptyR = crate::BitReader;
///Field `FIFOEMPTY` writer - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty.
pub type FifoemptyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFOFULL` reader - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full.
pub type FifofullR = crate::BitReader;
///Field `FIFOFULL` writer - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full.
pub type FifofullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFOHALF` reader - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered.
pub type FifohalfR = crate::BitReader;
///Field `FIFOHALF` writer - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered.
pub type FifohalfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRHIT` reader - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed.
pub type AddrhitR = crate::BitReader;
///Field `ADDRHIT` writer - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed.
pub type AddrhitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLOSE` reader - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode.
pub type ArbloseR = crate::BitReader;
///Field `ARBLOSE` writer - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode.
pub type ArbloseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected.
pub type StopR = crate::BitReader;
///Field `STOP` writer - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected.
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected.
pub type StartR = crate::BitReader;
///Field `START` writer - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected.
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTETRANS` reader - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted.
pub type BytetransR = crate::BitReader;
///Field `BYTETRANS` writer - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted.
pub type BytetransW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTERECV` reader - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually.
pub type ByterecvR = crate::BitReader;
///Field `BYTERECV` writer - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually.
pub type ByterecvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPL` reader - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed.
pub type CmplR = crate::BitReader;
///Field `CMPL` writer - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed.
pub type CmplW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty.
    #[inline(always)]
    pub fn fifoempty(&self) -> FifoemptyR {
        FifoemptyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full.
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered.
    #[inline(always)]
    pub fn fifohalf(&self) -> FifohalfR {
        FifohalfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed.
    #[inline(always)]
    pub fn addrhit(&self) -> AddrhitR {
        AddrhitR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode.
    #[inline(always)]
    pub fn arblose(&self) -> ArbloseR {
        ArbloseR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected.
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected.
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted.
    #[inline(always)]
    pub fn bytetrans(&self) -> BytetransR {
        BytetransR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually.
    #[inline(always)]
    pub fn byterecv(&self) -> ByterecvR {
        ByterecvR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed.
    #[inline(always)]
    pub fn cmpl(&self) -> CmplR {
        CmplR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty.
    #[inline(always)]
    pub fn fifoempty(&mut self) -> FifoemptyW<'_, IntEnSpec> {
        FifoemptyW::new(self, 0)
    }
    ///Bit 1 - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full.
    #[inline(always)]
    pub fn fifofull(&mut self) -> FifofullW<'_, IntEnSpec> {
        FifofullW::new(self, 1)
    }
    ///Bit 2 - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered.
    #[inline(always)]
    pub fn fifohalf(&mut self) -> FifohalfW<'_, IntEnSpec> {
        FifohalfW::new(self, 2)
    }
    ///Bit 3 - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed.
    #[inline(always)]
    pub fn addrhit(&mut self) -> AddrhitW<'_, IntEnSpec> {
        AddrhitW::new(self, 3)
    }
    ///Bit 4 - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode.
    #[inline(always)]
    pub fn arblose(&mut self) -> ArbloseW<'_, IntEnSpec> {
        ArbloseW::new(self, 4)
    }
    ///Bit 5 - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected.
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, IntEnSpec> {
        StopW::new(self, 5)
    }
    ///Bit 6 - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected.
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IntEnSpec> {
        StartW::new(self, 6)
    }
    ///Bit 7 - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted.
    #[inline(always)]
    pub fn bytetrans(&mut self) -> BytetransW<'_, IntEnSpec> {
        BytetransW::new(self, 7)
    }
    ///Bit 8 - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually.
    #[inline(always)]
    pub fn byterecv(&mut self) -> ByterecvW<'_, IntEnSpec> {
        ByterecvW::new(self, 8)
    }
    ///Bit 9 - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed.
    #[inline(always)]
    pub fn cmpl(&mut self) -> CmplW<'_, IntEnSpec> {
        CmplW::new(self, 9)
    }
}
/**Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u32;
}
///`read()` method returns [`int_en::R`](R) reader structure
impl crate::Readable for IntEnSpec {}
///`write(|w| ..)` method takes [`int_en::W`](W) writer structure
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IntEn to value 0
impl crate::Resettable for IntEnSpec {}
