///Register `sr` reader
pub type R = crate::R<SrSpec>;
///Register `sr` writer
pub type W = crate::W<SrSpec>;
///Field `REDGF` reader - Output rising edge flag. Write 1 to clear this flag.
pub type RedgfR = crate::BitReader;
///Field `REDGF` writer - Output rising edge flag. Write 1 to clear this flag.
pub type RedgfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEDGF` reader - Output falling edge flag. Write 1 to clear this flag.
pub type FedgfR = crate::BitReader;
///Field `FEDGF` writer - Output falling edge flag. Write 1 to clear this flag.
pub type FedgfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Output rising edge flag. Write 1 to clear this flag.
    #[inline(always)]
    pub fn redgf(&self) -> RedgfR {
        RedgfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output falling edge flag. Write 1 to clear this flag.
    #[inline(always)]
    pub fn fedgf(&self) -> FedgfR {
        FedgfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Output rising edge flag. Write 1 to clear this flag.
    #[inline(always)]
    pub fn redgf(&mut self) -> RedgfW<'_, SrSpec> {
        RedgfW::new(self, 0)
    }
    ///Bit 1 - Output falling edge flag. Write 1 to clear this flag.
    #[inline(always)]
    pub fn fedgf(&mut self) -> FedgfW<'_, SrSpec> {
        FedgfW::new(self, 1)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SrSpec {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets sr to value 0
impl crate::Resettable for SrSpec {}
