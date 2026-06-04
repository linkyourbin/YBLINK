///Register `cfg` reader
pub type R = crate::R<CfgSpec>;
///Register `cfg` writer
pub type W = crate::W<CfgSpec>;
///Field `FLTLEN` reader - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle.
pub type FltlenR = crate::FieldReader<u16>;
///Field `FLTLEN` writer - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle.
pub type FltlenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `FLTMODE` reader - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high
pub type FltmodeR = crate::FieldReader;
///Field `FLTMODE` writer - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high
pub type FltmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OPOL` reader - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted.
pub type OpolR = crate::BitReader;
///Field `OPOL` writer - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted.
pub type OpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WINEN` reader - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled
pub type WinenR = crate::BitReader;
///Field `WINEN` writer - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled
pub type WinenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTBYPS` reader - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed.
pub type FltbypsR = crate::BitReader;
///Field `FLTBYPS` writer - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed.
pub type FltbypsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPOEN` reader - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled
pub type CmpoenR = crate::BitReader;
///Field `CMPOEN` writer - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled
pub type CmpoenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINSEL` reader - MIN select, from pad_ai_acmp\[7:1\] and dac_out
pub type PinselR = crate::FieldReader;
///Field `PINSEL` writer - MIN select, from pad_ai_acmp\[7:1\] and dac_out
pub type PinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DAC_TRIG_EN` reader - if set, the dac value is from moto system when valid if clr, use dac_cfg value
pub type DacTrigEnR = crate::BitReader;
///Field `DAC_TRIG_EN` writer - if set, the dac value is from moto system when valid if clr, use dac_cfg value
pub type DacTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINSEL` reader - PIN select, from pad_ai_acmp\[7:1\] and dac_out
pub type MinselR = crate::FieldReader;
///Field `MINSEL` writer - PIN select, from pad_ai_acmp\[7:1\] and dac_out
pub type MinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CMPEN` reader - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled
pub type CmpenR = crate::BitReader;
///Field `CMPEN` writer - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPMODE` reader - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled
pub type HpmodeR = crate::BitReader;
///Field `HPMODE` writer - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled
pub type HpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACEN` reader - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled
pub type DacenR = crate::BitReader;
///Field `DACEN` writer - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - This bitfield configure the comparator hysteresis. 0: Hysteresis about 30mV 1: Hysteresis about 20mV 2: Hysteresis about 10mV 3: Disable hysteresis
pub type HystR = crate::FieldReader;
///Field `HYST` writer - This bitfield configure the comparator hysteresis. 0: Hysteresis about 30mV 1: Hysteresis about 20mV 2: Hysteresis about 10mV 3: Disable hysteresis
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:11 - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle.
    #[inline(always)]
    pub fn fltlen(&self) -> FltlenR {
        FltlenR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 13:15 - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high
    #[inline(always)]
    pub fn fltmode(&self) -> FltmodeR {
        FltmodeR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted.
    #[inline(always)]
    pub fn opol(&self) -> OpolR {
        OpolR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled
    #[inline(always)]
    pub fn winen(&self) -> WinenR {
        WinenR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed.
    #[inline(always)]
    pub fn fltbyps(&self) -> FltbypsR {
        FltbypsR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled
    #[inline(always)]
    pub fn cmpoen(&self) -> CmpoenR {
        CmpoenR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:22 - MIN select, from pad_ai_acmp\[7:1\] and dac_out
    #[inline(always)]
    pub fn pinsel(&self) -> PinselR {
        PinselR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - if set, the dac value is from moto system when valid if clr, use dac_cfg value
    #[inline(always)]
    pub fn dac_trig_en(&self) -> DacTrigEnR {
        DacTrigEnR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - PIN select, from pad_ai_acmp\[7:1\] and dac_out
    #[inline(always)]
    pub fn minsel(&self) -> MinselR {
        MinselR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled
    #[inline(always)]
    pub fn hpmode(&self) -> HpmodeR {
        HpmodeR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - This bitfield configure the comparator hysteresis. 0: Hysteresis about 30mV 1: Hysteresis about 20mV 2: Hysteresis about 10mV 3: Disable hysteresis
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:11 - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle.
    #[inline(always)]
    pub fn fltlen(&mut self) -> FltlenW<'_, CfgSpec> {
        FltlenW::new(self, 0)
    }
    ///Bits 13:15 - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high
    #[inline(always)]
    pub fn fltmode(&mut self) -> FltmodeW<'_, CfgSpec> {
        FltmodeW::new(self, 13)
    }
    ///Bit 16 - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted.
    #[inline(always)]
    pub fn opol(&mut self) -> OpolW<'_, CfgSpec> {
        OpolW::new(self, 16)
    }
    ///Bit 17 - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled
    #[inline(always)]
    pub fn winen(&mut self) -> WinenW<'_, CfgSpec> {
        WinenW::new(self, 17)
    }
    ///Bit 18 - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed.
    #[inline(always)]
    pub fn fltbyps(&mut self) -> FltbypsW<'_, CfgSpec> {
        FltbypsW::new(self, 18)
    }
    ///Bit 19 - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled
    #[inline(always)]
    pub fn cmpoen(&mut self) -> CmpoenW<'_, CfgSpec> {
        CmpoenW::new(self, 19)
    }
    ///Bits 20:22 - MIN select, from pad_ai_acmp\[7:1\] and dac_out
    #[inline(always)]
    pub fn pinsel(&mut self) -> PinselW<'_, CfgSpec> {
        PinselW::new(self, 20)
    }
    ///Bit 23 - if set, the dac value is from moto system when valid if clr, use dac_cfg value
    #[inline(always)]
    pub fn dac_trig_en(&mut self) -> DacTrigEnW<'_, CfgSpec> {
        DacTrigEnW::new(self, 23)
    }
    ///Bits 24:26 - PIN select, from pad_ai_acmp\[7:1\] and dac_out
    #[inline(always)]
    pub fn minsel(&mut self) -> MinselW<'_, CfgSpec> {
        MinselW::new(self, 24)
    }
    ///Bit 27 - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled
    #[inline(always)]
    pub fn cmpen(&mut self) -> CmpenW<'_, CfgSpec> {
        CmpenW::new(self, 27)
    }
    ///Bit 28 - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled
    #[inline(always)]
    pub fn hpmode(&mut self) -> HpmodeW<'_, CfgSpec> {
        HpmodeW::new(self, 28)
    }
    ///Bit 29 - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled
    #[inline(always)]
    pub fn dacen(&mut self) -> DacenW<'_, CfgSpec> {
        DacenW::new(self, 29)
    }
    ///Bits 30:31 - This bitfield configure the comparator hysteresis. 0: Hysteresis about 30mV 1: Hysteresis about 20mV 2: Hysteresis about 10mV 3: Disable hysteresis
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, CfgSpec> {
        HystW::new(self, 30)
    }
}
/**Configure Register

You can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfg::R`](R) reader structure
impl crate::Readable for CfgSpec {}
///`write(|w| ..)` method takes [`cfg::W`](W) writer structure
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets cfg to value 0x20
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x20;
}
