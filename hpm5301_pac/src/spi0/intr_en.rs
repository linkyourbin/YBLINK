///Register `IntrEn` reader
pub type R = crate::R<IntrEnSpec>;
///Register `IntrEn` writer
pub type W = crate::W<IntrEnSpec>;
///Field `RXFIFOORINTEN` reader - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)
pub type RxfifoorintenR = crate::BitReader;
///Field `RXFIFOORINTEN` writer - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)
pub type RxfifoorintenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOURINTEN` reader - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)
pub type TxfifourintenR = crate::BitReader;
///Field `TXFIFOURINTEN` writer - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)
pub type TxfifourintenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFOINTEN` reader - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold.
pub type RxfifointenR = crate::BitReader;
///Field `RXFIFOINTEN` writer - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold.
pub type RxfifointenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOINTEN` reader - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold.
pub type TxfifointenR = crate::BitReader;
///Field `TXFIFOINTEN` writer - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold.
pub type TxfifointenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENDINTEN` reader - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)
pub type EndintenR = crate::BitReader;
///Field `ENDINTEN` writer - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)
pub type EndintenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLVCMDEN` reader - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)
pub type SlvcmdenR = crate::BitReader;
///Field `SLVCMDEN` writer - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)
pub type SlvcmdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)
    #[inline(always)]
    pub fn rxfifoorinten(&self) -> RxfifoorintenR {
        RxfifoorintenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)
    #[inline(always)]
    pub fn txfifourinten(&self) -> TxfifourintenR {
        TxfifourintenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold.
    #[inline(always)]
    pub fn rxfifointen(&self) -> RxfifointenR {
        RxfifointenR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold.
    #[inline(always)]
    pub fn txfifointen(&self) -> TxfifointenR {
        TxfifointenR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)
    #[inline(always)]
    pub fn endinten(&self) -> EndintenR {
        EndintenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)
    #[inline(always)]
    pub fn slvcmden(&self) -> SlvcmdenR {
        SlvcmdenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)
    #[inline(always)]
    pub fn rxfifoorinten(&mut self) -> RxfifoorintenW<'_, IntrEnSpec> {
        RxfifoorintenW::new(self, 0)
    }
    ///Bit 1 - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)
    #[inline(always)]
    pub fn txfifourinten(&mut self) -> TxfifourintenW<'_, IntrEnSpec> {
        TxfifourintenW::new(self, 1)
    }
    ///Bit 2 - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold.
    #[inline(always)]
    pub fn rxfifointen(&mut self) -> RxfifointenW<'_, IntrEnSpec> {
        RxfifointenW::new(self, 2)
    }
    ///Bit 3 - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold.
    #[inline(always)]
    pub fn txfifointen(&mut self) -> TxfifointenW<'_, IntrEnSpec> {
        TxfifointenW::new(self, 3)
    }
    ///Bit 4 - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)
    #[inline(always)]
    pub fn endinten(&mut self) -> EndintenW<'_, IntrEnSpec> {
        EndintenW::new(self, 4)
    }
    ///Bit 5 - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)
    #[inline(always)]
    pub fn slvcmden(&mut self) -> SlvcmdenW<'_, IntrEnSpec> {
        SlvcmdenW::new(self, 5)
    }
}
/**Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`intr_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntrEnSpec;
impl crate::RegisterSpec for IntrEnSpec {
    type Ux = u32;
}
///`read()` method returns [`intr_en::R`](R) reader structure
impl crate::Readable for IntrEnSpec {}
///`write(|w| ..)` method takes [`intr_en::W`](W) writer structure
impl crate::Writable for IntrEnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IntrEn to value 0
impl crate::Resettable for IntrEnSpec {}
