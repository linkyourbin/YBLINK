///Register `config` reader
pub type R = crate::R<ConfigSpec>;
///Register `config` writer
pub type W = crate::W<ConfigSpec>;
///Field `POST_WAIT` reader - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
pub type PostWaitR = crate::FieldReader;
///Field `POST_WAIT` writer - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
pub type PostWaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSTCLK_NUM` reader - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M
pub type RstclkNumR = crate::FieldReader;
///Field `RSTCLK_NUM` writer - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M
pub type RstclkNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PRE_WAIT` reader - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
pub type PreWaitR = crate::FieldReader;
///Field `PRE_WAIT` writer - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
pub type PreWaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn post_wait(&self) -> PostWaitR {
        PostWaitR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn rstclk_num(&self) -> RstclkNumR {
        RstclkNumR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn pre_wait(&self) -> PreWaitR {
        PreWaitR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn post_wait(&mut self) -> PostWaitW<'_, ConfigSpec> {
        PostWaitW::new(self, 0)
    }
    ///Bits 8:15 - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn rstclk_num(&mut self) -> RstclkNumW<'_, ConfigSpec> {
        RstclkNumW::new(self, 8)
    }
    ///Bits 16:23 - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M
    #[inline(always)]
    pub fn pre_wait(&mut self) -> PreWaitW<'_, ConfigSpec> {
        PreWaitW::new(self, 16)
    }
}
/**Reset Setting

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
///`reset()` method sets config to value 0x0040_2003
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x0040_2003;
}
