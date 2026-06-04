///Register `THR` reader
pub type R = crate::R<Union20ThrSpec>;
///Register `THR` writer
pub type W = crate::W<Union20ThrSpec>;
///Field `THR` writer - Transmit data write port
pub type ThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Transmit data write port
    #[inline(always)]
    pub fn thr(&mut self) -> ThrW<'_, Union20ThrSpec> {
        ThrW::new(self, 0)
    }
}
/**Transmitter Holding Register (when DLAB = 0)

You can [`read`](crate::Reg::read) this register and get [`union_20_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_20_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union20ThrSpec;
impl crate::RegisterSpec for Union20ThrSpec {
    type Ux = u32;
}
///`read()` method returns [`union_20_thr::R`](R) reader structure
impl crate::Readable for Union20ThrSpec {}
///`write(|w| ..)` method takes [`union_20_thr::W`](W) writer structure
impl crate::Writable for Union20ThrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets THR to value 0
impl crate::Resettable for Union20ThrSpec {}
