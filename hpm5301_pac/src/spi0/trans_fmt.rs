///Register `TransFmt` reader
pub type R = crate::R<TransFmtSpec>;
///Register `TransFmt` writer
pub type W = crate::W<TransFmtSpec>;
///Field `CPHA` reader - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges
pub type CphaR = crate::BitReader;
///Field `CPHA` writer - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPOL` reader - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states
pub type CpolR = crate::BitReader;
///Field `CPOL` writer - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLVMODE` reader - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode
pub type SlvmodeR = crate::BitReader;
///Field `SLVMODE` writer - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode
pub type SlvmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSB` reader - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first
pub type LsbR = crate::BitReader;
///Field `LSB` writer - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first
pub type LsbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOSIBIDIR` reader - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two
pub type MosibidirR = crate::BitReader;
///Field `MOSIBIDIR` writer - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two
pub type MosibidirW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAMERGE` reader - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed.
pub type DatamergeR = crate::BitReader;
///Field `DATAMERGE` writer - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed.
pub type DatamergeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATALEN` reader - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)
pub type DatalenR = crate::FieldReader;
///Field `DATALEN` writer - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)
pub type DatalenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADDRLEN` reader - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes
pub type AddrlenR = crate::FieldReader;
///Field `ADDRLEN` writer - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes
pub type AddrlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode
    #[inline(always)]
    pub fn slvmode(&self) -> SlvmodeR {
        SlvmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first
    #[inline(always)]
    pub fn lsb(&self) -> LsbR {
        LsbR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two
    #[inline(always)]
    pub fn mosibidir(&self) -> MosibidirR {
        MosibidirR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed.
    #[inline(always)]
    pub fn datamerge(&self) -> DatamergeR {
        DatamergeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)
    #[inline(always)]
    pub fn datalen(&self) -> DatalenR {
        DatalenR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:17 - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes
    #[inline(always)]
    pub fn addrlen(&self) -> AddrlenR {
        AddrlenR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, TransFmtSpec> {
        CphaW::new(self, 0)
    }
    ///Bit 1 - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, TransFmtSpec> {
        CpolW::new(self, 1)
    }
    ///Bit 2 - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode
    #[inline(always)]
    pub fn slvmode(&mut self) -> SlvmodeW<'_, TransFmtSpec> {
        SlvmodeW::new(self, 2)
    }
    ///Bit 3 - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first
    #[inline(always)]
    pub fn lsb(&mut self) -> LsbW<'_, TransFmtSpec> {
        LsbW::new(self, 3)
    }
    ///Bit 4 - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two
    #[inline(always)]
    pub fn mosibidir(&mut self) -> MosibidirW<'_, TransFmtSpec> {
        MosibidirW::new(self, 4)
    }
    ///Bit 7 - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed.
    #[inline(always)]
    pub fn datamerge(&mut self) -> DatamergeW<'_, TransFmtSpec> {
        DatamergeW::new(self, 7)
    }
    ///Bits 8:12 - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)
    #[inline(always)]
    pub fn datalen(&mut self) -> DatalenW<'_, TransFmtSpec> {
        DatalenW::new(self, 8)
    }
    ///Bits 16:17 - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes
    #[inline(always)]
    pub fn addrlen(&mut self) -> AddrlenW<'_, TransFmtSpec> {
        AddrlenW::new(self, 16)
    }
}
/**Transfer Format Register

You can [`read`](crate::Reg::read) this register and get [`trans_fmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trans_fmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TransFmtSpec;
impl crate::RegisterSpec for TransFmtSpec {
    type Ux = u32;
}
///`read()` method returns [`trans_fmt::R`](R) reader structure
impl crate::Readable for TransFmtSpec {}
///`write(|w| ..)` method takes [`trans_fmt::W`](W) writer structure
impl crate::Writable for TransFmtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TransFmt to value 0x0002_0780
impl crate::Resettable for TransFmtSpec {
    const RESET_VALUE: u32 = 0x0002_0780;
}
