///Register `RC24M_TRACK` reader
pub type R = crate::R<Rc24mTrackSpec>;
///Register `RC24M_TRACK` writer
pub type W = crate::W<Rc24mTrackSpec>;
///Field `TRACK` reader - track mode 0: RC24M free running 1: track RC24M to external XTAL
pub type TrackR = crate::BitReader;
///Field `TRACK` writer - track mode 0: RC24M free running 1: track RC24M to external XTAL
pub type TrackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETURN` reader - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value
pub type ReturnR = crate::BitReader;
///Field `RETURN` writer - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value
pub type ReturnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL24M` reader - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference
pub type Sel24mR = crate::BitReader;
///Field `SEL24M` writer - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference
pub type Sel24mW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - track mode 0: RC24M free running 1: track RC24M to external XTAL
    #[inline(always)]
    pub fn track(&self) -> TrackR {
        TrackR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value
    #[inline(always)]
    pub fn return_(&self) -> ReturnR {
        ReturnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference
    #[inline(always)]
    pub fn sel24m(&self) -> Sel24mR {
        Sel24mR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - track mode 0: RC24M free running 1: track RC24M to external XTAL
    #[inline(always)]
    pub fn track(&mut self) -> TrackW<'_, Rc24mTrackSpec> {
        TrackW::new(self, 0)
    }
    ///Bit 4 - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value
    #[inline(always)]
    pub fn return_(&mut self) -> ReturnW<'_, Rc24mTrackSpec> {
        ReturnW::new(self, 4)
    }
    ///Bit 16 - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference
    #[inline(always)]
    pub fn sel24m(&mut self) -> Sel24mW<'_, Rc24mTrackSpec> {
        Sel24mW::new(self, 16)
    }
}
/**RC 24M track mode

You can [`read`](crate::Reg::read) this register and get [`rc24m_track::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc24m_track::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rc24mTrackSpec;
impl crate::RegisterSpec for Rc24mTrackSpec {
    type Ux = u32;
}
///`read()` method returns [`rc24m_track::R`](R) reader structure
impl crate::Readable for Rc24mTrackSpec {}
///`write(|w| ..)` method takes [`rc24m_track::W`](W) writer structure
impl crate::Writable for Rc24mTrackSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RC24M_TRACK to value 0
impl crate::Resettable for Rc24mTrackSpec {}
