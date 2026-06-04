///Register `Status` reader
pub type R = crate::R<StatusSpec>;
///Register `Status` writer
pub type W = crate::W<StatusSpec>;
///Field `SPIACTIVE` reader - SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used.
pub type SpiactiveR = crate::BitReader;
///Field `RXNUM_5_0` reader - Number of valid entries in the Receive FIFO
pub type Rxnum5_0R = crate::FieldReader;
///Field `RXEMPTY` reader - Receive FIFO Empty flag
pub type RxemptyR = crate::BitReader;
///Field `RXFULL` reader - Receive FIFO Full flag
pub type RxfullR = crate::BitReader;
///Field `TXNUM_5_0` reader - Number of valid entries in the Transmit FIFO
pub type Txnum5_0R = crate::FieldReader;
///Field `TXEMPTY` reader - Transmit FIFO Empty flag
pub type TxemptyR = crate::BitReader;
///Field `TXFULL` reader - Transmit FIFO Full flag
pub type TxfullR = crate::BitReader;
///Field `RXNUM_7_6` reader - Number of valid entries in the Receive FIFO
pub type Rxnum7_6R = crate::FieldReader;
///Field `TXNUM_7_6` reader - Number of valid entries in the Transmit FIFO
pub type Txnum7_6R = crate::FieldReader;
impl R {
    ///Bit 0 - SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used.
    #[inline(always)]
    pub fn spiactive(&self) -> SpiactiveR {
        SpiactiveR::new((self.bits & 1) != 0)
    }
    ///Bits 8:13 - Number of valid entries in the Receive FIFO
    #[inline(always)]
    pub fn rxnum_5_0(&self) -> Rxnum5_0R {
        Rxnum5_0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 14 - Receive FIFO Empty flag
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive FIFO Full flag
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:21 - Number of valid entries in the Transmit FIFO
    #[inline(always)]
    pub fn txnum_5_0(&self) -> Txnum5_0R {
        Txnum5_0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bit 22 - Transmit FIFO Empty flag
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Transmit FIFO Full flag
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - Number of valid entries in the Receive FIFO
    #[inline(always)]
    pub fn rxnum_7_6(&self) -> Rxnum7_6R {
        Rxnum7_6R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - Number of valid entries in the Transmit FIFO
    #[inline(always)]
    pub fn txnum_7_6(&self) -> Txnum7_6R {
        Txnum7_6R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {}
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
///`reset()` method sets Status to value 0
impl crate::Resettable for StatusSpec {}
