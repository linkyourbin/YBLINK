///Register `MCR` reader
pub type R = crate::R<McrSpec>;
///Register `MCR` writer
pub type W = crate::W<McrSpec>;
///Field `RTS` reader - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW
pub type RtsR = crate::BitReader;
///Field `RTS` writer - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOP` reader - Enable loopback mode 0: Disable 1: Enable
pub type LoopR = crate::BitReader;
///Field `LOOP` writer - Enable loopback mode 0: Disable 1: Enable
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFE` reader - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS
pub type AfeR = crate::BitReader;
///Field `AFE` writer - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS
pub type AfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Enable loopback mode 0: Disable 1: Enable
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS
    #[inline(always)]
    pub fn afe(&self) -> AfeR {
        AfeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, McrSpec> {
        RtsW::new(self, 1)
    }
    ///Bit 4 - Enable loopback mode 0: Disable 1: Enable
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<'_, McrSpec> {
        LoopW::new(self, 4)
    }
    ///Bit 5 - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS
    #[inline(always)]
    pub fn afe(&mut self) -> AfeW<'_, McrSpec> {
        AfeW::new(self, 5)
    }
}
/**Modem Control Register (

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for McrSpec {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for McrSpec {}
