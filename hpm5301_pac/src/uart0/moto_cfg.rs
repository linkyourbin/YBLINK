///Register `MOTO_CFG` reader
pub type R = crate::R<MotoCfgSpec>;
///Register `MOTO_CFG` writer
pub type W = crate::W<MotoCfgSpec>;
///Field `TXSTOP_INSERT` reader - set to insert STOP bits between each tx byte till tx fifo empty. NOTE: there will be no 1.5/2 STOP bits if enabled this feature, LCR.STB should be set to 0 if this bit is set
pub type TxstopInsertR = crate::BitReader;
///Field `TXSTOP_INSERT` writer - set to insert STOP bits between each tx byte till tx fifo empty. NOTE: there will be no 1.5/2 STOP bits if enabled this feature, LCR.STB should be set to 0 if this bit is set
pub type TxstopInsertW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRG_CLR_RFIFO` reader - set to enable the feature that, clear rxfifo at tx trigger(sw or hw), avoid unexpected data in rxfifo.
pub type TrgClrRfifoR = crate::BitReader;
///Field `TRG_CLR_RFIFO` writer - set to enable the feature that, clear rxfifo at tx trigger(sw or hw), avoid unexpected data in rxfifo.
pub type TrgClrRfifoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRG_MODE` reader - set to enable trigger mode. software should push needed data into txbuffer frist, uart will not start transmission at this time. User should send trigger signal(by hw or sw), uart will send all data in txfifo till empty NOTE: the hw_trigger should be pulse signal from trig mux.
pub type TrgModeR = crate::BitReader;
///Field `TRG_MODE` writer - set to enable trigger mode. software should push needed data into txbuffer frist, uart will not start transmission at this time. User should send trigger signal(by hw or sw), uart will send all data in txfifo till empty NOTE: the hw_trigger should be pulse signal from trig mux.
pub type TrgModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HWTRG_EN` reader - set to enable hardware trigger(trigger from moto is shared by other UART)
pub type HwtrgEnR = crate::BitReader;
///Field `HWTRG_EN` writer - set to enable hardware trigger(trigger from moto is shared by other UART)
pub type HwtrgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSTP_BITS` reader - if TXSTOP_INSERT is enabled, the STOP bits to be inserted between each byte. 0 for 1 bit; 0xFF for 256bits
pub type TxstpBitsR = crate::FieldReader;
///Field `TXSTP_BITS` writer - if TXSTOP_INSERT is enabled, the STOP bits to be inserted between each byte. 0 for 1 bit; 0xFF for 256bits
pub type TxstpBitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SWTRG` writer - software trigger. User should avoid use sw/hw trigger at same time, otherwise result unknown. Hardware auto reset.
pub type SwtrgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - set to insert STOP bits between each tx byte till tx fifo empty. NOTE: there will be no 1.5/2 STOP bits if enabled this feature, LCR.STB should be set to 0 if this bit is set
    #[inline(always)]
    pub fn txstop_insert(&self) -> TxstopInsertR {
        TxstopInsertR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - set to enable the feature that, clear rxfifo at tx trigger(sw or hw), avoid unexpected data in rxfifo.
    #[inline(always)]
    pub fn trg_clr_rfifo(&self) -> TrgClrRfifoR {
        TrgClrRfifoR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - set to enable trigger mode. software should push needed data into txbuffer frist, uart will not start transmission at this time. User should send trigger signal(by hw or sw), uart will send all data in txfifo till empty NOTE: the hw_trigger should be pulse signal from trig mux.
    #[inline(always)]
    pub fn trg_mode(&self) -> TrgModeR {
        TrgModeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - set to enable hardware trigger(trigger from moto is shared by other UART)
    #[inline(always)]
    pub fn hwtrg_en(&self) -> HwtrgEnR {
        HwtrgEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - if TXSTOP_INSERT is enabled, the STOP bits to be inserted between each byte. 0 for 1 bit; 0xFF for 256bits
    #[inline(always)]
    pub fn txstp_bits(&self) -> TxstpBitsR {
        TxstpBitsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bit 4 - set to insert STOP bits between each tx byte till tx fifo empty. NOTE: there will be no 1.5/2 STOP bits if enabled this feature, LCR.STB should be set to 0 if this bit is set
    #[inline(always)]
    pub fn txstop_insert(&mut self) -> TxstopInsertW<'_, MotoCfgSpec> {
        TxstopInsertW::new(self, 4)
    }
    ///Bit 5 - set to enable the feature that, clear rxfifo at tx trigger(sw or hw), avoid unexpected data in rxfifo.
    #[inline(always)]
    pub fn trg_clr_rfifo(&mut self) -> TrgClrRfifoW<'_, MotoCfgSpec> {
        TrgClrRfifoW::new(self, 5)
    }
    ///Bit 6 - set to enable trigger mode. software should push needed data into txbuffer frist, uart will not start transmission at this time. User should send trigger signal(by hw or sw), uart will send all data in txfifo till empty NOTE: the hw_trigger should be pulse signal from trig mux.
    #[inline(always)]
    pub fn trg_mode(&mut self) -> TrgModeW<'_, MotoCfgSpec> {
        TrgModeW::new(self, 6)
    }
    ///Bit 7 - set to enable hardware trigger(trigger from moto is shared by other UART)
    #[inline(always)]
    pub fn hwtrg_en(&mut self) -> HwtrgEnW<'_, MotoCfgSpec> {
        HwtrgEnW::new(self, 7)
    }
    ///Bits 8:15 - if TXSTOP_INSERT is enabled, the STOP bits to be inserted between each byte. 0 for 1 bit; 0xFF for 256bits
    #[inline(always)]
    pub fn txstp_bits(&mut self) -> TxstpBitsW<'_, MotoCfgSpec> {
        TxstpBitsW::new(self, 8)
    }
    ///Bit 31 - software trigger. User should avoid use sw/hw trigger at same time, otherwise result unknown. Hardware auto reset.
    #[inline(always)]
    pub fn swtrg(&mut self) -> SwtrgW<'_, MotoCfgSpec> {
        SwtrgW::new(self, 31)
    }
}
/**moto system control register

You can [`read`](crate::Reg::read) this register and get [`moto_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moto_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MotoCfgSpec;
impl crate::RegisterSpec for MotoCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`moto_cfg::R`](R) reader structure
impl crate::Readable for MotoCfgSpec {}
///`write(|w| ..)` method takes [`moto_cfg::W`](W) writer structure
impl crate::Writable for MotoCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOTO_CFG to value 0
impl crate::Resettable for MotoCfgSpec {}
