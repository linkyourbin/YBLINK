///Register `MFI` reader
pub type R = crate::R<MfiSpec>;
///Register `MFI` writer
pub type W = crate::W<MfiSpec>;
///Field `MFI` reader - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid
pub type MfiR = crate::FieldReader;
///Field `MFI` writer - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid
pub type MfiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `ENABLE` reader - PLL enable status 0: PLL is off 1: PLL is on
pub type EnableR = crate::BitReader;
///Field `RESPONSE` reader - PLL status 0: PLL is not stable 1: PLL is stable for use
pub type ResponseR = crate::BitReader;
///Field `BUSY` reader - Busy flag 0: PLL is stable or shutdown 1: PLL is changing status
pub type BusyR = crate::BitReader;
impl R {
    ///Bits 0:6 - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid
    #[inline(always)]
    pub fn mfi(&self) -> MfiR {
        MfiR::new((self.bits & 0x7f) as u8)
    }
    ///Bit 28 - PLL enable status 0: PLL is off 1: PLL is on
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLL status 0: PLL is not stable 1: PLL is stable for use
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Busy flag 0: PLL is stable or shutdown 1: PLL is changing status
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid
    #[inline(always)]
    pub fn mfi(&mut self) -> MfiW<'_, MfiSpec> {
        MfiW::new(self, 0)
    }
}
/**PLL0 multiple register

You can [`read`](crate::Reg::read) this register and get [`mfi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MfiSpec;
impl crate::RegisterSpec for MfiSpec {
    type Ux = u32;
}
///`read()` method returns [`mfi::R`](R) reader structure
impl crate::Readable for MfiSpec {}
///`write(|w| ..)` method takes [`mfi::W`](W) writer structure
impl crate::Writable for MfiSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MFI to value 0x10
impl crate::Resettable for MfiSpec {
    const RESET_VALUE: u32 = 0x10;
}
