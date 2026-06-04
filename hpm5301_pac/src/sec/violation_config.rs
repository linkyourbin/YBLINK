///Register `VIOLATION_CONFIG` reader
pub type R = crate::R<ViolationConfigSpec>;
///Register `VIOLATION_CONFIG` writer
pub type W = crate::W<ViolationConfigSpec>;
///Field `SEC_VIO_CFG` reader - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
pub type SecVioCfgR = crate::FieldReader<u16>;
///Field `SEC_VIO_CFG` writer - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
pub type SecVioCfgW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `LOCK_SEC` reader - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
pub type LockSecR = crate::BitReader;
///Field `LOCK_SEC` writer - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
pub type LockSecW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSC_VIO_CFG` reader - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
pub type NscVioCfgR = crate::FieldReader<u16>;
///Field `NSC_VIO_CFG` writer - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
pub type NscVioCfgW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `LOCK_NSC` reader - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
pub type LockNscR = crate::BitReader;
///Field `LOCK_NSC` writer - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
pub type LockNscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:14 - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
    #[inline(always)]
    pub fn sec_vio_cfg(&self) -> SecVioCfgR {
        SecVioCfgR::new((self.bits & 0x7fff) as u16)
    }
    ///Bit 15 - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
    #[inline(always)]
    pub fn lock_sec(&self) -> LockSecR {
        LockSecR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:30 - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
    #[inline(always)]
    pub fn nsc_vio_cfg(&self) -> NscVioCfgR {
        NscVioCfgR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31 - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
    #[inline(always)]
    pub fn lock_nsc(&self) -> LockNscR {
        LockNscR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:14 - configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
    #[inline(always)]
    pub fn sec_vio_cfg(&mut self) -> SecVioCfgW<'_, ViolationConfigSpec> {
        SecVioCfgW::new(self, 0)
    }
    ///Bit 15 - Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
    #[inline(always)]
    pub fn lock_sec(&mut self) -> LockSecW<'_, ViolationConfigSpec> {
        LockSecW::new(self, 15)
    }
    ///Bits 16:30 - configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation
    #[inline(always)]
    pub fn nsc_vio_cfg(&mut self) -> NscVioCfgW<'_, ViolationConfigSpec> {
        NscVioCfgW::new(self, 16)
    }
    ///Bit 31 - Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored
    #[inline(always)]
    pub fn lock_nsc(&mut self) -> LockNscW<'_, ViolationConfigSpec> {
        LockNscW::new(self, 31)
    }
}
/**Security violation config

You can [`read`](crate::Reg::read) this register and get [`violation_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`violation_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ViolationConfigSpec;
impl crate::RegisterSpec for ViolationConfigSpec {
    type Ux = u32;
}
///`read()` method returns [`violation_config::R`](R) reader structure
impl crate::Readable for ViolationConfigSpec {}
///`write(|w| ..)` method takes [`violation_config::W`](W) writer structure
impl crate::Writable for ViolationConfigSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VIOLATION_CONFIG to value 0
impl crate::Resettable for ViolationConfigSpec {}
