///Register `control` reader
pub type R = crate::R<ControlSpec>;
///Register `control` writer
pub type W = crate::W<ControlSpec>;
///Field `RESET` reader - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically
pub type ResetR = crate::BitReader;
///Field `RESET` writer - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD` reader - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold
pub type HoldR = crate::BitReader;
///Field `HOLD` writer - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold
pub type HoldW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLAG_WAKE` reader - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit
pub type FlagWakeR = crate::BitReader;
///Field `FLAG_WAKE` writer - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit
pub type FlagWakeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLAG` reader - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit
pub type FlagR = crate::BitReader;
///Field `FLAG` writer - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit
pub type FlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 30 - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit
    #[inline(always)]
    pub fn flag_wake(&self) -> FlagWakeR {
        FlagWakeR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, ControlSpec> {
        ResetW::new(self, 0)
    }
    ///Bit 4 - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<'_, ControlSpec> {
        HoldW::new(self, 4)
    }
    ///Bit 30 - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit
    #[inline(always)]
    pub fn flag_wake(&mut self) -> FlagWakeW<'_, ControlSpec> {
        FlagWakeW::new(self, 30)
    }
    ///Bit 31 - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, ControlSpec> {
        FlagW::new(self, 31)
    }
}
/**Reset Setting

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
///`reset()` method sets control to value 0x8000_0000
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
