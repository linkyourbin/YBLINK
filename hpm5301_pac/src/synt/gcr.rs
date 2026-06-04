///Register `gcr` reader
pub type R = crate::R<GcrSpec>;
///Register `gcr` writer
pub type W = crate::W<GcrSpec>;
///Field `CEN` reader - 1- Enable counter
pub type CenR = crate::BitReader;
///Field `CEN` writer - 1- Enable counter
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRST` reader - 1- Reset counter
pub type CrstR = crate::BitReader;
///Field `CRST` writer - 1- Reset counter
pub type CrstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTER_DEBUG_EN` reader - set to enable cpu_debug_mode to stop the counter
pub type CounterDebugEnR = crate::BitReader;
///Field `COUNTER_DEBUG_EN` writer - set to enable cpu_debug_mode to stop the counter
pub type CounterDebugEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMESTAMP_ENABLE` reader - set to enable the timesamp , clr to stop
pub type TimestampEnableR = crate::BitReader;
///Field `TIMESTAMP_ENABLE` writer - set to enable the timesamp , clr to stop
pub type TimestampEnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMESTAMP_DEBUG_EN` reader - set to enable cpu_debug_mode to stop the timesamp
pub type TimestampDebugEnR = crate::BitReader;
///Field `TIMESTAMP_DEBUG_EN` writer - set to enable cpu_debug_mode to stop the timesamp
pub type TimestampDebugEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMESTAMP_RESET` writer - reset timesamp to 0, auto clr
pub type TimestampResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMESTAMP_SET_NEW` writer - set the timesamp to new value, auto clr
pub type TimestampSetNewW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMESTAMP_DEC_NEW` writer - set to decrease the timesamp with new value, auto clr
pub type TimestampDecNewW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMESTAMP_INC_NEW` writer - set to increase the timesamp with new value, auto clr
pub type TimestampIncNewW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1- Enable counter
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1- Reset counter
    #[inline(always)]
    pub fn crst(&self) -> CrstR {
        CrstR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - set to enable cpu_debug_mode to stop the counter
    #[inline(always)]
    pub fn counter_debug_en(&self) -> CounterDebugEnR {
        CounterDebugEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - set to enable the timesamp , clr to stop
    #[inline(always)]
    pub fn timestamp_enable(&self) -> TimestampEnableR {
        TimestampEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - set to enable cpu_debug_mode to stop the timesamp
    #[inline(always)]
    pub fn timestamp_debug_en(&self) -> TimestampDebugEnR {
        TimestampDebugEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - 1- Enable counter
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, GcrSpec> {
        CenW::new(self, 0)
    }
    ///Bit 1 - 1- Reset counter
    #[inline(always)]
    pub fn crst(&mut self) -> CrstW<'_, GcrSpec> {
        CrstW::new(self, 1)
    }
    ///Bit 2 - set to enable cpu_debug_mode to stop the counter
    #[inline(always)]
    pub fn counter_debug_en(&mut self) -> CounterDebugEnW<'_, GcrSpec> {
        CounterDebugEnW::new(self, 2)
    }
    ///Bit 4 - set to enable the timesamp , clr to stop
    #[inline(always)]
    pub fn timestamp_enable(&mut self) -> TimestampEnableW<'_, GcrSpec> {
        TimestampEnableW::new(self, 4)
    }
    ///Bit 5 - set to enable cpu_debug_mode to stop the timesamp
    #[inline(always)]
    pub fn timestamp_debug_en(&mut self) -> TimestampDebugEnW<'_, GcrSpec> {
        TimestampDebugEnW::new(self, 5)
    }
    ///Bit 28 - reset timesamp to 0, auto clr
    #[inline(always)]
    pub fn timestamp_reset(&mut self) -> TimestampResetW<'_, GcrSpec> {
        TimestampResetW::new(self, 28)
    }
    ///Bit 29 - set the timesamp to new value, auto clr
    #[inline(always)]
    pub fn timestamp_set_new(&mut self) -> TimestampSetNewW<'_, GcrSpec> {
        TimestampSetNewW::new(self, 29)
    }
    ///Bit 30 - set to decrease the timesamp with new value, auto clr
    #[inline(always)]
    pub fn timestamp_dec_new(&mut self) -> TimestampDecNewW<'_, GcrSpec> {
        TimestampDecNewW::new(self, 30)
    }
    ///Bit 31 - set to increase the timesamp with new value, auto clr
    #[inline(always)]
    pub fn timestamp_inc_new(&mut self) -> TimestampIncNewW<'_, GcrSpec> {
        TimestampIncNewW::new(self, 31)
    }
}
/**Global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GcrSpec;
impl crate::RegisterSpec for GcrSpec {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GcrSpec {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets gcr to value 0
impl crate::Resettable for GcrSpec {}
