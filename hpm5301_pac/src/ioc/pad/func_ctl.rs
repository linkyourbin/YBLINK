///Register `FUNC_CTL` reader
pub type R = crate::R<FuncCtlSpec>;
///Register `FUNC_CTL` writer
pub type W = crate::W<FuncCtlSpec>;
///Field `ALT_SELECT` reader - alt select 0: ALT0 1: ALT1 ... 31:ALT31
pub type AltSelectR = crate::FieldReader;
///Field `ALT_SELECT` writer - alt select 0: ALT0 1: ALT1 ... 31:ALT31
pub type AltSelectW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ANALOG` reader - select analog pin in pad 0: disable 1: enable
pub type AnalogR = crate::BitReader;
///Field `ANALOG` writer - select analog pin in pad 0: disable 1: enable
pub type AnalogW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOP_BACK` reader - force input on 0: disable 1: enable
pub type LoopBackR = crate::BitReader;
///Field `LOOP_BACK` writer - force input on 0: disable 1: enable
pub type LoopBackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - alt select 0: ALT0 1: ALT1 ... 31:ALT31
    #[inline(always)]
    pub fn alt_select(&self) -> AltSelectR {
        AltSelectR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - select analog pin in pad 0: disable 1: enable
    #[inline(always)]
    pub fn analog(&self) -> AnalogR {
        AnalogR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - force input on 0: disable 1: enable
    #[inline(always)]
    pub fn loop_back(&self) -> LoopBackR {
        LoopBackR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - alt select 0: ALT0 1: ALT1 ... 31:ALT31
    #[inline(always)]
    pub fn alt_select(&mut self) -> AltSelectW<'_, FuncCtlSpec> {
        AltSelectW::new(self, 0)
    }
    ///Bit 8 - select analog pin in pad 0: disable 1: enable
    #[inline(always)]
    pub fn analog(&mut self) -> AnalogW<'_, FuncCtlSpec> {
        AnalogW::new(self, 8)
    }
    ///Bit 16 - force input on 0: disable 1: enable
    #[inline(always)]
    pub fn loop_back(&mut self) -> LoopBackW<'_, FuncCtlSpec> {
        LoopBackW::new(self, 16)
    }
}
/**ALT SELECT

You can [`read`](crate::Reg::read) this register and get [`func_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FuncCtlSpec;
impl crate::RegisterSpec for FuncCtlSpec {
    type Ux = u32;
}
///`read()` method returns [`func_ctl::R`](R) reader structure
impl crate::Readable for FuncCtlSpec {}
///`write(|w| ..)` method takes [`func_ctl::W`](W) writer structure
impl crate::Writable for FuncCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FUNC_CTL to value 0
impl crate::Resettable for FuncCtlSpec {}
