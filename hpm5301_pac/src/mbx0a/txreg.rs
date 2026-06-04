///Register `TXREG` reader
pub type R = crate::R<TxregSpec>;
///Register `TXREG` writer
pub type W = crate::W<TxregSpec>;
///Field `TXREG` writer - Transmit word message to other core.
pub type TxregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Transmit word message to other core.
    #[inline(always)]
    pub fn txreg(&mut self) -> TxregW<'_, TxregSpec> {
        TxregW::new(self, 0)
    }
}
/**Transmit word message to other core.

You can [`read`](crate::Reg::read) this register and get [`txreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TxregSpec;
impl crate::RegisterSpec for TxregSpec {
    type Ux = u32;
}
///`read()` method returns [`txreg::R`](R) reader structure
impl crate::Readable for TxregSpec {}
///`write(|w| ..)` method takes [`txreg::W`](W) writer structure
impl crate::Writable for TxregSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXREG to value 0
impl crate::Resettable for TxregSpec {}
