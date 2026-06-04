///Register `DCDC_MODE` reader
pub type R = crate::R<DcdcModeSpec>;
///Register `DCDC_MODE` writer
pub type W = crate::W<DcdcModeSpec>;
///Field `VOLT` reader - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
pub type VoltR = crate::FieldReader<u16>;
///Field `VOLT` writer - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
pub type VoltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `MODE` reader - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode
pub type ModeR = crate::FieldReader;
///Field `MODE` writer - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `READY` reader - Ready flag 0: DCDC is applying new change 1: DCDC is ready
pub type ReadyR = crate::BitReader;
impl R {
    ///Bits 0:11 - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
    #[inline(always)]
    pub fn volt(&self) -> VoltR {
        VoltR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:18 - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 28 - Ready flag 0: DCDC is applying new change 1: DCDC is ready
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV
    #[inline(always)]
    pub fn volt(&mut self) -> VoltW<'_, DcdcModeSpec> {
        VoltW::new(self, 0)
    }
    ///Bits 16:18 - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DcdcModeSpec> {
        ModeW::new(self, 16)
    }
}
/**DCDC mode select

You can [`read`](crate::Reg::read) this register and get [`dcdc_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcModeSpec;
impl crate::RegisterSpec for DcdcModeSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_mode::R`](R) reader structure
impl crate::Readable for DcdcModeSpec {}
///`write(|w| ..)` method takes [`dcdc_mode::W`](W) writer structure
impl crate::Writable for DcdcModeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_MODE to value 0x0001_047e
impl crate::Resettable for DcdcModeSpec {
    const RESET_VALUE: u32 = 0x0001_047e;
}
