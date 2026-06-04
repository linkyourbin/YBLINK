///Register `IER` reader
pub type R = crate::R<Union24IerSpec>;
///Register `IER` writer
pub type W = crate::W<Union24IerSpec>;
///Field `ERBI` reader - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable
pub type ErbiR = crate::BitReader;
///Field `ERBI` writer - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable
pub type ErbiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHEI` reader - Enable transmitter holding register interrupt
pub type EtheiR = crate::BitReader;
///Field `ETHEI` writer - Enable transmitter holding register interrupt
pub type EtheiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELSI` reader - Enable receiver line status interrupt
pub type ElsiR = crate::BitReader;
///Field `ELSI` writer - Enable receiver line status interrupt
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMSI` reader - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter.
pub type EmsiR = crate::BitReader;
///Field `EMSI` writer - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter.
pub type EmsiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDATLOST` reader - enable DATA_LOST interrupt
pub type EdatlostR = crate::BitReader;
///Field `EDATLOST` writer - enable DATA_LOST interrupt
pub type EdatlostW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EADDRM_IDLE` reader - enable ADDR_MATCH_IDLE interrupt
pub type EaddrmIdleR = crate::BitReader;
///Field `EADDRM_IDLE` writer - enable ADDR_MATCH_IDLE interrupt
pub type EaddrmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EADDRM` reader - enable ADDR_MATCH interrupt
pub type EaddrmR = crate::BitReader;
///Field `EADDRM` writer - enable ADDR_MATCH interrupt
pub type EaddrmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETXIDLE` reader - enable transmit idle interrupt
pub type EtxidleR = crate::BitReader;
///Field `ETXIDLE` writer - enable transmit idle interrupt
pub type EtxidleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERXIDLE` reader - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt
pub type ErxidleR = crate::BitReader;
///Field `ERXIDLE` writer - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt
pub type ErxidleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable
    #[inline(always)]
    pub fn erbi(&self) -> ErbiR {
        ErbiR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable transmitter holding register interrupt
    #[inline(always)]
    pub fn ethei(&self) -> EtheiR {
        EtheiR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable receiver line status interrupt
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter.
    #[inline(always)]
    pub fn emsi(&self) -> EmsiR {
        EmsiR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 27 - enable DATA_LOST interrupt
    #[inline(always)]
    pub fn edatlost(&self) -> EdatlostR {
        EdatlostR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - enable ADDR_MATCH_IDLE interrupt
    #[inline(always)]
    pub fn eaddrm_idle(&self) -> EaddrmIdleR {
        EaddrmIdleR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - enable ADDR_MATCH interrupt
    #[inline(always)]
    pub fn eaddrm(&self) -> EaddrmR {
        EaddrmR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - enable transmit idle interrupt
    #[inline(always)]
    pub fn etxidle(&self) -> EtxidleR {
        EtxidleR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt
    #[inline(always)]
    pub fn erxidle(&self) -> ErxidleR {
        ErxidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable
    #[inline(always)]
    pub fn erbi(&mut self) -> ErbiW<'_, Union24IerSpec> {
        ErbiW::new(self, 0)
    }
    ///Bit 1 - Enable transmitter holding register interrupt
    #[inline(always)]
    pub fn ethei(&mut self) -> EtheiW<'_, Union24IerSpec> {
        EtheiW::new(self, 1)
    }
    ///Bit 2 - Enable receiver line status interrupt
    #[inline(always)]
    pub fn elsi(&mut self) -> ElsiW<'_, Union24IerSpec> {
        ElsiW::new(self, 2)
    }
    ///Bit 3 - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter.
    #[inline(always)]
    pub fn emsi(&mut self) -> EmsiW<'_, Union24IerSpec> {
        EmsiW::new(self, 3)
    }
    ///Bit 27 - enable DATA_LOST interrupt
    #[inline(always)]
    pub fn edatlost(&mut self) -> EdatlostW<'_, Union24IerSpec> {
        EdatlostW::new(self, 27)
    }
    ///Bit 28 - enable ADDR_MATCH_IDLE interrupt
    #[inline(always)]
    pub fn eaddrm_idle(&mut self) -> EaddrmIdleW<'_, Union24IerSpec> {
        EaddrmIdleW::new(self, 28)
    }
    ///Bit 29 - enable ADDR_MATCH interrupt
    #[inline(always)]
    pub fn eaddrm(&mut self) -> EaddrmW<'_, Union24IerSpec> {
        EaddrmW::new(self, 29)
    }
    ///Bit 30 - enable transmit idle interrupt
    #[inline(always)]
    pub fn etxidle(&mut self) -> EtxidleW<'_, Union24IerSpec> {
        EtxidleW::new(self, 30)
    }
    ///Bit 31 - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt
    #[inline(always)]
    pub fn erxidle(&mut self) -> ErxidleW<'_, Union24IerSpec> {
        ErxidleW::new(self, 31)
    }
}
/**Interrupt Enable Register (when DLAB = 0)

You can [`read`](crate::Reg::read) this register and get [`union_24_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_24_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union24IerSpec;
impl crate::RegisterSpec for Union24IerSpec {
    type Ux = u32;
}
///`read()` method returns [`union_24_ier::R`](R) reader structure
impl crate::Readable for Union24IerSpec {}
///`write(|w| ..)` method takes [`union_24_ier::W`](W) writer structure
impl crate::Writable for Union24IerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for Union24IerSpec {}
