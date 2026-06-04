///Register `INFO` reader
pub type R = crate::R<InfoSpec>;
///Register `INFO` writer
pub type W = crate::W<InfoSpec>;
///Field `VERSION` reader - The version of the PLIC design
pub type VersionR = crate::FieldReader<u16>;
///Field `MAX_PRIORITY` reader - The maximum priority supported
pub type MaxPriorityR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - The version of the PLIC design
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - The maximum priority supported
    #[inline(always)]
    pub fn max_priority(&self) -> MaxPriorityR {
        MaxPriorityR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
/**Version and the maximum priority

You can [`read`](crate::Reg::read) this register and get [`info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
///`read()` method returns [`info::R`](R) reader structure
impl crate::Readable for InfoSpec {}
///`write(|w| ..)` method takes [`info::W`](W) writer structure
impl crate::Writable for InfoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INFO to value 0
impl crate::Resettable for InfoSpec {}
