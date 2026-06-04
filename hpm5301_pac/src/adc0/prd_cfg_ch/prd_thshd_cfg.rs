///Register `prd_thshd_cfg` reader
pub type R = crate::R<PrdThshdCfgSpec>;
///Register `prd_thshd_cfg` writer
pub type W = crate::W<PrdThshdCfgSpec>;
///Field `THSHDL` reader - threshold low
pub type ThshdlR = crate::FieldReader<u16>;
///Field `THSHDL` writer - threshold low
pub type ThshdlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `THSHDH` reader - threshold high, assert interrupt(if enabled) if result exceed high or low.
pub type ThshdhR = crate::FieldReader<u16>;
///Field `THSHDH` writer - threshold high, assert interrupt(if enabled) if result exceed high or low.
pub type ThshdhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - threshold low
    #[inline(always)]
    pub fn thshdl(&self) -> ThshdlR {
        ThshdlR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - threshold high, assert interrupt(if enabled) if result exceed high or low.
    #[inline(always)]
    pub fn thshdh(&self) -> ThshdhR {
        ThshdhR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - threshold low
    #[inline(always)]
    pub fn thshdl(&mut self) -> ThshdlW<'_, PrdThshdCfgSpec> {
        ThshdlW::new(self, 0)
    }
    ///Bits 16:31 - threshold high, assert interrupt(if enabled) if result exceed high or low.
    #[inline(always)]
    pub fn thshdh(&mut self) -> ThshdhW<'_, PrdThshdCfgSpec> {
        ThshdhW::new(self, 16)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`prd_thshd_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prd_thshd_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PrdThshdCfgSpec;
impl crate::RegisterSpec for PrdThshdCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`prd_thshd_cfg::R`](R) reader structure
impl crate::Readable for PrdThshdCfgSpec {}
///`write(|w| ..)` method takes [`prd_thshd_cfg::W`](W) writer structure
impl crate::Writable for PrdThshdCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets prd_thshd_cfg to value 0
impl crate::Resettable for PrdThshdCfgSpec {}
