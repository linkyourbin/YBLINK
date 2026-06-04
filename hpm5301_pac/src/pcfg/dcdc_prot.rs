///Register `DCDC_PROT` reader
pub type R = crate::R<DcdcProtSpec>;
///Register `DCDC_PROT` writer
pub type W = crate::W<DcdcProtSpec>;
///Field `SHORT_FLAG` reader - short circuit flag 0: current is within limit 1: short circuits detected
pub type ShortFlagR = crate::BitReader;
///Field `SHORT_CURRENT` reader - short circuit current setting 0: 2.0A, 1: 1.3A
pub type ShortCurrentR = crate::BitReader;
///Field `SHORT_CURRENT` writer - short circuit current setting 0: 2.0A, 1: 1.3A
pub type ShortCurrentW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISABLE_SHORT` reader - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled
pub type DisableShortR = crate::BitReader;
///Field `DISABLE_SHORT` writer - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled
pub type DisableShortW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVERVOLT_FLAG` reader - output over voltage flag 0: output is normal 1: output is unexpected high
pub type OvervoltFlagR = crate::BitReader;
///Field `DISABLE_OVERVOLTAGE` reader - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage
pub type DisableOvervoltageR = crate::BitReader;
///Field `DISABLE_OVERVOLTAGE` writer - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage
pub type DisableOvervoltageW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POWER_LOSS_FLAG` reader - power loss 0: input power is good 1: input power is too low
pub type PowerLossFlagR = crate::BitReader;
///Field `OVERLOAD_LP` reader - over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode
pub type OverloadLpR = crate::BitReader;
///Field `ILIMIT_LP` reader - over current setting for low power mode 0:250mA 1:200mA
pub type IlimitLpR = crate::BitReader;
///Field `ILIMIT_LP` writer - over current setting for low power mode 0:250mA 1:200mA
pub type IlimitLpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - short circuit flag 0: current is within limit 1: short circuits detected
    #[inline(always)]
    pub fn short_flag(&self) -> ShortFlagR {
        ShortFlagR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - short circuit current setting 0: 2.0A, 1: 1.3A
    #[inline(always)]
    pub fn short_current(&self) -> ShortCurrentR {
        ShortCurrentR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled
    #[inline(always)]
    pub fn disable_short(&self) -> DisableShortR {
        DisableShortR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - output over voltage flag 0: output is normal 1: output is unexpected high
    #[inline(always)]
    pub fn overvolt_flag(&self) -> OvervoltFlagR {
        OvervoltFlagR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage
    #[inline(always)]
    pub fn disable_overvoltage(&self) -> DisableOvervoltageR {
        DisableOvervoltageR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - power loss 0: input power is good 1: input power is too low
    #[inline(always)]
    pub fn power_loss_flag(&self) -> PowerLossFlagR {
        PowerLossFlagR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode
    #[inline(always)]
    pub fn overload_lp(&self) -> OverloadLpR {
        OverloadLpR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - over current setting for low power mode 0:250mA 1:200mA
    #[inline(always)]
    pub fn ilimit_lp(&self) -> IlimitLpR {
        IlimitLpR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - short circuit current setting 0: 2.0A, 1: 1.3A
    #[inline(always)]
    pub fn short_current(&mut self) -> ShortCurrentW<'_, DcdcProtSpec> {
        ShortCurrentW::new(self, 4)
    }
    ///Bit 7 - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled
    #[inline(always)]
    pub fn disable_short(&mut self) -> DisableShortW<'_, DcdcProtSpec> {
        DisableShortW::new(self, 7)
    }
    ///Bit 15 - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage
    #[inline(always)]
    pub fn disable_overvoltage(&mut self) -> DisableOvervoltageW<'_, DcdcProtSpec> {
        DisableOvervoltageW::new(self, 15)
    }
    ///Bit 28 - over current setting for low power mode 0:250mA 1:200mA
    #[inline(always)]
    pub fn ilimit_lp(&mut self) -> IlimitLpW<'_, DcdcProtSpec> {
        IlimitLpW::new(self, 28)
    }
}
/**DCDC protection

You can [`read`](crate::Reg::read) this register and get [`dcdc_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcProtSpec;
impl crate::RegisterSpec for DcdcProtSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_prot::R`](R) reader structure
impl crate::Readable for DcdcProtSpec {}
///`write(|w| ..)` method takes [`dcdc_prot::W`](W) writer structure
impl crate::Writable for DcdcProtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_PROT to value 0x10
impl crate::Resettable for DcdcProtSpec {
    const RESET_VALUE: u32 = 0x10;
}
