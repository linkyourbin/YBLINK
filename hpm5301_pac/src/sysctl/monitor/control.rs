///Register `control` reader
pub type R = crate::R<ControlSpec>;
///Register `control` writer
pub type W = crate::W<ControlSpec>;
///Field `SELECTION` reader - clock measurement selection
pub type SelectionR = crate::FieldReader;
///Field `SELECTION` writer - clock measurement selection
pub type SelectionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `REFERENCE` reader - reference clock selection, 0: 32k 1: 24M
pub type ReferenceR = crate::BitReader;
///Field `REFERENCE` writer - reference clock selection, 0: 32k 1: 24M
pub type ReferenceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACCURACY` reader - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz
pub type AccuracyR = crate::BitReader;
///Field `ACCURACY` writer - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz
pub type AccuracyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register
pub type ModeR = crate::BitReader;
///Field `MODE` writer - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - start measurement
pub type StartR = crate::BitReader;
///Field `START` writer - start measurement
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOW` reader - clock frequency lower than lower limit
pub type LowR = crate::BitReader;
///Field `LOW` writer - clock frequency lower than lower limit
pub type LowW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HIGH` reader - clock frequency higher than upper limit
pub type HighR = crate::BitReader;
///Field `HIGH` writer - clock frequency higher than upper limit
pub type HighW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIV` reader - output divider
pub type DivR = crate::FieldReader;
///Field `DIV` writer - output divider
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `OUTEN` reader - enable clock output
pub type OutenR = crate::BitReader;
///Field `OUTEN` writer - enable clock output
pub type OutenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIV_BUSY` reader - divider is applying new setting
pub type DivBusyR = crate::BitReader;
///Field `VALID` reader - result is ready for read 0: not ready 1: result is ready
pub type ValidR = crate::BitReader;
///Field `VALID` writer - result is ready for read 0: not ready 1: result is ready
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - clock measurement selection
    #[inline(always)]
    pub fn selection(&self) -> SelectionR {
        SelectionR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - reference clock selection, 0: 32k 1: 24M
    #[inline(always)]
    pub fn reference(&self) -> ReferenceR {
        ReferenceR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz
    #[inline(always)]
    pub fn accuracy(&self) -> AccuracyR {
        AccuracyR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - start measurement
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - clock frequency lower than lower limit
    #[inline(always)]
    pub fn low(&self) -> LowR {
        LowR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - clock frequency higher than upper limit
    #[inline(always)]
    pub fn high(&self) -> HighR {
        HighR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - output divider
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - enable clock output
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 27 - divider is applying new setting
    #[inline(always)]
    pub fn div_busy(&self) -> DivBusyR {
        DivBusyR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - result is ready for read 0: not ready 1: result is ready
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - clock measurement selection
    #[inline(always)]
    pub fn selection(&mut self) -> SelectionW<'_, ControlSpec> {
        SelectionW::new(self, 0)
    }
    ///Bit 8 - reference clock selection, 0: 32k 1: 24M
    #[inline(always)]
    pub fn reference(&mut self) -> ReferenceW<'_, ControlSpec> {
        ReferenceW::new(self, 8)
    }
    ///Bit 9 - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz
    #[inline(always)]
    pub fn accuracy(&mut self) -> AccuracyW<'_, ControlSpec> {
        AccuracyW::new(self, 9)
    }
    ///Bit 10 - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ControlSpec> {
        ModeW::new(self, 10)
    }
    ///Bit 12 - start measurement
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, ControlSpec> {
        StartW::new(self, 12)
    }
    ///Bit 14 - clock frequency lower than lower limit
    #[inline(always)]
    pub fn low(&mut self) -> LowW<'_, ControlSpec> {
        LowW::new(self, 14)
    }
    ///Bit 15 - clock frequency higher than upper limit
    #[inline(always)]
    pub fn high(&mut self) -> HighW<'_, ControlSpec> {
        HighW::new(self, 15)
    }
    ///Bits 16:23 - output divider
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, ControlSpec> {
        DivW::new(self, 16)
    }
    ///Bit 24 - enable clock output
    #[inline(always)]
    pub fn outen(&mut self) -> OutenW<'_, ControlSpec> {
        OutenW::new(self, 24)
    }
    ///Bit 31 - result is ready for read 0: not ready 1: result is ready
    #[inline(always)]
    pub fn valid(&mut self) -> ValidW<'_, ControlSpec> {
        ValidW::new(self, 31)
    }
}
/**Clock measure and monitor control

You can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
///`read()` method returns [`control::R`](R) reader structure
impl crate::Readable for ControlSpec {}
///`write(|w| ..)` method takes [`control::W`](W) writer structure
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets control to value 0
impl crate::Resettable for ControlSpec {}
