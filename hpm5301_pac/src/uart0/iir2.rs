///Register `IIR2` reader
pub type R = crate::R<Iir2Spec>;
///Register `IIR2` writer
pub type W = crate::W<Iir2Spec>;
///Field `INTRID` reader - Interrupt ID, see IIR2 for detail decoding
pub type IntridR = crate::FieldReader;
///Field `FIFOED` reader - FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1.
pub type FifoedR = crate::FieldReader;
///Field `DATA_LOST` writer - assert if data lost before address match status, write one clear; It will not assert if no address match occurs
pub type DataLostW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDR_MATCH_IDLE` writer - address match and idle irq status, assert at rx bus idle if address match event triggered. Write one clear;
pub type AddrMatchIdleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDR_MATCH` writer - address match irq status, assert if either address match(and enabled). Write one clear NOTE: the address byte may not moved by DMA at this point. User can wait next addr_match_idle irq for the whole data include address
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXIDLE_FLAG` writer - UART TX IDLE Flag, assert after txd high and then tx idle timeout, write one clear 0 - UART TX is busy 1 - UART TX is idle
pub type TxidleFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXIDLE_FLAG` writer - UART RX IDLE Flag, assert after rxd high and then rx idle timeout, write one clear 0 - UART RX is busy 1 - UART RX is idle
pub type RxidleFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Interrupt ID, see IIR2 for detail decoding
    #[inline(always)]
    pub fn intrid(&self) -> IntridR {
        IntridR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:7 - FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1.
    #[inline(always)]
    pub fn fifoed(&self) -> FifoedR {
        FifoedR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    ///Bit 27 - assert if data lost before address match status, write one clear; It will not assert if no address match occurs
    #[inline(always)]
    pub fn data_lost(&mut self) -> DataLostW<'_, Iir2Spec> {
        DataLostW::new(self, 27)
    }
    ///Bit 28 - address match and idle irq status, assert at rx bus idle if address match event triggered. Write one clear;
    #[inline(always)]
    pub fn addr_match_idle(&mut self) -> AddrMatchIdleW<'_, Iir2Spec> {
        AddrMatchIdleW::new(self, 28)
    }
    ///Bit 29 - address match irq status, assert if either address match(and enabled). Write one clear NOTE: the address byte may not moved by DMA at this point. User can wait next addr_match_idle irq for the whole data include address
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<'_, Iir2Spec> {
        AddrMatchW::new(self, 29)
    }
    ///Bit 30 - UART TX IDLE Flag, assert after txd high and then tx idle timeout, write one clear 0 - UART TX is busy 1 - UART TX is idle
    #[inline(always)]
    pub fn txidle_flag(&mut self) -> TxidleFlagW<'_, Iir2Spec> {
        TxidleFlagW::new(self, 30)
    }
    ///Bit 31 - UART RX IDLE Flag, assert after rxd high and then rx idle timeout, write one clear 0 - UART RX is busy 1 - UART RX is idle
    #[inline(always)]
    pub fn rxidle_flag(&mut self) -> RxidleFlagW<'_, Iir2Spec> {
        RxidleFlagW::new(self, 31)
    }
}
/**Interrupt Identification Register2

You can [`read`](crate::Reg::read) this register and get [`iir2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iir2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Iir2Spec;
impl crate::RegisterSpec for Iir2Spec {
    type Ux = u32;
}
///`read()` method returns [`iir2::R`](R) reader structure
impl crate::Readable for Iir2Spec {}
///`write(|w| ..)` method takes [`iir2::W`](W) writer structure
impl crate::Writable for Iir2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IIR2 to value 0x01
impl crate::Resettable for Iir2Spec {
    const RESET_VALUE: u32 = 0x01;
}
