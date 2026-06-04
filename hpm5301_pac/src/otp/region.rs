///Register `REGION[%s]` reader
pub type R = crate::R<RegionSpec>;
///Register `REGION[%s]` writer
pub type W = crate::W<RegionSpec>;
///Field `START` reader - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable
pub type StartR = crate::FieldReader;
///Field `START` writer - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `STOP` reader - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable
pub type StopR = crate::FieldReader;
///Field `STOP` writer - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, RegionSpec> {
        StartW::new(self, 0)
    }
    ///Bits 8:14 - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, RegionSpec> {
        StopW::new(self, 8)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`region::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RegionSpec;
impl crate::RegisterSpec for RegionSpec {
    type Ux = u32;
}
///`read()` method returns [`region::R`](R) reader structure
impl crate::Readable for RegionSpec {}
///`write(|w| ..)` method takes [`region::W`](W) writer structure
impl crate::Writable for RegionSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REGION[%s] to value 0x0800
impl crate::Resettable for RegionSpec {
    const RESET_VALUE: u32 = 0x0800;
}
