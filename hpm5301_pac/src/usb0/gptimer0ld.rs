///Register `GPTIMER0LD` reader
pub type R = crate::R<Gptimer0ldSpec>;
///Register `GPTIMER0LD` writer
pub type W = crate::W<Gptimer0ldSpec>;
///Field `GPTLD` reader - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds.
pub type GptldR = crate::FieldReader<u32>;
///Field `GPTLD` writer - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds.
pub type GptldW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds.
    #[inline(always)]
    pub fn gptld(&self) -> GptldR {
        GptldR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds.
    #[inline(always)]
    pub fn gptld(&mut self) -> GptldW<'_, Gptimer0ldSpec> {
        GptldW::new(self, 0)
    }
}
/**General Purpose Timer #0 Load Register

You can [`read`](crate::Reg::read) this register and get [`gptimer0ld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptimer0ld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Gptimer0ldSpec;
impl crate::RegisterSpec for Gptimer0ldSpec {
    type Ux = u32;
}
///`read()` method returns [`gptimer0ld::R`](R) reader structure
impl crate::Readable for Gptimer0ldSpec {}
///`write(|w| ..)` method takes [`gptimer0ld::W`](W) writer structure
impl crate::Writable for Gptimer0ldSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPTIMER0LD to value 0
impl crate::Resettable for Gptimer0ldSpec {}
