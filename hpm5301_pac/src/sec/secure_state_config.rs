///Register `SECURE_STATE_CONFIG` reader
pub type R = crate::R<SecureStateConfigSpec>;
///Register `SECURE_STATE_CONFIG` writer
pub type W = crate::W<SecureStateConfigSpec>;
///Field `ALLOW_RESTART` reader - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state
pub type AllowRestartR = crate::BitReader;
///Field `ALLOW_RESTART` writer - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state
pub type AllowRestartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored
pub type LockR = crate::BitReader;
///Field `LOCK` writer - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state
    #[inline(always)]
    pub fn allow_restart(&self) -> AllowRestartR {
        AllowRestartR::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state
    #[inline(always)]
    pub fn allow_restart(&mut self) -> AllowRestartW<'_, SecureStateConfigSpec> {
        AllowRestartW::new(self, 0)
    }
    ///Bit 3 - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, SecureStateConfigSpec> {
        LockW::new(self, 3)
    }
}
/**secure state configuration

You can [`read`](crate::Reg::read) this register and get [`secure_state_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_state_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SecureStateConfigSpec;
impl crate::RegisterSpec for SecureStateConfigSpec {
    type Ux = u32;
}
///`read()` method returns [`secure_state_config::R`](R) reader structure
impl crate::Readable for SecureStateConfigSpec {}
///`write(|w| ..)` method takes [`secure_state_config::W`](W) writer structure
impl crate::Writable for SecureStateConfigSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECURE_STATE_CONFIG to value 0
impl crate::Resettable for SecureStateConfigSpec {}
