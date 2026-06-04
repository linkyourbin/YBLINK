///Register `CONTROL` reader
pub type R = crate::R<ControlSpec>;
///Register `CONTROL` writer
pub type W = crate::W<ControlSpec>;
///Field `ENABLE` reader - enable glitch detector 0: detector disabled 1: detector enabled
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - enable glitch detector 0: detector disabled 1: detector enabled
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACTIVE` reader - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destroy DFF value 1: active mode, check glitch by DFF chain
pub type ActiveR = crate::BitReader;
///Field `ACTIVE` writer - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destroy DFF value 1: active mode, check glitch by DFF chain
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable glitch detector 0: detector disabled 1: detector enabled
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destroy DFF value 1: active mode, check glitch by DFF chain
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - enable glitch detector 0: detector disabled 1: detector enabled
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ControlSpec> {
        EnableW::new(self, 0)
    }
    ///Bit 4 - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destroy DFF value 1: active mode, check glitch by DFF chain
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, ControlSpec> {
        ActiveW::new(self, 4)
    }
}
/**Glitch and clock monitor control

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
///`reset()` method sets CONTROL to value 0
impl crate::Resettable for ControlSpec {}
