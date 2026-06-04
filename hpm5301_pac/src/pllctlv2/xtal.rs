///Register `XTAL` reader
pub type R = crate::R<XtalSpec>;
///Register `XTAL` writer
pub type W = crate::W<XtalSpec>;
///Field `RAMP_TIME` reader - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles
pub type RampTimeR = crate::FieldReader<u32>;
///Field `RAMP_TIME` writer - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles
pub type RampTimeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `ENABLE` reader - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on
pub type EnableR = crate::BitReader;
///Field `RESPONSE` reader - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use
pub type ResponseR = crate::BitReader;
///Field `BUSY` reader - Busy flag 0: Oscillator is working or shutdown 1: Oscillator is changing status
pub type BusyR = crate::BitReader;
impl R {
    ///Bits 0:19 - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles
    #[inline(always)]
    pub fn ramp_time(&self) -> RampTimeR {
        RampTimeR::new(self.bits & 0x000f_ffff)
    }
    ///Bit 28 - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Busy flag 0: Oscillator is working or shutdown 1: Oscillator is changing status
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:19 - Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles
    #[inline(always)]
    pub fn ramp_time(&mut self) -> RampTimeW<'_, XtalSpec> {
        RampTimeW::new(self, 0)
    }
}
/**OSC configuration

You can [`read`](crate::Reg::read) this register and get [`xtal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XtalSpec;
impl crate::RegisterSpec for XtalSpec {
    type Ux = u32;
}
///`read()` method returns [`xtal::R`](R) reader structure
impl crate::Readable for XtalSpec {}
///`write(|w| ..)` method takes [`xtal::W`](W) writer structure
impl crate::Writable for XtalSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets XTAL to value 0x0001_ffff
impl crate::Resettable for XtalSpec {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
