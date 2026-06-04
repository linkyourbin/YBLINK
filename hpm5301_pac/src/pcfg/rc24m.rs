///Register `RC24M` reader
pub type R = crate::R<Rc24mSpec>;
///Register `RC24M` writer
pub type W = crate::W<Rc24mSpec>;
///Field `TRIM_F` reader - Fine trim for RC24M, bigger value means faster
pub type TrimFR = crate::FieldReader;
///Field `TRIM_F` writer - Fine trim for RC24M, bigger value means faster
pub type TrimFW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRIM_C` reader - Coarse trim for RC24M, bigger value means faster
pub type TrimCR = crate::FieldReader;
///Field `TRIM_C` writer - Coarse trim for RC24M, bigger value means faster
pub type TrimCW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RC_TRIMMED` reader - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed
pub type RcTrimmedR = crate::BitReader;
///Field `RC_TRIMMED` writer - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed
pub type RcTrimmedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Fine trim for RC24M, bigger value means faster
    #[inline(always)]
    pub fn trim_f(&self) -> TrimFR {
        TrimFR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:10 - Coarse trim for RC24M, bigger value means faster
    #[inline(always)]
    pub fn trim_c(&self) -> TrimCR {
        TrimCR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 31 - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed
    #[inline(always)]
    pub fn rc_trimmed(&self) -> RcTrimmedR {
        RcTrimmedR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Fine trim for RC24M, bigger value means faster
    #[inline(always)]
    pub fn trim_f(&mut self) -> TrimFW<'_, Rc24mSpec> {
        TrimFW::new(self, 0)
    }
    ///Bits 8:10 - Coarse trim for RC24M, bigger value means faster
    #[inline(always)]
    pub fn trim_c(&mut self) -> TrimCW<'_, Rc24mSpec> {
        TrimCW::new(self, 8)
    }
    ///Bit 31 - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed
    #[inline(always)]
    pub fn rc_trimmed(&mut self) -> RcTrimmedW<'_, Rc24mSpec> {
        RcTrimmedW::new(self, 31)
    }
}
/**RC 24M config

You can [`read`](crate::Reg::read) this register and get [`rc24m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc24m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rc24mSpec;
impl crate::RegisterSpec for Rc24mSpec {
    type Ux = u32;
}
///`read()` method returns [`rc24m::R`](R) reader structure
impl crate::Readable for Rc24mSpec {}
///`write(|w| ..)` method takes [`rc24m::W`](W) writer structure
impl crate::Writable for Rc24mSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RC24M to value 0x0310
impl crate::Resettable for Rc24mSpec {
    const RESET_VALUE: u32 = 0x0310;
}
