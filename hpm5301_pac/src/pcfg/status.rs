///Register `STATUS` reader
pub type R = crate::R<StatusSpec>;
///Register `STATUS` writer
pub type W = crate::W<StatusSpec>;
///Field `TRIM_F` reader - default fine trim value
pub type TrimFR = crate::FieldReader;
///Field `TRIM_C` reader - default coarse trim value
pub type TrimCR = crate::FieldReader;
///Field `EN_TRIM` reader - default value takes effect 0: default value is invalid 1: default value is valid
pub type EnTrimR = crate::BitReader;
///Field `SEL24M` reader - track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M
pub type Sel24mR = crate::BitReader;
///Field `SEL32K` reader - track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K
pub type Sel32kR = crate::BitReader;
impl R {
    ///Bits 0:4 - default fine trim value
    #[inline(always)]
    pub fn trim_f(&self) -> TrimFR {
        TrimFR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:10 - default coarse trim value
    #[inline(always)]
    pub fn trim_c(&self) -> TrimCR {
        TrimCR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 15 - default value takes effect 0: default value is invalid 1: default value is valid
    #[inline(always)]
    pub fn en_trim(&self) -> EnTrimR {
        EnTrimR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M
    #[inline(always)]
    pub fn sel24m(&self) -> Sel24mR {
        Sel24mR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K
    #[inline(always)]
    pub fn sel32k(&self) -> Sel32kR {
        Sel32kR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {}
/**RC 24M track status

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
///`reset()` method sets STATUS to value 0
impl crate::Resettable for StatusSpec {}
