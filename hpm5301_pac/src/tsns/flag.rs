///Register `FLAG` reader
pub type R = crate::R<FlagSpec>;
///Register `FLAG` writer
pub type W = crate::W<FlagSpec>;
///Field `IRQ` reader - IRQ flag, write 1 to clear
pub type IrqR = crate::BitReader;
///Field `IRQ` writer - IRQ flag, write 1 to clear
pub type IrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVER_TEMP` reader - Clear over temperature status, write 1 to clear
pub type OverTempR = crate::BitReader;
///Field `OVER_TEMP` writer - Clear over temperature status, write 1 to clear
pub type OverTempW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNDER_TEMP` reader - Clear under temperature status, write 1 to clear
pub type UnderTempR = crate::BitReader;
///Field `UNDER_TEMP` writer - Clear under temperature status, write 1 to clear
pub type UnderTempW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RECORD_MAX_CLR` reader - Clear maximum recorder of temerature, write 1 to clear
pub type RecordMaxClrR = crate::BitReader;
///Field `RECORD_MAX_CLR` writer - Clear maximum recorder of temerature, write 1 to clear
pub type RecordMaxClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RECORD_MIN_CLR` reader - Clear minimum recorder of temerature, write 1 to clear
pub type RecordMinClrR = crate::BitReader;
///Field `RECORD_MIN_CLR` writer - Clear minimum recorder of temerature, write 1 to clear
pub type RecordMinClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IRQ flag, write 1 to clear
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Clear over temperature status, write 1 to clear
    #[inline(always)]
    pub fn over_temp(&self) -> OverTempR {
        OverTempR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Clear under temperature status, write 1 to clear
    #[inline(always)]
    pub fn under_temp(&self) -> UnderTempR {
        UnderTempR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Clear maximum recorder of temerature, write 1 to clear
    #[inline(always)]
    pub fn record_max_clr(&self) -> RecordMaxClrR {
        RecordMaxClrR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Clear minimum recorder of temerature, write 1 to clear
    #[inline(always)]
    pub fn record_min_clr(&self) -> RecordMinClrR {
        RecordMinClrR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ flag, write 1 to clear
    #[inline(always)]
    pub fn irq(&mut self) -> IrqW<'_, FlagSpec> {
        IrqW::new(self, 0)
    }
    ///Bit 16 - Clear over temperature status, write 1 to clear
    #[inline(always)]
    pub fn over_temp(&mut self) -> OverTempW<'_, FlagSpec> {
        OverTempW::new(self, 16)
    }
    ///Bit 17 - Clear under temperature status, write 1 to clear
    #[inline(always)]
    pub fn under_temp(&mut self) -> UnderTempW<'_, FlagSpec> {
        UnderTempW::new(self, 17)
    }
    ///Bit 20 - Clear maximum recorder of temerature, write 1 to clear
    #[inline(always)]
    pub fn record_max_clr(&mut self) -> RecordMaxClrW<'_, FlagSpec> {
        RecordMaxClrW::new(self, 20)
    }
    ///Bit 21 - Clear minimum recorder of temerature, write 1 to clear
    #[inline(always)]
    pub fn record_min_clr(&mut self) -> RecordMinClrW<'_, FlagSpec> {
        RecordMinClrW::new(self, 21)
    }
}
/**Temperature flag

You can [`read`](crate::Reg::read) this register and get [`flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FlagSpec;
impl crate::RegisterSpec for FlagSpec {
    type Ux = u32;
}
///`read()` method returns [`flag::R`](R) reader structure
impl crate::Readable for FlagSpec {}
///`write(|w| ..)` method takes [`flag::W`](W) writer structure
impl crate::Writable for FlagSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLAG to value 0
impl crate::Resettable for FlagSpec {}
