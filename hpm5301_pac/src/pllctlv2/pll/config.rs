///Register `CONFIG` reader
pub type R = crate::R<ConfigSpec>;
///Register `CONFIG` writer
pub type W = crate::W<ConfigSpec>;
///Field `REFSEL` reader - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M
pub type RefselR = crate::BitReader;
///Field `REFSEL` writer - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M
pub type RefselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPREAD` reader - Enable spread spectrum function. This field supports changing during PLL running.
pub type SpreadR = crate::BitReader;
///Field `SPREAD` writer - Enable spread spectrum function. This field supports changing during PLL running.
pub type SpreadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Enable spread spectrum function. This field supports changing during PLL running.
    #[inline(always)]
    pub fn spread(&self) -> SpreadR {
        SpreadR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, ConfigSpec> {
        RefselW::new(self, 0)
    }
    ///Bit 8 - Enable spread spectrum function. This field supports changing during PLL running.
    #[inline(always)]
    pub fn spread(&mut self) -> SpreadW<'_, ConfigSpec> {
        SpreadW::new(self, 8)
    }
}
/**PLL0 confguration register

You can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
///`read()` method returns [`config::R`](R) reader structure
impl crate::Readable for ConfigSpec {}
///`write(|w| ..)` method takes [`config::W`](W) writer structure
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFIG to value 0
impl crate::Resettable for ConfigSpec {}
