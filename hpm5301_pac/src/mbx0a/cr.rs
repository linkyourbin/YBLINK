///Register `CR` reader
pub type R = crate::R<CrSpec>;
///Register `CR` writer
pub type W = crate::W<CrSpec>;
///Field `RWMVIE` reader - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt.
pub type RwmvieR = crate::BitReader;
///Field `RWMVIE` writer - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt.
pub type RwmvieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TWMEIE` reader - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt.
pub type TwmeieR = crate::BitReader;
///Field `TWMEIE` writer - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt.
pub type TwmeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFMFIE` reader - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt.
pub type RfmfieR = crate::BitReader;
///Field `RFMFIE` writer - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt.
pub type RfmfieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFMAIE` reader - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt.
pub type RfmaieR = crate::BitReader;
///Field `RFMAIE` writer - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt.
pub type RfmaieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFMEIE` reader - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt.
pub type TfmeieR = crate::BitReader;
///Field `TFMEIE` writer - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt.
pub type TfmeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFMAIE` reader - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt.
pub type TfmaieR = crate::BitReader;
///Field `TFMAIE` writer - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt.
pub type TfmaieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BEIE` reader - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt.
pub type BeieR = crate::BitReader;
///Field `BEIE` writer - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt.
pub type BeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BARCTL` reader - Bus Access Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved.
pub type BarctlR = crate::FieldReader;
///Field `BARCTL` writer - Bus Access Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved.
pub type BarctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TXRESET` reader - Reset TX Fifo and word.
pub type TxresetR = crate::BitReader;
///Field `TXRESET` writer - Reset TX Fifo and word.
pub type TxresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt.
    #[inline(always)]
    pub fn rwmvie(&self) -> RwmvieR {
        RwmvieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt.
    #[inline(always)]
    pub fn twmeie(&self) -> TwmeieR {
        TwmeieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt.
    #[inline(always)]
    pub fn rfmfie(&self) -> RfmfieR {
        RfmfieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt.
    #[inline(always)]
    pub fn rfmaie(&self) -> RfmaieR {
        RfmaieR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt.
    #[inline(always)]
    pub fn tfmeie(&self) -> TfmeieR {
        TfmeieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt.
    #[inline(always)]
    pub fn tfmaie(&self) -> TfmaieR {
        TfmaieR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt.
    #[inline(always)]
    pub fn beie(&self) -> BeieR {
        BeieR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 14:15 - Bus Access Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved.
    #[inline(always)]
    pub fn barctl(&self) -> BarctlR {
        BarctlR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 31 - Reset TX Fifo and word.
    #[inline(always)]
    pub fn txreset(&self) -> TxresetR {
        TxresetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt.
    #[inline(always)]
    pub fn rwmvie(&mut self) -> RwmvieW<'_, CrSpec> {
        RwmvieW::new(self, 0)
    }
    ///Bit 1 - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt.
    #[inline(always)]
    pub fn twmeie(&mut self) -> TwmeieW<'_, CrSpec> {
        TwmeieW::new(self, 1)
    }
    ///Bit 4 - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt.
    #[inline(always)]
    pub fn rfmfie(&mut self) -> RfmfieW<'_, CrSpec> {
        RfmfieW::new(self, 4)
    }
    ///Bit 5 - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt.
    #[inline(always)]
    pub fn rfmaie(&mut self) -> RfmaieW<'_, CrSpec> {
        RfmaieW::new(self, 5)
    }
    ///Bit 6 - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt.
    #[inline(always)]
    pub fn tfmeie(&mut self) -> TfmeieW<'_, CrSpec> {
        TfmeieW::new(self, 6)
    }
    ///Bit 7 - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt.
    #[inline(always)]
    pub fn tfmaie(&mut self) -> TfmaieW<'_, CrSpec> {
        TfmaieW::new(self, 7)
    }
    ///Bit 8 - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt.
    #[inline(always)]
    pub fn beie(&mut self) -> BeieW<'_, CrSpec> {
        BeieW::new(self, 8)
    }
    ///Bits 14:15 - Bus Access Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved.
    #[inline(always)]
    pub fn barctl(&mut self) -> BarctlW<'_, CrSpec> {
        BarctlW::new(self, 14)
    }
    ///Bit 31 - Reset TX Fifo and word.
    #[inline(always)]
    pub fn txreset(&mut self) -> TxresetW<'_, CrSpec> {
        TxresetW::new(self, 31)
    }
}
/**Command Registers

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CrSpec {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CrSpec {}
