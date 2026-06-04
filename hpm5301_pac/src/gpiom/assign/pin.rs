///Register `PIN[%s]` reader
pub type R = crate::R<PinSpec>;
///Register `PIN[%s]` writer
pub type W = crate::W<PinSpec>;
///Field `SELECT` reader - select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio
pub type SelectR = crate::FieldReader;
///Field `SELECT` writer - select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HIDE` reader - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio
pub type HideR = crate::FieldReader;
///Field `HIDE` writer - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio
pub type HideW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LOCK` reader - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable
pub type LockR = crate::BitReader;
///Field `LOCK` writer - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 3) as u8)
    }
    ///Bits 8:11 - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio
    #[inline(always)]
    pub fn hide(&self) -> HideR {
        HideR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 31 - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio
    #[inline(always)]
    pub fn select(&mut self) -> SelectW<'_, PinSpec> {
        SelectW::new(self, 0)
    }
    ///Bits 8:11 - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio
    #[inline(always)]
    pub fn hide(&mut self) -> HideW<'_, PinSpec> {
        HideW::new(self, 8)
    }
    ///Bit 31 - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, PinSpec> {
        LockW::new(self, 31)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`pin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PinSpec;
impl crate::RegisterSpec for PinSpec {
    type Ux = u32;
}
///`read()` method returns [`pin::R`](R) reader structure
impl crate::Readable for PinSpec {}
///`write(|w| ..)` method takes [`pin::W`](W) writer structure
impl crate::Writable for PinSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIN[%s] to value 0
impl crate::Resettable for PinSpec {}
