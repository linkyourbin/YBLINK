///Register `seq_dma_cfg` reader
pub type R = crate::R<SeqDmaCfgSpec>;
///Register `seq_dma_cfg` writer
pub type W = crate::W<SeqDmaCfgSpec>;
///Field `BUF_LEN` reader - dma buffer length, after write to (tar_addr\[31:2\]+buf_len)*4, the next dma address will be tar_addr\[31:2\]*4 0 for 4byte; 0xFFF for 16kbyte.
pub type BufLenR = crate::FieldReader<u16>;
///Field `BUF_LEN` writer - dma buffer length, after write to (tar_addr\[31:2\]+buf_len)*4, the next dma address will be tar_addr\[31:2\]*4 0 for 4byte; 0xFFF for 16kbyte.
pub type BufLenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `STOP_EN` reader - set to stop dma if reach the stop_pos
pub type StopEnR = crate::BitReader;
///Field `STOP_EN` writer - set to stop dma if reach the stop_pos
pub type StopEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_RST` reader - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst
pub type DmaRstR = crate::BitReader;
///Field `DMA_RST` writer - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst
pub type DmaRstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP_POS` reader - if stop_en is set, SW is responsible to update this field to the next read point, HW should not write data to this point since it's not read out by SW yet
pub type StopPosR = crate::FieldReader<u16>;
///Field `STOP_POS` writer - if stop_en is set, SW is responsible to update this field to the next read point, HW should not write data to this point since it's not read out by SW yet
pub type StopPosW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - dma buffer length, after write to (tar_addr\[31:2\]+buf_len)*4, the next dma address will be tar_addr\[31:2\]*4 0 for 4byte; 0xFFF for 16kbyte.
    #[inline(always)]
    pub fn buf_len(&self) -> BufLenR {
        BufLenR::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - set to stop dma if reach the stop_pos
    #[inline(always)]
    pub fn stop_en(&self) -> StopEnR {
        StopEnR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst
    #[inline(always)]
    pub fn dma_rst(&self) -> DmaRstR {
        DmaRstR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:27 - if stop_en is set, SW is responsible to update this field to the next read point, HW should not write data to this point since it's not read out by SW yet
    #[inline(always)]
    pub fn stop_pos(&self) -> StopPosR {
        StopPosR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - dma buffer length, after write to (tar_addr\[31:2\]+buf_len)*4, the next dma address will be tar_addr\[31:2\]*4 0 for 4byte; 0xFFF for 16kbyte.
    #[inline(always)]
    pub fn buf_len(&mut self) -> BufLenW<'_, SeqDmaCfgSpec> {
        BufLenW::new(self, 0)
    }
    ///Bit 12 - set to stop dma if reach the stop_pos
    #[inline(always)]
    pub fn stop_en(&mut self) -> StopEnW<'_, SeqDmaCfgSpec> {
        StopEnW::new(self, 12)
    }
    ///Bit 13 - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DmaRstW<'_, SeqDmaCfgSpec> {
        DmaRstW::new(self, 13)
    }
    ///Bits 16:27 - if stop_en is set, SW is responsible to update this field to the next read point, HW should not write data to this point since it's not read out by SW yet
    #[inline(always)]
    pub fn stop_pos(&mut self) -> StopPosW<'_, SeqDmaCfgSpec> {
        StopPosW::new(self, 16)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`seq_dma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_dma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SeqDmaCfgSpec;
impl crate::RegisterSpec for SeqDmaCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`seq_dma_cfg::R`](R) reader structure
impl crate::Readable for SeqDmaCfgSpec {}
///`write(|w| ..)` method takes [`seq_dma_cfg::W`](W) writer structure
impl crate::Writable for SeqDmaCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets seq_dma_cfg to value 0
impl crate::Resettable for SeqDmaCfgSpec {}
