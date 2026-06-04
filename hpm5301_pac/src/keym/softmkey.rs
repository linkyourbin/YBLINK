///Register `SOFTMKEY[%s]` reader
pub type R = crate::R<SoftmkeySpec>;
///Register `SOFTMKEY[%s]` writer
pub type W = crate::W<SoftmkeySpec>;
///Field `KEY` reader - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0
pub type KeyR = crate::FieldReader<u32>;
///Field `KEY` writer - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SoftmkeySpec> {
        KeyW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`softmkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softmkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SoftmkeySpec;
impl crate::RegisterSpec for SoftmkeySpec {
    type Ux = u32;
}
///`read()` method returns [`softmkey::R`](R) reader structure
impl crate::Readable for SoftmkeySpec {}
///`write(|w| ..)` method takes [`softmkey::W`](W) writer structure
impl crate::Writable for SoftmkeySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOFTMKEY[%s] to value 0
impl crate::Resettable for SoftmkeySpec {}
