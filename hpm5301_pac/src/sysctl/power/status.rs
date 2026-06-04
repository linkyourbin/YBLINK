///Register `status` reader
pub type R = crate::R<StatusSpec>;
///Register `status` writer
pub type W = crate::W<StatusSpec>;
///Field `LF_ACK` reader - low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off
pub type LfAckR = crate::BitReader;
///Field `LF_DISABLE` reader - low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off
pub type LfDisableR = crate::BitReader;
///Field `MEM_RET_P` reader - memory info retention control signal 0: memory not enterexitretention mode 1: memory enter retention mode
pub type MemRetPR = crate::BitReader;
///Field `MEM_RET_N` reader - memory info retention control signal 0: memory enter retention mode 1: memory exit retention mode
pub type MemRetNR = crate::BitReader;
///Field `FLAG_WAKE` reader - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit
pub type FlagWakeR = crate::BitReader;
///Field `FLAG_WAKE` writer - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit
pub type FlagWakeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLAG` reader - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit
pub type FlagR = crate::BitReader;
///Field `FLAG` writer - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit
pub type FlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off
    #[inline(always)]
    pub fn lf_ack(&self) -> LfAckR {
        LfAckR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off
    #[inline(always)]
    pub fn lf_disable(&self) -> LfDisableR {
        LfDisableR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - memory info retention control signal 0: memory not enterexitretention mode 1: memory enter retention mode
    #[inline(always)]
    pub fn mem_ret_p(&self) -> MemRetPR {
        MemRetPR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - memory info retention control signal 0: memory enter retention mode 1: memory exit retention mode
    #[inline(always)]
    pub fn mem_ret_n(&self) -> MemRetNR {
        MemRetNR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 30 - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit
    #[inline(always)]
    pub fn flag_wake(&self) -> FlagWakeR {
        FlagWakeR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 30 - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit
    #[inline(always)]
    pub fn flag_wake(&mut self) -> FlagWakeW<'_, StatusSpec> {
        FlagWakeW::new(self, 30)
    }
    ///Bit 31 - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, StatusSpec> {
        FlagW::new(self, 31)
    }
}
/**Power Setting

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for StatusSpec {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets status to value 0x8000_0000
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
