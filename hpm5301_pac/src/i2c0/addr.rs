///Register `Addr` reader
pub type R = crate::R<AddrSpec>;
///Register `Addr` writer
pub type W = crate::W<AddrSpec>;
///Field `ADDR` reader - The slave address. For 7-bit addressing mode, the most significant 3 bits are ignored and only the least-significant 7 bits of Addr are valid
pub type AddrR = crate::FieldReader<u16>;
///Field `ADDR` writer - The slave address. For 7-bit addressing mode, the most significant 3 bits are ignored and only the least-significant 7 bits of Addr are valid
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - The slave address. For 7-bit addressing mode, the most significant 3 bits are ignored and only the least-significant 7 bits of Addr are valid
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - The slave address. For 7-bit addressing mode, the most significant 3 bits are ignored and only the least-significant 7 bits of Addr are valid
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, AddrSpec> {
        AddrW::new(self, 0)
    }
}
/**Address Register

You can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
///`read()` method returns [`addr::R`](R) reader structure
impl crate::Readable for AddrSpec {}
///`write(|w| ..)` method takes [`addr::W`](W) writer structure
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Addr to value 0
impl crate::Resettable for AddrSpec {}
