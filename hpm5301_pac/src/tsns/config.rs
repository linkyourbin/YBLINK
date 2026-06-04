///Register `CONFIG` reader
pub type R = crate::R<ConfigSpec>;
///Register `CONFIG` writer
pub type W = crate::W<ConfigSpec>;
///Field `ENABLE` reader - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNC` reader - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode
pub type AsyncR = crate::BitReader;
///Field `ASYNC` writer - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode
pub type AsyncW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONTINUOUS` reader - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode
pub type ContinuousR = crate::BitReader;
///Field `CONTINUOUS` writer - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode
pub type ContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVERAGE` reader - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average
pub type AverageR = crate::FieldReader;
///Field `AVERAGE` writer - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average
pub type AverageW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPEED` reader - cycles of a progressive step in 24M clock, valid from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step
pub type SpeedR = crate::FieldReader;
///Field `SPEED` writer - cycles of a progressive step in 24M clock, valid from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COMPARE_MAX_EN` reader - Enable compare for maximum temperature
pub type CompareMaxEnR = crate::BitReader;
///Field `COMPARE_MAX_EN` writer - Enable compare for maximum temperature
pub type CompareMaxEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPARE_MIN_EN` reader - Enable compare for minimum temperature
pub type CompareMinEnR = crate::BitReader;
///Field `COMPARE_MIN_EN` writer - Enable compare for minimum temperature
pub type CompareMinEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_EN` reader - Enable reset
pub type RstEnR = crate::BitReader;
///Field `RST_EN` writer - Enable reset
pub type RstEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQ_EN` reader - Enable interrupt
pub type IrqEnR = crate::BitReader;
///Field `IRQ_EN` writer - Enable interrupt
pub type IrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode
    #[inline(always)]
    pub fn async_(&self) -> AsyncR {
        AsyncR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode
    #[inline(always)]
    pub fn continuous(&self) -> ContinuousR {
        ContinuousR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:10 - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average
    #[inline(always)]
    pub fn average(&self) -> AverageR {
        AverageR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:23 - cycles of a progressive step in 24M clock, valid from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Enable compare for maximum temperature
    #[inline(always)]
    pub fn compare_max_en(&self) -> CompareMaxEnR {
        CompareMaxEnR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Enable compare for minimum temperature
    #[inline(always)]
    pub fn compare_min_en(&self) -> CompareMinEnR {
        CompareMinEnR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - Enable reset
    #[inline(always)]
    pub fn rst_en(&self) -> RstEnR {
        RstEnR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Enable interrupt
    #[inline(always)]
    pub fn irq_en(&self) -> IrqEnR {
        IrqEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ConfigSpec> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode
    #[inline(always)]
    pub fn async_(&mut self) -> AsyncW<'_, ConfigSpec> {
        AsyncW::new(self, 1)
    }
    ///Bit 4 - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode
    #[inline(always)]
    pub fn continuous(&mut self) -> ContinuousW<'_, ConfigSpec> {
        ContinuousW::new(self, 4)
    }
    ///Bits 8:10 - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average
    #[inline(always)]
    pub fn average(&mut self) -> AverageW<'_, ConfigSpec> {
        AverageW::new(self, 8)
    }
    ///Bits 16:23 - cycles of a progressive step in 24M clock, valid from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, ConfigSpec> {
        SpeedW::new(self, 16)
    }
    ///Bit 24 - Enable compare for maximum temperature
    #[inline(always)]
    pub fn compare_max_en(&mut self) -> CompareMaxEnW<'_, ConfigSpec> {
        CompareMaxEnW::new(self, 24)
    }
    ///Bit 25 - Enable compare for minimum temperature
    #[inline(always)]
    pub fn compare_min_en(&mut self) -> CompareMinEnW<'_, ConfigSpec> {
        CompareMinEnW::new(self, 25)
    }
    ///Bit 30 - Enable reset
    #[inline(always)]
    pub fn rst_en(&mut self) -> RstEnW<'_, ConfigSpec> {
        RstEnW::new(self, 30)
    }
    ///Bit 31 - Enable interrupt
    #[inline(always)]
    pub fn irq_en(&mut self) -> IrqEnW<'_, ConfigSpec> {
        IrqEnW::new(self, 31)
    }
}
/**Configuration

You can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
///`read()` method returns [`config::R`](R) reader structure
impl crate::Readable for ConfigSpec {}
///`write(|w| ..)` method takes [`config::W`](W) writer structure
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFIG to value 0x0060_0300
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x0060_0300;
}
