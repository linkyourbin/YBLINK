///Register `Cfg` reader
pub type R = crate::R<CfgSpec>;
///Register `Cfg` writer
pub type W = crate::W<CfgSpec>;
///Field `FIFOSIZE` reader - The depth of RXFIFO and TXFIFO 0: 16-byte FIFO 1: 32-byte FIFO 2: 64-byte FIFO 3: 128-byte FIFO
pub type FifosizeR = crate::FieldReader;
impl R {
    ///Bits 0:1 - The depth of RXFIFO and TXFIFO 0: 16-byte FIFO 1: 32-byte FIFO 2: 64-byte FIFO 3: 128-byte FIFO
    #[inline(always)]
    pub fn fifosize(&self) -> FifosizeR {
        FifosizeR::new((self.bits & 3) as u8)
    }
}
impl W {}
/**Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfg::R`](R) reader structure
impl crate::Readable for CfgSpec {}
///`write(|w| ..)` method takes [`cfg::W`](W) writer structure
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Cfg to value 0
impl crate::Resettable for CfgSpec {}
