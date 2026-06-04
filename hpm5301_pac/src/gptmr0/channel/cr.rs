///Register `CR` reader
pub type R = crate::R<CrSpec>;
///Register `CR` writer
pub type W = crate::W<CrSpec>;
///Field `CAPMODE` reader - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture
pub type CapmodeR = crate::FieldReader;
///Field `CAPMODE` writer - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture
pub type CapmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBGPAUSE` reader - 1- counter will pause if chip is in debug mode
pub type DbgpauseR = crate::BitReader;
///Field `DBGPAUSE` writer - 1- counter will pause if chip is in debug mode
pub type DbgpauseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWSYNCIEN` reader - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set
pub type SwsyncienR = crate::BitReader;
///Field `SWSYNCIEN` writer - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set
pub type SwsyncienW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - 1- enable dma
pub type DmaenR = crate::BitReader;
///Field `DMAEN` writer - 1- enable dma
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMASEL` reader - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;
pub type DmaselR = crate::FieldReader;
///Field `DMASEL` writer - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;
pub type DmaselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CMPEN` reader - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings.
pub type CmpenR = crate::BitReader;
///Field `CMPEN` writer - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings.
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPINIT` reader - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1.
pub type CmpinitR = crate::BitReader;
///Field `CMPINIT` writer - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1.
pub type CmpinitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEN` reader - 1- counter enable
pub type CenR = crate::BitReader;
///Field `CEN` writer - 1- counter enable
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCIREN` reader - 1- SYNCI is valid on its rising edge
pub type SyncirenR = crate::BitReader;
///Field `SYNCIREN` writer - 1- SYNCI is valid on its rising edge
pub type SyncirenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCIFEN` reader - 1- SYNCI is valid on its falling edge
pub type SyncifenR = crate::BitReader;
///Field `SYNCIFEN` writer - 1- SYNCI is valid on its falling edge
pub type SyncifenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCFLW` reader - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0.
pub type SyncflwR = crate::BitReader;
///Field `SYNCFLW` writer - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0.
pub type SyncflwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTRST` reader - 1- reset counter
pub type CntrstR = crate::BitReader;
///Field `CNTRST` writer - 1- reset counter
pub type CntrstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONITOR_EN` reader - set to monitor input signal period or high level time. When this bit is set, if detected period less than val_0 or more than val_1, will set related irq_sts * only can be used when trig_mode is selected as measure mode(100) * the time may not correct after reload, so monitor is disabled after reload point, and enabled again after two continul posedge. if no posedge after reload for more than val_1, will also assert irq_capt
pub type MonitorEnR = crate::BitReader;
///Field `MONITOR_EN` writer - set to monitor input signal period or high level time. When this bit is set, if detected period less than val_0 or more than val_1, will set related irq_sts * only can be used when trig_mode is selected as measure mode(100) * the time may not correct after reload, so monitor is disabled after reload point, and enabled again after two continul posedge. if no posedge after reload for more than val_1, will also assert irq_capt
pub type MonitorEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONITOR_SEL` reader - set to monitor input signal high level time(chan_meas_high) clr to monitor input signal period(chan_meas_prd)
pub type MonitorSelR = crate::BitReader;
///Field `MONITOR_SEL` writer - set to monitor input signal high level time(chan_meas_high) clr to monitor input signal period(chan_meas_prd)
pub type MonitorSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPMODE` reader - 0: round mode 1: one-shot mode, timer will stopped at reload point.user need clear CEN and set it to start timer agian. NOTE: reload irq will be always set at one-shot mode at end
pub type OpmodeR = crate::BitReader;
///Field `OPMODE` writer - 0: round mode 1: one-shot mode, timer will stopped at reload point.user need clear CEN and set it to start timer agian. NOTE: reload irq will be always set at one-shot mode at end
pub type OpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTUPT` writer - 1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle
pub type CntuptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture
    #[inline(always)]
    pub fn capmode(&self) -> CapmodeR {
        CapmodeR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - 1- counter will pause if chip is in debug mode
    #[inline(always)]
    pub fn dbgpause(&self) -> DbgpauseR {
        DbgpauseR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set
    #[inline(always)]
    pub fn swsyncien(&self) -> SwsyncienR {
        SwsyncienR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 1- enable dma
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;
    #[inline(always)]
    pub fn dmasel(&self) -> DmaselR {
        DmaselR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings.
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1.
    #[inline(always)]
    pub fn cmpinit(&self) -> CmpinitR {
        CmpinitR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 1- counter enable
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 1- SYNCI is valid on its rising edge
    #[inline(always)]
    pub fn synciren(&self) -> SyncirenR {
        SyncirenR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 1- SYNCI is valid on its falling edge
    #[inline(always)]
    pub fn syncifen(&self) -> SyncifenR {
        SyncifenR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0.
    #[inline(always)]
    pub fn syncflw(&self) -> SyncflwR {
        SyncflwR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - 1- reset counter
    #[inline(always)]
    pub fn cntrst(&self) -> CntrstR {
        CntrstR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - set to monitor input signal period or high level time. When this bit is set, if detected period less than val_0 or more than val_1, will set related irq_sts * only can be used when trig_mode is selected as measure mode(100) * the time may not correct after reload, so monitor is disabled after reload point, and enabled again after two continul posedge. if no posedge after reload for more than val_1, will also assert irq_capt
    #[inline(always)]
    pub fn monitor_en(&self) -> MonitorEnR {
        MonitorEnR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - set to monitor input signal high level time(chan_meas_high) clr to monitor input signal period(chan_meas_prd)
    #[inline(always)]
    pub fn monitor_sel(&self) -> MonitorSelR {
        MonitorSelR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - 0: round mode 1: one-shot mode, timer will stopped at reload point.user need clear CEN and set it to start timer agian. NOTE: reload irq will be always set at one-shot mode at end
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture
    #[inline(always)]
    pub fn capmode(&mut self) -> CapmodeW<'_, CrSpec> {
        CapmodeW::new(self, 0)
    }
    ///Bit 3 - 1- counter will pause if chip is in debug mode
    #[inline(always)]
    pub fn dbgpause(&mut self) -> DbgpauseW<'_, CrSpec> {
        DbgpauseW::new(self, 3)
    }
    ///Bit 4 - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set
    #[inline(always)]
    pub fn swsyncien(&mut self) -> SwsyncienW<'_, CrSpec> {
        SwsyncienW::new(self, 4)
    }
    ///Bit 5 - 1- enable dma
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, CrSpec> {
        DmaenW::new(self, 5)
    }
    ///Bits 6:7 - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;
    #[inline(always)]
    pub fn dmasel(&mut self) -> DmaselW<'_, CrSpec> {
        DmaselW::new(self, 6)
    }
    ///Bit 8 - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings.
    #[inline(always)]
    pub fn cmpen(&mut self) -> CmpenW<'_, CrSpec> {
        CmpenW::new(self, 8)
    }
    ///Bit 9 - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1.
    #[inline(always)]
    pub fn cmpinit(&mut self) -> CmpinitW<'_, CrSpec> {
        CmpinitW::new(self, 9)
    }
    ///Bit 10 - 1- counter enable
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, CrSpec> {
        CenW::new(self, 10)
    }
    ///Bit 11 - 1- SYNCI is valid on its rising edge
    #[inline(always)]
    pub fn synciren(&mut self) -> SyncirenW<'_, CrSpec> {
        SyncirenW::new(self, 11)
    }
    ///Bit 12 - 1- SYNCI is valid on its falling edge
    #[inline(always)]
    pub fn syncifen(&mut self) -> SyncifenW<'_, CrSpec> {
        SyncifenW::new(self, 12)
    }
    ///Bit 13 - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0.
    #[inline(always)]
    pub fn syncflw(&mut self) -> SyncflwW<'_, CrSpec> {
        SyncflwW::new(self, 13)
    }
    ///Bit 14 - 1- reset counter
    #[inline(always)]
    pub fn cntrst(&mut self) -> CntrstW<'_, CrSpec> {
        CntrstW::new(self, 14)
    }
    ///Bit 15 - set to monitor input signal period or high level time. When this bit is set, if detected period less than val_0 or more than val_1, will set related irq_sts * only can be used when trig_mode is selected as measure mode(100) * the time may not correct after reload, so monitor is disabled after reload point, and enabled again after two continul posedge. if no posedge after reload for more than val_1, will also assert irq_capt
    #[inline(always)]
    pub fn monitor_en(&mut self) -> MonitorEnW<'_, CrSpec> {
        MonitorEnW::new(self, 15)
    }
    ///Bit 16 - set to monitor input signal high level time(chan_meas_high) clr to monitor input signal period(chan_meas_prd)
    #[inline(always)]
    pub fn monitor_sel(&mut self) -> MonitorSelW<'_, CrSpec> {
        MonitorSelW::new(self, 16)
    }
    ///Bit 17 - 0: round mode 1: one-shot mode, timer will stopped at reload point.user need clear CEN and set it to start timer agian. NOTE: reload irq will be always set at one-shot mode at end
    #[inline(always)]
    pub fn opmode(&mut self) -> OpmodeW<'_, CrSpec> {
        OpmodeW::new(self, 17)
    }
    ///Bit 31 - 1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle
    #[inline(always)]
    pub fn cntupt(&mut self) -> CntuptW<'_, CrSpec> {
        CntuptW::new(self, 31)
    }
}
/**Control Register

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
