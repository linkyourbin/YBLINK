///Register `TXWRD[%s]` reader
pub type R = crate::R<TxwrdSpec>;
///Register `TXWRD[%s]` writer
pub type W = crate::W<TxwrdSpec>;
///Field `TXFIFO` writer - TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO.
pub type TxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO.
    #[inline(always)]
    pub fn txfifo(&mut self) -> TxfifoW<'_, TxwrdSpec> {
        TxfifoW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`txwrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txwrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TxwrdSpec;
impl crate::RegisterSpec for TxwrdSpec {
    type Ux = u32;
}
///`read()` method returns [`txwrd::R`](R) reader structure
impl crate::Readable for TxwrdSpec {}
///`write(|w| ..)` method takes [`txwrd::W`](W) writer structure
impl crate::Writable for TxwrdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXWRD[%s] to value 0
impl crate::Resettable for TxwrdSpec {}
