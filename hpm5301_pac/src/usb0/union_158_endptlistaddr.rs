///Register `ENDPTLISTADDR` reader
pub type R = crate::R<Union158EndptlistaddrSpec>;
///Register `ENDPTLISTADDR` writer
pub type W = crate::W<Union158EndptlistaddrSpec>;
///Field `EPBASE` reader - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \[31:11\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction).
pub type EpbaseR = crate::FieldReader<u32>;
///Field `EPBASE` writer - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \[31:11\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction).
pub type EpbaseW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    ///Bits 11:31 - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \[31:11\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction).
    #[inline(always)]
    pub fn epbase(&self) -> EpbaseR {
        EpbaseR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    ///Bits 11:31 - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \[31:11\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction).
    #[inline(always)]
    pub fn epbase(&mut self) -> EpbaseW<'_, Union158EndptlistaddrSpec> {
        EpbaseW::new(self, 11)
    }
}
/**Endpoint List Address Register

You can [`read`](crate::Reg::read) this register and get [`union_158_endptlistaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_158_endptlistaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union158EndptlistaddrSpec;
impl crate::RegisterSpec for Union158EndptlistaddrSpec {
    type Ux = u32;
}
///`read()` method returns [`union_158_endptlistaddr::R`](R) reader structure
impl crate::Readable for Union158EndptlistaddrSpec {}
///`write(|w| ..)` method takes [`union_158_endptlistaddr::W`](W) writer structure
impl crate::Writable for Union158EndptlistaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTLISTADDR to value 0
impl crate::Resettable for Union158EndptlistaddrSpec {}
