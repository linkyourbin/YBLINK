///Register `SEC_KEY_CTL` reader
pub type R = crate::R<SecKeyCtlSpec>;
///Register `SEC_KEY_CTL` writer
pub type W = crate::W<SecKeyCtlSpec>;
///Field `KEY_SEL` reader - secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected
pub type KeySelR = crate::FieldReader;
///Field `KEY_SEL` writer - secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected
pub type KeySelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FMK_SEL` reader - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key
pub type FmkSelR = crate::BitReader;
///Field `FMK_SEL` writer - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key
pub type FmkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZMK_SEL` reader - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key
pub type ZmkSelR = crate::BitReader;
///Field `ZMK_SEL` writer - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key
pub type ZmkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMK_SEL` reader - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key
pub type SmkSelR = crate::BitReader;
///Field `SMK_SEL` writer - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key
pub type SmkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SK_VAL` reader - session key valid 0: session key is all 0's and not usable 1: session key is valid
pub type SkValR = crate::BitReader;
///Field `LOCK_SEC_CTL` reader - block secure state key setting being changed
pub type LockSecCtlR = crate::BitReader;
///Field `LOCK_SEC_CTL` writer - block secure state key setting being changed
pub type LockSecCtlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected
    #[inline(always)]
    pub fn key_sel(&self) -> KeySelR {
        KeySelR::new((self.bits & 7) as u8)
    }
    ///Bit 4 - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key
    #[inline(always)]
    pub fn fmk_sel(&self) -> FmkSelR {
        FmkSelR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key
    #[inline(always)]
    pub fn zmk_sel(&self) -> ZmkSelR {
        ZmkSelR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key
    #[inline(always)]
    pub fn smk_sel(&self) -> SmkSelR {
        SmkSelR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - session key valid 0: session key is all 0's and not usable 1: session key is valid
    #[inline(always)]
    pub fn sk_val(&self) -> SkValR {
        SkValR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - block secure state key setting being changed
    #[inline(always)]
    pub fn lock_sec_ctl(&self) -> LockSecCtlR {
        LockSecCtlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected
    #[inline(always)]
    pub fn key_sel(&mut self) -> KeySelW<'_, SecKeyCtlSpec> {
        KeySelW::new(self, 0)
    }
    ///Bit 4 - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key
    #[inline(always)]
    pub fn fmk_sel(&mut self) -> FmkSelW<'_, SecKeyCtlSpec> {
        FmkSelW::new(self, 4)
    }
    ///Bit 8 - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key
    #[inline(always)]
    pub fn zmk_sel(&mut self) -> ZmkSelW<'_, SecKeyCtlSpec> {
        ZmkSelW::new(self, 8)
    }
    ///Bit 12 - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key
    #[inline(always)]
    pub fn smk_sel(&mut self) -> SmkSelW<'_, SecKeyCtlSpec> {
        SmkSelW::new(self, 12)
    }
    ///Bit 31 - block secure state key setting being changed
    #[inline(always)]
    pub fn lock_sec_ctl(&mut self) -> LockSecCtlW<'_, SecKeyCtlSpec> {
        LockSecCtlW::new(self, 31)
    }
}
/**secure key generation

You can [`read`](crate::Reg::read) this register and get [`sec_key_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_key_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SecKeyCtlSpec;
impl crate::RegisterSpec for SecKeyCtlSpec {
    type Ux = u32;
}
///`read()` method returns [`sec_key_ctl::R`](R) reader structure
impl crate::Readable for SecKeyCtlSpec {}
///`write(|w| ..)` method takes [`sec_key_ctl::W`](W) writer structure
impl crate::Writable for SecKeyCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEC_KEY_CTL to value 0
impl crate::Resettable for SecKeyCtlSpec {}
