///Register `IntrSt` reader
pub type R = crate::R<IntrStSpec>;
///Register `IntrSt` writer
pub type W = crate::W<IntrStSpec>;
///Field `RXFIFOORINT` writer - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)
pub type RxfifoorintW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOURINT` writer - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)
pub type TxfifourintW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFOINT` writer - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur.
pub type RxfifointW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOINT` writer - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur.
pub type TxfifointW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENDINT` writer - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur.
pub type EndintW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLVCMDINT` writer - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)
pub type SlvcmdintW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bit 0 - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)
    #[inline(always)]
    pub fn rxfifoorint(&mut self) -> RxfifoorintW<'_, IntrStSpec> {
        RxfifoorintW::new(self, 0)
    }
    ///Bit 1 - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)
    #[inline(always)]
    pub fn txfifourint(&mut self) -> TxfifourintW<'_, IntrStSpec> {
        TxfifourintW::new(self, 1)
    }
    ///Bit 2 - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur.
    #[inline(always)]
    pub fn rxfifoint(&mut self) -> RxfifointW<'_, IntrStSpec> {
        RxfifointW::new(self, 2)
    }
    ///Bit 3 - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur.
    #[inline(always)]
    pub fn txfifoint(&mut self) -> TxfifointW<'_, IntrStSpec> {
        TxfifointW::new(self, 3)
    }
    ///Bit 4 - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur.
    #[inline(always)]
    pub fn endint(&mut self) -> EndintW<'_, IntrStSpec> {
        EndintW::new(self, 4)
    }
    ///Bit 5 - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)
    #[inline(always)]
    pub fn slvcmdint(&mut self) -> SlvcmdintW<'_, IntrStSpec> {
        SlvcmdintW::new(self, 5)
    }
}
/**Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`intr_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntrStSpec;
impl crate::RegisterSpec for IntrStSpec {
    type Ux = u32;
}
///`read()` method returns [`intr_st::R`](R) reader structure
impl crate::Readable for IntrStSpec {}
///`write(|w| ..)` method takes [`intr_st::W`](W) writer structure
impl crate::Writable for IntrStSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IntrSt to value 0
impl crate::Resettable for IntrStSpec {}
