///Register `CTRL0` reader
pub type R = crate::R<Ctrl0Spec>;
///Register `CTRL0` writer
pub type W = crate::W<Ctrl0Spec>;
///Field `EN_LP` reader - WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode
pub type EnLpR = crate::FieldReader;
///Field `EN_LP` writer - WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode
pub type EnLpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EN_DBG` reader - WTD enable or not in debug mode
pub type EnDbgR = crate::BitReader;
///Field `EN_DBG` writer - WTD enable or not in debug mode
pub type EnDbgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REF_UNLOCK_MEC` reader - Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1
pub type RefUnlockMecR = crate::FieldReader;
///Field `REF_UNLOCK_MEC` writer - Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1
pub type RefUnlockMecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REF_LOCK` reader - WDT refresh has to be unlocked firstly once refresh lock is enable.
pub type RefLockR = crate::BitReader;
///Field `REF_LOCK` writer - WDT refresh has to be unlocked firstly once refresh lock is enable.
pub type RefLockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIN_UPPER` reader - The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation
pub type WinUpperR = crate::FieldReader;
///Field `WIN_UPPER` writer - The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation
pub type WinUpperW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `REF_OT_REQ` reader - If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
pub type RefOtReqR = crate::BitReader;
///Field `REF_OT_REQ` writer - If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
pub type RefOtReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFG_LOCK` reader - The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register
pub type CfgLockR = crate::BitReader;
///Field `CFG_LOCK` writer - The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register
pub type CfgLockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIN_LOWER` reader - Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value
pub type WinLowerR = crate::FieldReader;
///Field `WIN_LOWER` writer - Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value
pub type WinLowerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WIN_EN` reader - window mode enable
pub type WinEnR = crate::BitReader;
///Field `WIN_EN` writer - window mode enable
pub type WinEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIV_VALUE` reader - clock divider, the clock divider works as 2 ^ div_value for wdt counter
pub type DivValueR = crate::FieldReader;
///Field `DIV_VALUE` writer - clock divider, the clock divider works as 2 ^ div_value for wdt counter
pub type DivValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLK_SEL` reader - clock select 0：bus clock 1：ext clock
pub type ClkSelR = crate::BitReader;
///Field `CLK_SEL` writer - clock select 0：bus clock 1：ext clock
pub type ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode
    #[inline(always)]
    pub fn en_lp(&self) -> EnLpR {
        EnLpR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - WTD enable or not in debug mode
    #[inline(always)]
    pub fn en_dbg(&self) -> EnDbgR {
        EnDbgR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1
    #[inline(always)]
    pub fn ref_unlock_mec(&self) -> RefUnlockMecR {
        RefUnlockMecR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - WDT refresh has to be unlocked firstly once refresh lock is enable.
    #[inline(always)]
    pub fn ref_lock(&self) -> RefLockR {
        RefLockR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 12:14 - The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation
    #[inline(always)]
    pub fn win_upper(&self) -> WinUpperR {
        WinUpperR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
    #[inline(always)]
    pub fn ref_ot_req(&self) -> RefOtReqR {
        RefOtReqR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register
    #[inline(always)]
    pub fn cfg_lock(&self) -> CfgLockR {
        CfgLockR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value
    #[inline(always)]
    pub fn win_lower(&self) -> WinLowerR {
        WinLowerR::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - window mode enable
    #[inline(always)]
    pub fn win_en(&self) -> WinEnR {
        WinEnR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - clock divider, the clock divider works as 2 ^ div_value for wdt counter
    #[inline(always)]
    pub fn div_value(&self) -> DivValueR {
        DivValueR::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 29 - clock select 0：bus clock 1：ext clock
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode
    #[inline(always)]
    pub fn en_lp(&mut self) -> EnLpW<'_, Ctrl0Spec> {
        EnLpW::new(self, 0)
    }
    ///Bit 2 - WTD enable or not in debug mode
    #[inline(always)]
    pub fn en_dbg(&mut self) -> EnDbgW<'_, Ctrl0Spec> {
        EnDbgW::new(self, 2)
    }
    ///Bits 3:4 - Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1
    #[inline(always)]
    pub fn ref_unlock_mec(&mut self) -> RefUnlockMecW<'_, Ctrl0Spec> {
        RefUnlockMecW::new(self, 3)
    }
    ///Bit 5 - WDT refresh has to be unlocked firstly once refresh lock is enable.
    #[inline(always)]
    pub fn ref_lock(&mut self) -> RefLockW<'_, Ctrl0Spec> {
        RefLockW::new(self, 5)
    }
    ///Bits 12:14 - The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation
    #[inline(always)]
    pub fn win_upper(&mut self) -> WinUpperW<'_, Ctrl0Spec> {
        WinUpperW::new(self, 12)
    }
    ///Bit 15 - If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter
    #[inline(always)]
    pub fn ref_ot_req(&mut self) -> RefOtReqW<'_, Ctrl0Spec> {
        RefOtReqW::new(self, 15)
    }
    ///Bit 21 - The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register
    #[inline(always)]
    pub fn cfg_lock(&mut self) -> CfgLockW<'_, Ctrl0Spec> {
        CfgLockW::new(self, 21)
    }
    ///Bits 22:23 - Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value
    #[inline(always)]
    pub fn win_lower(&mut self) -> WinLowerW<'_, Ctrl0Spec> {
        WinLowerW::new(self, 22)
    }
    ///Bit 24 - window mode enable
    #[inline(always)]
    pub fn win_en(&mut self) -> WinEnW<'_, Ctrl0Spec> {
        WinEnW::new(self, 24)
    }
    ///Bits 25:27 - clock divider, the clock divider works as 2 ^ div_value for wdt counter
    #[inline(always)]
    pub fn div_value(&mut self) -> DivValueW<'_, Ctrl0Spec> {
        DivValueW::new(self, 25)
    }
    ///Bit 29 - clock select 0：bus clock 1：ext clock
    #[inline(always)]
    pub fn clk_sel(&mut self) -> ClkSelW<'_, Ctrl0Spec> {
        ClkSelW::new(self, 29)
    }
}
/**wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits

You can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
///`read()` method returns [`ctrl0::R`](R) reader structure
impl crate::Readable for Ctrl0Spec {}
///`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTRL0 to value 0
impl crate::Resettable for Ctrl0Spec {}
