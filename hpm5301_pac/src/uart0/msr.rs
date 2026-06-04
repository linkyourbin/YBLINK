///Register `MSR` reader
pub type R = crate::R<MsrSpec>;
///Register `MSR` writer
pub type W = crate::W<MsrSpec>;
///Field `DCTS` reader - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read.
pub type DctsR = crate::BitReader;
///Field `CTS` reader - Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW.
pub type CtsR = crate::BitReader;
impl R {
    ///Bit 0 - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read.
    #[inline(always)]
    pub fn dcts(&self) -> DctsR {
        DctsR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW.
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {}
/**Modem Status Register

You can [`read`](crate::Reg::read) this register and get [`msr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
///`read()` method returns [`msr::R`](R) reader structure
impl crate::Readable for MsrSpec {}
///`write(|w| ..)` method takes [`msr::W`](W) writer structure
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSR to value 0
impl crate::Resettable for MsrSpec {}
