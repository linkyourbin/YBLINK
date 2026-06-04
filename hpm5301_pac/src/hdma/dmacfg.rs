///Register `DMACfg` reader
pub type R = crate::R<DmacfgSpec>;
///Register `DMACfg` writer
pub type W = crate::W<DmacfgSpec>;
///Field `CHANNELNUM` reader - Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid
pub type ChannelnumR = crate::FieldReader;
///Field `FIFODEPTH` reader - FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid
pub type FifodepthR = crate::FieldReader;
///Field `REQNUM` reader - Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs
pub type ReqnumR = crate::FieldReader;
///Field `BUSNUM` reader - AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses
pub type BusnumR = crate::BitReader;
///Field `CORENUM` reader - DMA core number 0x0: 1 core 0x1: 2 cores
pub type CorenumR = crate::BitReader;
///Field `ADDRWIDTH` reader - AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid
pub type AddrwidthR = crate::FieldReader;
///Field `DATAWIDTH` reader - AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits
pub type DatawidthR = crate::FieldReader;
///Field `REQSYNC` reader - DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured
pub type ReqsyncR = crate::BitReader;
///Field `CHAINXFR` reader - Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured
pub type ChainxfrR = crate::BitReader;
impl R {
    ///Bits 0:3 - Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid
    #[inline(always)]
    pub fn channelnum(&self) -> ChannelnumR {
        ChannelnumR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:9 - FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid
    #[inline(always)]
    pub fn fifodepth(&self) -> FifodepthR {
        FifodepthR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 10:14 - Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs
    #[inline(always)]
    pub fn reqnum(&self) -> ReqnumR {
        ReqnumR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bit 15 - AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses
    #[inline(always)]
    pub fn busnum(&self) -> BusnumR {
        BusnumR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DMA core number 0x0: 1 core 0x1: 2 cores
    #[inline(always)]
    pub fn corenum(&self) -> CorenumR {
        CorenumR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:23 - AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid
    #[inline(always)]
    pub fn addrwidth(&self) -> AddrwidthR {
        AddrwidthR::new(((self.bits >> 17) & 0x7f) as u8)
    }
    ///Bits 24:25 - AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits
    #[inline(always)]
    pub fn datawidth(&self) -> DatawidthR {
        DatawidthR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 30 - DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured
    #[inline(always)]
    pub fn reqsync(&self) -> ReqsyncR {
        ReqsyncR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured
    #[inline(always)]
    pub fn chainxfr(&self) -> ChainxfrR {
        ChainxfrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
/**DMAC Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dmacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacfgSpec;
impl crate::RegisterSpec for DmacfgSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacfg::R`](R) reader structure
impl crate::Readable for DmacfgSpec {}
///`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure
impl crate::Writable for DmacfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACfg to value 0
impl crate::Resettable for DmacfgSpec {}
