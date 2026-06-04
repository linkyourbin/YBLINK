///Register `DCDC_ADVMODE` reader
pub type R = crate::R<DcdcAdvmodeSpec>;
///Register `DCDC_ADVMODE` writer
pub type W = crate::W<DcdcAdvmodeSpec>;
///Field `EN_DCM` reader - DCM mode 0: CCM mode 1: DCM mode
pub type EnDcmR = crate::BitReader;
///Field `EN_DCM` writer - DCM mode 0: CCM mode 1: DCM mode
pub type EnDcmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_IDLE` reader - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess
pub type EnIdleR = crate::BitReader;
///Field `EN_IDLE` writer - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess
pub type EnIdleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_SKIP` reader - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse
pub type EnSkipR = crate::BitReader;
///Field `EN_SKIP` writer - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse
pub type EnSkipW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_DCM_EXIT` reader - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess
pub type EnDcmExitR = crate::BitReader;
///Field `EN_DCM_EXIT` writer - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess
pub type EnDcmExitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_AUTOLP` reader - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low
pub type EnAutolpR = crate::BitReader;
///Field `EN_AUTOLP` writer - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low
pub type EnAutolpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_FF_LOOP` reader - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled
pub type EnFfLoopR = crate::BitReader;
///Field `EN_FF_LOOP` writer - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled
pub type EnFfLoopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_FF_DET` reader - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled
pub type EnFfDetR = crate::BitReader;
///Field `EN_FF_DET` writer - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled
pub type EnFfDetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_R` reader - Loop R number
pub type DcRR = crate::FieldReader;
///Field `DC_R` writer - Loop R number
pub type DcRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DC_C` reader - Loop C number
pub type DcCR = crate::FieldReader;
///Field `DC_C` writer - Loop C number
pub type DcCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EN_RCSCALE` reader - Enable RC scale
pub type EnRcscaleR = crate::FieldReader;
///Field `EN_RCSCALE` writer - Enable RC scale
pub type EnRcscaleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - DCM mode 0: CCM mode 1: DCM mode
    #[inline(always)]
    pub fn en_dcm(&self) -> EnDcmR {
        EnDcmR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess
    #[inline(always)]
    pub fn en_idle(&self) -> EnIdleR {
        EnIdleR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse
    #[inline(always)]
    pub fn en_skip(&self) -> EnSkipR {
        EnSkipR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess
    #[inline(always)]
    pub fn en_dcm_exit(&self) -> EnDcmExitR {
        EnDcmExitR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low
    #[inline(always)]
    pub fn en_autolp(&self) -> EnAutolpR {
        EnAutolpR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled
    #[inline(always)]
    pub fn en_ff_loop(&self) -> EnFfLoopR {
        EnFfLoopR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled
    #[inline(always)]
    pub fn en_ff_det(&self) -> EnFfDetR {
        EnFfDetR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:19 - Loop R number
    #[inline(always)]
    pub fn dc_r(&self) -> DcRR {
        DcRR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - Loop C number
    #[inline(always)]
    pub fn dc_c(&self) -> DcCR {
        DcCR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:26 - Enable RC scale
    #[inline(always)]
    pub fn en_rcscale(&self) -> EnRcscaleR {
        EnRcscaleR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - DCM mode 0: CCM mode 1: DCM mode
    #[inline(always)]
    pub fn en_dcm(&mut self) -> EnDcmW<'_, DcdcAdvmodeSpec> {
        EnDcmW::new(self, 0)
    }
    ///Bit 1 - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess
    #[inline(always)]
    pub fn en_idle(&mut self) -> EnIdleW<'_, DcdcAdvmodeSpec> {
        EnIdleW::new(self, 1)
    }
    ///Bit 2 - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse
    #[inline(always)]
    pub fn en_skip(&mut self) -> EnSkipW<'_, DcdcAdvmodeSpec> {
        EnSkipW::new(self, 2)
    }
    ///Bit 3 - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess
    #[inline(always)]
    pub fn en_dcm_exit(&mut self) -> EnDcmExitW<'_, DcdcAdvmodeSpec> {
        EnDcmExitW::new(self, 3)
    }
    ///Bit 4 - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low
    #[inline(always)]
    pub fn en_autolp(&mut self) -> EnAutolpW<'_, DcdcAdvmodeSpec> {
        EnAutolpW::new(self, 4)
    }
    ///Bit 5 - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled
    #[inline(always)]
    pub fn en_ff_loop(&mut self) -> EnFfLoopW<'_, DcdcAdvmodeSpec> {
        EnFfLoopW::new(self, 5)
    }
    ///Bit 6 - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled
    #[inline(always)]
    pub fn en_ff_det(&mut self) -> EnFfDetW<'_, DcdcAdvmodeSpec> {
        EnFfDetW::new(self, 6)
    }
    ///Bits 16:19 - Loop R number
    #[inline(always)]
    pub fn dc_r(&mut self) -> DcRW<'_, DcdcAdvmodeSpec> {
        DcRW::new(self, 16)
    }
    ///Bits 20:21 - Loop C number
    #[inline(always)]
    pub fn dc_c(&mut self) -> DcCW<'_, DcdcAdvmodeSpec> {
        DcCW::new(self, 20)
    }
    ///Bits 24:26 - Enable RC scale
    #[inline(always)]
    pub fn en_rcscale(&mut self) -> EnRcscaleW<'_, DcdcAdvmodeSpec> {
        EnRcscaleW::new(self, 24)
    }
}
/**DCDC advance setting

You can [`read`](crate::Reg::read) this register and get [`dcdc_advmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_advmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcdcAdvmodeSpec;
impl crate::RegisterSpec for DcdcAdvmodeSpec {
    type Ux = u32;
}
///`read()` method returns [`dcdc_advmode::R`](R) reader structure
impl crate::Readable for DcdcAdvmodeSpec {}
///`write(|w| ..)` method takes [`dcdc_advmode::W`](W) writer structure
impl crate::Writable for DcdcAdvmodeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCDC_ADVMODE to value 0x0312_0040
impl crate::Resettable for DcdcAdvmodeSpec {
    const RESET_VALUE: u32 = 0x0312_0040;
}
