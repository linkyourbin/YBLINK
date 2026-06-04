///Register `Ctrl` reader
pub type R = crate::R<CtrlSpec>;
///Register `Ctrl` writer
pub type W = crate::W<CtrlSpec>;
///Field `DATACNT` reader - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
pub type DatacntR = crate::FieldReader;
///Field `DATACNT` writer - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
pub type DatacntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DIR` reader - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter
pub type DirR = crate::BitReader;
///Field `DIR` writer - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHASE_STOP` reader - Enable this bit to send a STOP condition at the end of a transaction. Master mode only.
pub type PhaseStopR = crate::BitReader;
///Field `PHASE_STOP` writer - Enable this bit to send a STOP condition at the end of a transaction. Master mode only.
pub type PhaseStopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHASE_DATA` reader - Enable this bit to send the data after Address phase. Master mode only.
pub type PhaseDataR = crate::BitReader;
///Field `PHASE_DATA` writer - Enable this bit to send the data after Address phase. Master mode only.
pub type PhaseDataW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHASE_ADDR` reader - Enable this bit to send the address after START condition. Master mode only.
pub type PhaseAddrR = crate::BitReader;
///Field `PHASE_ADDR` writer - Enable this bit to send the address after START condition. Master mode only.
pub type PhaseAddrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHASE_START` reader - Enable this bit to send a START condition at the beginning of transaction. Master mode only.
pub type PhaseStartR = crate::BitReader;
///Field `PHASE_START` writer - Enable this bit to send a START condition at the beginning of transaction. Master mode only.
pub type PhaseStartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET_ON` reader - set to send reset signals(just toggle clock bus defined by reset_len). this register is clered when reset is end, can't be cleared by software
pub type ResetOnR = crate::BitReader;
///Field `RESET_ON` writer - set to send reset signals(just toggle clock bus defined by reset_len). this register is clered when reset is end, can't be cleared by software
pub type ResetOnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET_HOLD_SCKIN` reader - set to hold input clock to high when reset is active
pub type ResetHoldSckinR = crate::BitReader;
///Field `RESET_HOLD_SCKIN` writer - set to hold input clock to high when reset is active
pub type ResetHoldSckinW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET_LEN` reader - reset clock cycles. the clock high/low time is defined by Setup.T_SCLHi, 50% duty cycle.
pub type ResetLenR = crate::FieldReader;
///Field `RESET_LEN` writer - reset clock cycles. the clock high/low time is defined by Setup.T_SCLHi, 50% duty cycle.
pub type ResetLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATACNT_HIGH` reader - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
pub type DatacntHighR = crate::FieldReader;
///Field `DATACNT_HIGH` writer - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
pub type DatacntHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
    #[inline(always)]
    pub fn datacnt(&self) -> DatacntR {
        DatacntR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable this bit to send a STOP condition at the end of a transaction. Master mode only.
    #[inline(always)]
    pub fn phase_stop(&self) -> PhaseStopR {
        PhaseStopR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable this bit to send the data after Address phase. Master mode only.
    #[inline(always)]
    pub fn phase_data(&self) -> PhaseDataR {
        PhaseDataR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable this bit to send the address after START condition. Master mode only.
    #[inline(always)]
    pub fn phase_addr(&self) -> PhaseAddrR {
        PhaseAddrR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable this bit to send a START condition at the beginning of transaction. Master mode only.
    #[inline(always)]
    pub fn phase_start(&self) -> PhaseStartR {
        PhaseStartR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - set to send reset signals(just toggle clock bus defined by reset_len). this register is clered when reset is end, can't be cleared by software
    #[inline(always)]
    pub fn reset_on(&self) -> ResetOnR {
        ResetOnR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - set to hold input clock to high when reset is active
    #[inline(always)]
    pub fn reset_hold_sckin(&self) -> ResetHoldSckinR {
        ResetHoldSckinR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 20:23 - reset clock cycles. the clock high/low time is defined by Setup.T_SCLHi, 50% duty cycle.
    #[inline(always)]
    pub fn reset_len(&self) -> ResetLenR {
        ResetLenR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:31 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
    #[inline(always)]
    pub fn datacnt_high(&self) -> DatacntHighR {
        DatacntHighR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
    #[inline(always)]
    pub fn datacnt(&mut self) -> DatacntW<'_, CtrlSpec> {
        DatacntW::new(self, 0)
    }
    ///Bit 8 - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, CtrlSpec> {
        DirW::new(self, 8)
    }
    ///Bit 9 - Enable this bit to send a STOP condition at the end of a transaction. Master mode only.
    #[inline(always)]
    pub fn phase_stop(&mut self) -> PhaseStopW<'_, CtrlSpec> {
        PhaseStopW::new(self, 9)
    }
    ///Bit 10 - Enable this bit to send the data after Address phase. Master mode only.
    #[inline(always)]
    pub fn phase_data(&mut self) -> PhaseDataW<'_, CtrlSpec> {
        PhaseDataW::new(self, 10)
    }
    ///Bit 11 - Enable this bit to send the address after START condition. Master mode only.
    #[inline(always)]
    pub fn phase_addr(&mut self) -> PhaseAddrW<'_, CtrlSpec> {
        PhaseAddrW::new(self, 11)
    }
    ///Bit 12 - Enable this bit to send a START condition at the beginning of transaction. Master mode only.
    #[inline(always)]
    pub fn phase_start(&mut self) -> PhaseStartW<'_, CtrlSpec> {
        PhaseStartW::new(self, 12)
    }
    ///Bit 13 - set to send reset signals(just toggle clock bus defined by reset_len). this register is clered when reset is end, can't be cleared by software
    #[inline(always)]
    pub fn reset_on(&mut self) -> ResetOnW<'_, CtrlSpec> {
        ResetOnW::new(self, 13)
    }
    ///Bit 14 - set to hold input clock to high when reset is active
    #[inline(always)]
    pub fn reset_hold_sckin(&mut self) -> ResetHoldSckinW<'_, CtrlSpec> {
        ResetHoldSckinW::new(self, 14)
    }
    ///Bits 20:23 - reset clock cycles. the clock high/low time is defined by Setup.T_SCLHi, 50% duty cycle.
    #[inline(always)]
    pub fn reset_len(&mut self) -> ResetLenW<'_, CtrlSpec> {
        ResetLenW::new(self, 20)
    }
    ///Bits 24:31 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.
    #[inline(always)]
    pub fn datacnt_high(&mut self) -> DatacntHighW<'_, CtrlSpec> {
        DatacntHighW::new(self, 24)
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
///`reset()` method sets Ctrl to value 0x0090_5e00
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0090_5e00;
}
