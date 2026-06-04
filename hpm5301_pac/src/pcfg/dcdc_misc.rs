///Register `DCDC_MISC` reader
pub type R = crate::R<DcdcMiscSpec>;
///Register `DCDC_MISC` writer
pub type W = crate::W<DcdcMiscSpec>;
///Field `EN_STEP` reader - enable stepping in voltage change 0: stepping disabled 1: steping enabled
pub type EnStepR = crate::BitReader;
///Field `EN_STEP` writer - enable stepping in voltage change 0: stepping disabled 1: steping enabled
pub type EnStepW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_SEL` reader - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator
pub type ClkSelR = crate::BitReader;
///Field `CLK_SEL` writer - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator
pub type ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DELAY` reader - enable delay 0: delay disabled, 1: delay enabled
pub type DelayR = crate::BitReader;
///Field `DELAY` writer - enable delay 0: delay disabled, 1: delay enabled
pub type DelayW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OL_HYST` reader - voltage ripple threshold in low power mode 0: 12.5mV 1: 25mV
pub type OlHystR = crate::BitReader;
///Field `OL_HYST` writer - voltage ripple threshold in low power mode 0: 12.5mV 1: 25mV
pub type OlHystW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OL_THRE` reader - overload threshold in low power mode
pub type OlThreR = crate::FieldReader;
///Field `OL_THRE` writer - overload threshold in low power mode
pub type OlThreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DC_FF` reader - Loop feed forward number
pub type DcFfR = crate::FieldReader;
///Field `DC_FF` writer - Loop feed forward number
pub type DcFfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RC_SCALE` reader - Loop RC scale threshold
pub type RcScaleR = crate::BitReader;
///Field `RC_SCALE` writer - Loop RC scale threshold
pub type RcScaleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST_THRS` reader - hysteres threshold
pub type HystThrsR = crate::BitReader;
///Field `HYST_THRS` writer - hysteres threshold
pub type HystThrsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST_SIGN` reader - hysteres sign
pub type HystSignR = crate::BitReader;
///Field `HYST_SIGN` writer - hysteres sign
pub type HystSignW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_HYST` reader - hysteres enable
pub type EnHystR = crate::BitReader;
///Field `EN_HYST` writer - hysteres enable
pub type EnHystW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable stepping in voltage change 0: stepping disabled 1: steping enabled
    #[inline(always)]
    pub fn en_step(&self) -> EnStepR {
        EnStepR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable delay 0: delay disabled, 1: delay enabled
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - voltage ripple threshold in low power mode 0: 12.5mV 1: 25mV
    #[inline(always)]
    pub fn ol_hyst(&self) -> OlHystR {
        OlHystR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:9 - overload threshold in low power mode
    #[inline(always)]
    pub fn ol_thre(&self) -> OlThreR {
        OlThreR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:18 - Loop feed forward number
    #[inline(always)]
    pub fn dc_ff(&self) -> DcFfR {
        DcFfR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - Loop RC scale threshold
    #[inline(always)]
    pub fn rc_scale(&self) -> RcScaleR {
        RcScaleR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - hysteres threshold
    #[inline(always)]
    pub fn hyst_thrs(&self) -> HystThrsR {
        HystThrsR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - hysteres sign
    #[inline(always)]
    pub fn hyst_sign(&self) -> HystSignR {
        HystSignR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - hysteres enable
    #[inline(always)]
    pub fn en_hyst(&self) -> EnHystR {
        EnHystR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - enable stepping in voltage change 0: stepping disabled 1: steping enabled
    #[inline(always)]
    pub fn en_step(&mut self) -> EnStepW<'_, DcdcMiscSpec> {
        EnStepW::new(self, 0)
    }
    ///Bit 1 - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator
    #[inline(always)]
    pub fn clk_sel(&mut self) -> ClkSelW<'_, DcdcMiscSpec> {
        ClkSelW::new(self, 1)
    }
    ///Bit 2 - enable delay 0: delay disabled, 1: delay enabled
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<'_, DcdcMiscSpec> {
        DelayW::new(self, 2)
    }
    ///Bit 4 - voltage ripple threshold in low power mode 0: 12.5mV 1: 25mV
    #[inline(always)]
    pub fn ol_hyst(&mut self) -> OlHystW<'_, DcdcMiscSpec> {
        OlHystW::new(self, 4)
    }
    ///Bits 8:9 - overload threshold in low power mode
    #[inline(always)]
    pub fn ol_thre(&mut self) -> OlThreW<'_, DcdcMiscSpec> {
        OlThreW::new(self, 8)
    }
    ///Bits 16:18 - Loop feed forward number
    #[inline(always)]
    pub fn dc_ff(&mut self) -> DcFfW<'_, DcdcMiscSpec> {
        DcFfW::new(self, 16)
    }
    ///Bit 20 - Loop RC scale threshold
    #[inline(always)]
    pub fn rc_scale(&mut self) -> RcScaleW<'_, DcdcMiscSpec> {
        RcScaleW::new(self, 20)
    }
    ///Bit 24 - hysteres threshold
    #[inline(always)]
    pub fn hyst_thrs(&mut self) -> HystThrsW<'_, DcdcMiscSpec> {
        HystThrsW::new(self, 24)
    }
    ///Bit 25 - hysteres sign
    #[inline(always)]
    pub fn hyst_sign(&mut self) -> HystSignW<'_, DcdcMiscSpec> {
        HystSignW::new(self, 25)
    }
    ///Bit 28 - hysteres enable
    #[inline(always)]
    pub fn en_hyst(&mut self) -> EnHystW<'_, DcdcMiscSpec> {
        EnHystW::new(self, 28)
    }
}
/**DCDC misc parameter

You can [`read`](crate::Reg::read) this register and get [`dcdc_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcMiscSpec;
impl crate::RegisterSpec for DcdcMiscSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_misc::R`](R) reader structure
impl crate::Readable for DcdcMiscSpec {}
///`write(|w| ..)` method takes [`dcdc_misc::W`](W) writer structure
impl crate::Writable for DcdcMiscSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_MISC to value 0x0007_0100
impl crate::Resettable for DcdcMiscSpec {
    const RESET_VALUE: u32 = 0x0007_0100;
}
