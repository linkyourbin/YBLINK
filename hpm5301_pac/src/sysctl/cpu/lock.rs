///Register `LOCK` reader
pub type R = crate::R<LockSpec>;
///Register `LOCK` writer
pub type W = crate::W<LockSpec>;
///Field `LOCK` reader - Lock bit for CPU_LOCK
pub type LockR = crate::BitReader;
///Field `LOCK` writer - Lock bit for CPU_LOCK
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPR` reader - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset
pub type GprR = crate::FieldReader<u16>;
///Field `GPR` writer - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset
pub type GprW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 1 - Lock bit for CPU_LOCK
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:15 - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset
    #[inline(always)]
    pub fn gpr(&self) -> GprR {
        GprR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    ///Bit 1 - Lock bit for CPU_LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, LockSpec> {
        LockW::new(self, 1)
    }
    ///Bits 2:15 - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset
    #[inline(always)]
    pub fn gpr(&mut self) -> GprW<'_, LockSpec> {
        GprW::new(self, 2)
    }
}
/**CPU0 Lock GPR

You can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
///`read()` method returns [`lock::R`](R) reader structure
impl crate::Readable for LockSpec {}
///`write(|w| ..)` method takes [`lock::W`](W) writer structure
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCK to value 0
impl crate::Resettable for LockSpec {}
