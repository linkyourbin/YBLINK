///Register `VALIDITY` reader
pub type R = crate::R<ValiditySpec>;
///Register `VALIDITY` writer
pub type W = crate::W<ValiditySpec>;
///Field `VALIDITY` reader - time for temperature values to expire in 24M clock cycles
pub type ValidityR = crate::FieldReader<u32>;
///Field `VALIDITY` writer - time for temperature values to expire in 24M clock cycles
pub type ValidityW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - time for temperature values to expire in 24M clock cycles
    #[inline(always)]
    pub fn validity(&self) -> ValidityR {
        ValidityR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - time for temperature values to expire in 24M clock cycles
    #[inline(always)]
    pub fn validity(&mut self) -> ValidityW<'_, ValiditySpec> {
        ValidityW::new(self, 0)
    }
}
/**Sample validity

You can [`read`](crate::Reg::read) this register and get [`validity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`validity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ValiditySpec;
impl crate::RegisterSpec for ValiditySpec {
    type Ux = u32;
}
///`read()` method returns [`validity::R`](R) reader structure
impl crate::Readable for ValiditySpec {}
///`write(|w| ..)` method takes [`validity::W`](W) writer structure
impl crate::Writable for ValiditySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VALIDITY to value 0x016e_3600
impl crate::Resettable for ValiditySpec {
    const RESET_VALUE: u32 = 0x016e_3600;
}
