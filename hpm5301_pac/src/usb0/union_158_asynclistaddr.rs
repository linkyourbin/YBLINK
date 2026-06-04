///Register `ASYNCLISTADDR` reader
pub type R = crate::R<Union158AsynclistaddrSpec>;
///Register `ASYNCLISTADDR` writer
pub type W = crate::W<Union158AsynclistaddrSpec>;
///Field `ASYBASE` reader - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \[31:5\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller.
pub type AsybaseR = crate::FieldReader<u32>;
///Field `ASYBASE` writer - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \[31:5\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller.
pub type AsybaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 5:31 - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \[31:5\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller.
    #[inline(always)]
    pub fn asybase(&self) -> AsybaseR {
        AsybaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    ///Bits 5:31 - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \[31:5\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller.
    #[inline(always)]
    pub fn asybase(&mut self) -> AsybaseW<'_, Union158AsynclistaddrSpec> {
        AsybaseW::new(self, 5)
    }
}
/**Next Asynch. Address Register

You can [`read`](crate::Reg::read) this register and get [`union_158_asynclistaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_158_asynclistaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union158AsynclistaddrSpec;
impl crate::RegisterSpec for Union158AsynclistaddrSpec {
    type Ux = u32;
}
///`read()` method returns [`union_158_asynclistaddr::R`](R) reader structure
impl crate::Readable for Union158AsynclistaddrSpec {}
///`write(|w| ..)` method takes [`union_158_asynclistaddr::W`](W) writer structure
impl crate::Writable for Union158AsynclistaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASYNCLISTADDR to value 0
impl crate::Resettable for Union158AsynclistaddrSpec {}
