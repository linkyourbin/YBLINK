///Register `DIV[%s]` reader
pub type R = crate::R<DivSpec>;
///Register `DIV[%s]` writer
pub type W = crate::W<DivSpec>;
///Field `DIV` reader - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6
pub type DivR = crate::FieldReader;
///Field `DIV` writer - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `ENABLE` reader - Divider enable status 0: Divider is off 1: Divider is on
pub type EnableR = crate::BitReader;
///Field `RESPONSE` reader - Divider response status 0: Divider is not stable 1: Divider is stable for use
pub type ResponseR = crate::BitReader;
///Field `BUSY` reader - Busy flag 0: divider is working 1: divider is changing status
pub type BusyR = crate::BitReader;
impl R {
    ///Bits 0:5 - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x3f) as u8)
    }
    ///Bit 28 - Divider enable status 0: Divider is off 1: Divider is on
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Divider response status 0: Divider is not stable 1: Divider is stable for use
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Busy flag 0: divider is working 1: divider is changing status
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, DivSpec> {
        DivW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DivSpec;
impl crate::RegisterSpec for DivSpec {
    type Ux = u32;
}
///`read()` method returns [`div::R`](R) reader structure
impl crate::Readable for DivSpec {}
///`write(|w| ..)` method takes [`div::W`](W) writer structure
impl crate::Writable for DivSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIV[%s] to value 0
impl crate::Resettable for DivSpec {}
