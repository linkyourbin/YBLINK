///Register `RXWRD[%s]` reader
pub type R = crate::R<RxwrdSpec>;
///Register `RXWRD[%s]` writer
pub type W = crate::W<RxwrdSpec>;
///Field `RXFIFO` reader - RXFIFO for receiving message from other core, FIFO size, 4x32 can read one of the word address to pop data to the FIFO; can also use 4x32 burst read from 0x020 to read 4 words from the FIFO.
pub type RxfifoR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RXFIFO for receiving message from other core, FIFO size, 4x32 can read one of the word address to pop data to the FIFO; can also use 4x32 burst read from 0x020 to read 4 words from the FIFO.
    #[inline(always)]
    pub fn rxfifo(&self) -> RxfifoR {
        RxfifoR::new(self.bits)
    }
}
impl W {}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`rxwrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxwrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RxwrdSpec;
impl crate::RegisterSpec for RxwrdSpec {
    type Ux = u32;
}
///`read()` method returns [`rxwrd::R`](R) reader structure
impl crate::Readable for RxwrdSpec {}
///`write(|w| ..)` method takes [`rxwrd::W`](W) writer structure
impl crate::Writable for RxwrdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXWRD[%s] to value 0
impl crate::Resettable for RxwrdSpec {}
