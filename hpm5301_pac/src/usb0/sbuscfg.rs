///Register `SBUSCFG` reader
pub type R = crate::R<SbuscfgSpec>;
///Register `SBUSCFG` writer
pub type W = crate::W<SbuscfgSpec>;
///Field `AHBBRST` reader - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length
pub type AhbbrstR = crate::FieldReader;
///Field `AHBBRST` writer - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length
pub type AhbbrstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length
    #[inline(always)]
    pub fn ahbbrst(&self) -> AhbbrstR {
        AhbbrstR::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length
    #[inline(always)]
    pub fn ahbbrst(&mut self) -> AhbbrstW<'_, SbuscfgSpec> {
        AhbbrstW::new(self, 0)
    }
}
/**System Bus Config Register

You can [`read`](crate::Reg::read) this register and get [`sbuscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbuscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SbuscfgSpec;
impl crate::RegisterSpec for SbuscfgSpec {
    type Ux = u32;
}
///`read()` method returns [`sbuscfg::R`](R) reader structure
impl crate::Readable for SbuscfgSpec {}
///`write(|w| ..)` method takes [`sbuscfg::W`](W) writer structure
impl crate::Writable for SbuscfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SBUSCFG to value 0
impl crate::Resettable for SbuscfgSpec {}
