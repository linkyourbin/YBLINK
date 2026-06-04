///Register `POWER_TRAP` reader
pub type R = crate::R<PowerTrapSpec>;
///Register `POWER_TRAP` writer
pub type W = crate::W<PowerTrapSpec>;
///Field `TRAP` reader - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned.
pub type TrapR = crate::BitReader;
///Field `TRAP` writer - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned.
pub type TrapW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETENTION` reader - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage
pub type RetentionR = crate::BitReader;
///Field `RETENTION` writer - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage
pub type RetentionW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGGERED` reader - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered
pub type TriggeredR = crate::BitReader;
///Field `TRIGGERED` writer - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered
pub type TriggeredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned.
    #[inline(always)]
    pub fn trap(&self) -> TrapR {
        TrapR::new((self.bits & 1) != 0)
    }
    ///Bit 16 - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage
    #[inline(always)]
    pub fn retention(&self) -> RetentionR {
        RetentionR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered
    #[inline(always)]
    pub fn triggered(&self) -> TriggeredR {
        TriggeredR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned.
    #[inline(always)]
    pub fn trap(&mut self) -> TrapW<'_, PowerTrapSpec> {
        TrapW::new(self, 0)
    }
    ///Bit 16 - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage
    #[inline(always)]
    pub fn retention(&mut self) -> RetentionW<'_, PowerTrapSpec> {
        RetentionW::new(self, 16)
    }
    ///Bit 31 - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered
    #[inline(always)]
    pub fn triggered(&mut self) -> TriggeredW<'_, PowerTrapSpec> {
        TriggeredW::new(self, 31)
    }
}
/**SOC power trap

You can [`read`](crate::Reg::read) this register and get [`power_trap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_trap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PowerTrapSpec;
impl crate::RegisterSpec for PowerTrapSpec {
    type Ux = u32;
}
///`read()` method returns [`power_trap::R`](R) reader structure
impl crate::Readable for PowerTrapSpec {}
///`write(|w| ..)` method takes [`power_trap::W`](W) writer structure
impl crate::Writable for PowerTrapSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POWER_TRAP to value 0
impl crate::Resettable for PowerTrapSpec {}
