///Register `LP` reader
pub type R = crate::R<LpSpec>;
///Register `LP` writer
pub type W = crate::W<LpSpec>;
///Field `MODE` reader - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved
pub type ModeR = crate::FieldReader;
///Field `MODE` writer - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RESET_FLAG` reader - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened
pub type ResetFlagR = crate::BitReader;
///Field `RESET_FLAG` writer - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened
pub type ResetFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_FLAG` reader - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened
pub type SleepFlagR = crate::BitReader;
///Field `SLEEP_FLAG` writer - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened
pub type SleepFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAKE_FLAG` reader - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened
pub type WakeFlagR = crate::BitReader;
///Field `WAKE_FLAG` writer - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened
pub type WakeFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXEC` reader - CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing
pub type ExecR = crate::BitReader;
///Field `WAKE` reader - CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted
pub type WakeR = crate::BitReader;
///Field `HALT` reader - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI
pub type HaltR = crate::BitReader;
///Field `HALT` writer - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAKE_CNT` reader - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear
pub type WakeCntR = crate::FieldReader;
///Field `WAKE_CNT` writer - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear
pub type WakeCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    ///Bit 8 - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened
    #[inline(always)]
    pub fn reset_flag(&self) -> ResetFlagR {
        ResetFlagR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened
    #[inline(always)]
    pub fn sleep_flag(&self) -> SleepFlagR {
        SleepFlagR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened
    #[inline(always)]
    pub fn wake_flag(&self) -> WakeFlagR {
        WakeFlagR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing
    #[inline(always)]
    pub fn exec(&self) -> ExecR {
        ExecR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 24:31 - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear
    #[inline(always)]
    pub fn wake_cnt(&self) -> WakeCntR {
        WakeCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:1 - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, LpSpec> {
        ModeW::new(self, 0)
    }
    ///Bit 8 - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened
    #[inline(always)]
    pub fn reset_flag(&mut self) -> ResetFlagW<'_, LpSpec> {
        ResetFlagW::new(self, 8)
    }
    ///Bit 9 - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened
    #[inline(always)]
    pub fn sleep_flag(&mut self) -> SleepFlagW<'_, LpSpec> {
        SleepFlagW::new(self, 9)
    }
    ///Bit 10 - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened
    #[inline(always)]
    pub fn wake_flag(&mut self) -> WakeFlagW<'_, LpSpec> {
        WakeFlagW::new(self, 10)
    }
    ///Bit 16 - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<'_, LpSpec> {
        HaltW::new(self, 16)
    }
    ///Bits 24:31 - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear
    #[inline(always)]
    pub fn wake_cnt(&mut self) -> WakeCntW<'_, LpSpec> {
        WakeCntW::new(self, 24)
    }
}
/**CPU0 LP control

You can [`read`](crate::Reg::read) this register and get [`lp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpSpec;
impl crate::RegisterSpec for LpSpec {
    type Ux = u32;
}
///`read()` method returns [`lp::R`](R) reader structure
impl crate::Readable for LpSpec {}
///`write(|w| ..)` method takes [`lp::W`](W) writer structure
impl crate::Writable for LpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LP to value 0x1000
impl crate::Resettable for LpSpec {
    const RESET_VALUE: u32 = 0x1000;
}
