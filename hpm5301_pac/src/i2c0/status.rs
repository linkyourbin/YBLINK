///Register `Status` reader
pub type R = crate::R<StatusSpec>;
///Register `Status` writer
pub type W = crate::W<StatusSpec>;
///Field `FIFOEMPTY` reader - Indicates that the FIFO is empty.
pub type FifoemptyR = crate::BitReader;
///Field `FIFOFULL` reader - Indicates that the FIFO is full.
pub type FifofullR = crate::BitReader;
///Field `FIFOHALF` reader - Transmitter: Indicates that the FIFO is half-empty.
pub type FifohalfR = crate::BitReader;
///Field `ADDRHIT` writer - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call).
pub type AddrhitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLOSE` writer - Indicates that the controller has lost the bus arbitration.
pub type ArbloseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` writer - Indicates that a STOP Condition has been transmitted/received.
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` writer - Indicates that a START Condition or a repeated START condition has been transmitted/received.
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTETRANS` writer - Indicates that a byte of data has been transmitted.
pub type BytetransW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTERECV` writer - Indicates that a byte of data has been received.
pub type ByterecvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPL` writer - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked.
pub type CmplW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` reader - Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK
pub type AckR = crate::BitReader;
///Field `BUSBUSY` reader - Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy
pub type BusbusyR = crate::BitReader;
///Field `GENCALL` reader - Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call
pub type GencallR = crate::BitReader;
///Field `LINESCL` reader - Indicates the current status of the SCL line on the bus 1: high 0: low
pub type LinesclR = crate::BitReader;
///Field `LINESDA` reader - Indicates the current status of the SDA line on the bus 1: high 0: low
pub type LinesdaR = crate::BitReader;
impl R {
    ///Bit 0 - Indicates that the FIFO is empty.
    #[inline(always)]
    pub fn fifoempty(&self) -> FifoemptyR {
        FifoemptyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Indicates that the FIFO is full.
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmitter: Indicates that the FIFO is half-empty.
    #[inline(always)]
    pub fn fifohalf(&self) -> FifohalfR {
        FifohalfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy
    #[inline(always)]
    pub fn busbusy(&self) -> BusbusyR {
        BusbusyR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call
    #[inline(always)]
    pub fn gencall(&self) -> GencallR {
        GencallR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Indicates the current status of the SCL line on the bus 1: high 0: low
    #[inline(always)]
    pub fn linescl(&self) -> LinesclR {
        LinesclR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Indicates the current status of the SDA line on the bus 1: high 0: low
    #[inline(always)]
    pub fn linesda(&self) -> LinesdaR {
        LinesdaR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call).
    #[inline(always)]
    pub fn addrhit(&mut self) -> AddrhitW<'_, StatusSpec> {
        AddrhitW::new(self, 3)
    }
    ///Bit 4 - Indicates that the controller has lost the bus arbitration.
    #[inline(always)]
    pub fn arblose(&mut self) -> ArbloseW<'_, StatusSpec> {
        ArbloseW::new(self, 4)
    }
    ///Bit 5 - Indicates that a STOP Condition has been transmitted/received.
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, StatusSpec> {
        StopW::new(self, 5)
    }
    ///Bit 6 - Indicates that a START Condition or a repeated START condition has been transmitted/received.
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, StatusSpec> {
        StartW::new(self, 6)
    }
    ///Bit 7 - Indicates that a byte of data has been transmitted.
    #[inline(always)]
    pub fn bytetrans(&mut self) -> BytetransW<'_, StatusSpec> {
        BytetransW::new(self, 7)
    }
    ///Bit 8 - Indicates that a byte of data has been received.
    #[inline(always)]
    pub fn byterecv(&mut self) -> ByterecvW<'_, StatusSpec> {
        ByterecvW::new(self, 8)
    }
    ///Bit 9 - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked.
    #[inline(always)]
    pub fn cmpl(&mut self) -> CmplW<'_, StatusSpec> {
        CmplW::new(self, 9)
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for StatusSpec {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Status to value 0x01
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
