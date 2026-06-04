///Register `IIR` reader
pub type R = crate::R<Union28IirSpec>;
///Register `IIR` writer
pub type W = crate::W<Union28IirSpec>;
///Field `INTRID` reader - Interrupt ID, see IIR2 for detail decoding
pub type IntridR = crate::FieldReader;
///Field `FIFOED` reader - FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1.
pub type FifoedR = crate::FieldReader;
///Field `RXIDLE_FLAG` writer - UART IDLE Flag 0 - UART is busy 1 - UART is idle NOTE: when write one to clear this bit, avoid changging FCR register since it's same address as IIR
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
    ///Bit 31 - UART IDLE Flag 0 - UART is busy 1 - UART is idle NOTE: when write one to clear this bit, avoid changging FCR register since it's same address as IIR
    #[inline(always)]
    pub fn rxidle_flag(&mut self) -> RxidleFlagW<'_, Union28IirSpec> {
        RxidleFlagW::new(self, 31)
    }
}
/**Interrupt Identification Register

You can [`read`](crate::Reg::read) this register and get [`union_28_iir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_28_iir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union28IirSpec;
impl crate::RegisterSpec for Union28IirSpec {
    type Ux = u32;
}
///`read()` method returns [`union_28_iir::R`](R) reader structure
impl crate::Readable for Union28IirSpec {}
///`write(|w| ..)` method takes [`union_28_iir::W`](W) writer structure
impl crate::Writable for Union28IirSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IIR to value 0x01
impl crate::Resettable for Union28IirSpec {
    const RESET_VALUE: u32 = 0x01;
}
