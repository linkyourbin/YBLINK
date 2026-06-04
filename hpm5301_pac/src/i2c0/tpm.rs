///Register `TPM` reader
pub type R = crate::R<TpmSpec>;
///Register `TPM` writer
pub type W = crate::W<TpmSpec>;
///Field `TPM` reader - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1).
pub type TpmR = crate::FieldReader;
///Field `TPM` writer - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1).
pub type TpmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1).
    #[inline(always)]
    pub fn tpm(&self) -> TpmR {
        TpmR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1).
    #[inline(always)]
    pub fn tpm(&mut self) -> TpmW<'_, TpmSpec> {
        TpmW::new(self, 0)
    }
}
/**I2C Timing Paramater Multiplier

You can [`read`](crate::Reg::read) this register and get [`tpm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TpmSpec;
impl crate::RegisterSpec for TpmSpec {
    type Ux = u32;
}
///`read()` method returns [`tpm::R`](R) reader structure
impl crate::Readable for TpmSpec {}
///`write(|w| ..)` method takes [`tpm::W`](W) writer structure
impl crate::Writable for TpmSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TPM to value 0
impl crate::Resettable for TpmSpec {}
