///Register `INTHALFSTS` reader
pub type R = crate::R<InthalfstsSpec>;
///Register `INTHALFSTS` writer
pub type W = crate::W<InthalfstsSpec>;
///Field `STS` reader - half transfer done irq status
pub type StsR = crate::FieldReader<u32>;
///Field `STS` writer - half transfer done irq status
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - half transfer done irq status
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - half transfer done irq status
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<'_, InthalfstsSpec> {
        StsW::new(self, 0)
    }
}
/**Harlf Complete Interrupt Status

You can [`read`](crate::Reg::read) this register and get [`inthalfsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inthalfsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct InthalfstsSpec;
impl crate::RegisterSpec for InthalfstsSpec {
    type Ux = u32;
}
///`read()` method returns [`inthalfsts::R`](R) reader structure
impl crate::Readable for InthalfstsSpec {}
///`write(|w| ..)` method takes [`inthalfsts::W`](W) writer structure
impl crate::Writable for InthalfstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTHALFSTS to value 0
impl crate::Resettable for InthalfstsSpec {}
