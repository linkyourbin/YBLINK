///Register `SOFTPKEY[%s]` reader
pub type R = crate::R<SoftpkeySpec>;
///Register `SOFTPKEY[%s]` writer
pub type W = crate::W<SoftpkeySpec>;
///Field `KEY` reader - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0
pub type KeyR = crate::FieldReader<u32>;
///Field `KEY` writer - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SoftpkeySpec> {
        KeyW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`softpkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softpkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SoftpkeySpec;
impl crate::RegisterSpec for SoftpkeySpec {
    type Ux = u32;
}
///`read()` method returns [`softpkey::R`](R) reader structure
impl crate::Readable for SoftpkeySpec {}
///`write(|w| ..)` method takes [`softpkey::W`](W) writer structure
impl crate::Writable for SoftpkeySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOFTPKEY[%s] to value 0
impl crate::Resettable for SoftpkeySpec {}
