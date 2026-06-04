///Register `SECURE_STATE` reader
pub type R = crate::R<SecureStateSpec>;
///Register `SECURE_STATE` writer
pub type W = crate::W<SecureStateSpec>;
///Field `PMIC_INS` reader - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state
pub type PmicInsR = crate::BitReader;
///Field `PMIC_INS` writer - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state
pub type PmicInsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMIC_SEC` reader - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state
pub type PmicSecR = crate::BitReader;
///Field `PMIC_SEC` writer - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state
pub type PmicSecW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMIC_NSC` reader - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state
pub type PmicNscR = crate::BitReader;
///Field `PMIC_NSC` writer - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state
pub type PmicNscW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMIC_FAIL` reader - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state
pub type PmicFailR = crate::BitReader;
///Field `PMIC_FAIL` writer - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state
pub type PmicFailW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALLOW_SEC` reader - Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state
pub type AllowSecR = crate::BitReader;
///Field `ALLOW_NSC` reader - Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state
pub type AllowNscR = crate::BitReader;
impl R {
    ///Bit 4 - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state
    #[inline(always)]
    pub fn pmic_ins(&self) -> PmicInsR {
        PmicInsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state
    #[inline(always)]
    pub fn pmic_sec(&self) -> PmicSecR {
        PmicSecR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state
    #[inline(always)]
    pub fn pmic_nsc(&self) -> PmicNscR {
        PmicNscR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state
    #[inline(always)]
    pub fn pmic_fail(&self) -> PmicFailR {
        PmicFailR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state
    #[inline(always)]
    pub fn allow_sec(&self) -> AllowSecR {
        AllowSecR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state
    #[inline(always)]
    pub fn allow_nsc(&self) -> AllowNscR {
        AllowNscR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state
    #[inline(always)]
    pub fn pmic_ins(&mut self) -> PmicInsW<'_, SecureStateSpec> {
        PmicInsW::new(self, 4)
    }
    ///Bit 5 - PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state
    #[inline(always)]
    pub fn pmic_sec(&mut self) -> PmicSecW<'_, SecureStateSpec> {
        PmicSecW::new(self, 5)
    }
    ///Bit 6 - PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state
    #[inline(always)]
    pub fn pmic_nsc(&mut self) -> PmicNscW<'_, SecureStateSpec> {
        PmicNscW::new(self, 6)
    }
    ///Bit 7 - PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state
    #[inline(always)]
    pub fn pmic_fail(&mut self) -> PmicFailW<'_, SecureStateSpec> {
        PmicFailW::new(self, 7)
    }
}
/**Secure state

You can [`read`](crate::Reg::read) this register and get [`secure_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SecureStateSpec;
impl crate::RegisterSpec for SecureStateSpec {
    type Ux = u32;
}
///`read()` method returns [`secure_state::R`](R) reader structure
impl crate::Readable for SecureStateSpec {}
///`write(|w| ..)` method takes [`secure_state::W`](W) writer structure
impl crate::Writable for SecureStateSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECURE_STATE to value 0
impl crate::Resettable for SecureStateSpec {}
