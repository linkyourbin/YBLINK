///Register `Config` reader
pub type R = crate::R<ConfigSpec>;
///Register `Config` writer
pub type W = crate::W<ConfigSpec>;
///Field `RXFIFOSIZE` reader - Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words
pub type RxfifosizeR = crate::FieldReader;
///Field `TXFIFOSIZE` reader - Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words
pub type TxfifosizeR = crate::FieldReader;
///Field `DUALSPI` reader - Support for Dual I/O SPI
pub type DualspiR = crate::BitReader;
///Field `QUADSPI` reader - Support for Quad I/O SPI
pub type QuadspiR = crate::BitReader;
///Field `SLAVE` reader - Support for SPI Slave mode
pub type SlaveR = crate::BitReader;
impl R {
    ///Bits 0:3 - Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words
    #[inline(always)]
    pub fn rxfifosize(&self) -> RxfifosizeR {
        RxfifosizeR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words
    #[inline(always)]
    pub fn txfifosize(&self) -> TxfifosizeR {
        TxfifosizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Support for Dual I/O SPI
    #[inline(always)]
    pub fn dualspi(&self) -> DualspiR {
        DualspiR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Support for Quad I/O SPI
    #[inline(always)]
    pub fn quadspi(&self) -> QuadspiR {
        QuadspiR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - Support for SPI Slave mode
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {}
/**Configuration Register

You can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
///`read()` method returns [`config::R`](R) reader structure
impl crate::Readable for ConfigSpec {}
///`write(|w| ..)` method takes [`config::W`](W) writer structure
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Config to value 0x4311
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x4311;
}
