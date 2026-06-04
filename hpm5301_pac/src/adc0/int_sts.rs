///Register `int_sts` reader
pub type R = crate::R<IntStsSpec>;
///Register `int_sts` writer
pub type W = crate::W<IntStsSpec>;
///Field `WDOG` reader - set if one chanel watch dog event triggered
pub type WdogR = crate::FieldReader<u16>;
///Field `WDOG` writer - set if one chanel watch dog event triggered
pub type WdogW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AHB_ERR` reader - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr
pub type AhbErrR = crate::BitReader;
///Field `AHB_ERR` writer - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr
pub type AhbErrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_FIFO_FULL` reader - DMA fifo full interrupt, user need to check clock frequency if it's set.
pub type DmaFifoFullR = crate::BitReader;
///Field `DMA_FIFO_FULL` writer - DMA fifo full interrupt, user need to check clock frequency if it's set.
pub type DmaFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_CVC` reader - one conversion complete in seq_queue if related seq_int_en is set
pub type SeqCvcR = crate::BitReader;
///Field `SEQ_CVC` writer - one conversion complete in seq_queue if related seq_int_en is set
pub type SeqCvcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_CMPT` reader - the whole sequence complete interrupt
pub type SeqCmptR = crate::BitReader;
///Field `SEQ_CMPT` writer - the whole sequence complete interrupt
pub type SeqCmptW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_DMAABT` reader - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set
pub type SeqDmaabtR = crate::BitReader;
///Field `SEQ_DMAABT` writer - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set
pub type SeqDmaabtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_HW_CFLCT` reader - No description available
pub type SeqHwCflctR = crate::BitReader;
///Field `SEQ_HW_CFLCT` writer - No description available
pub type SeqHwCflctW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_SW_CFLCT` reader - sequence queue conflict interrupt, set if HW or SW trigger received during conversion
pub type SeqSwCflctR = crate::BitReader;
///Field `SEQ_SW_CFLCT` writer - sequence queue conflict interrupt, set if HW or SW trigger received during conversion
pub type SeqSwCflctW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READ_CFLCT` reader - read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel
pub type ReadCflctR = crate::BitReader;
///Field `READ_CFLCT` writer - read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel
pub type ReadCflctW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIG_HW_CFLCT` reader - No description available
pub type TrigHwCflctR = crate::BitReader;
///Field `TRIG_HW_CFLCT` writer - No description available
pub type TrigHwCflctW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIG_SW_CFLCT` reader - No description available
pub type TrigSwCflctR = crate::BitReader;
///Field `TRIG_SW_CFLCT` writer - No description available
pub type TrigSwCflctW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIG_CMPT` reader - interrupt for one trigger conversion complete if enabled
pub type TrigCmptR = crate::BitReader;
///Field `TRIG_CMPT` writer - interrupt for one trigger conversion complete if enabled
pub type TrigCmptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - set if one chanel watch dog event triggered
    #[inline(always)]
    pub fn wdog(&self) -> WdogR {
        WdogR::new((self.bits & 0xffff) as u16)
    }
    ///Bit 21 - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr
    #[inline(always)]
    pub fn ahb_err(&self) -> AhbErrR {
        AhbErrR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DMA fifo full interrupt, user need to check clock frequency if it's set.
    #[inline(always)]
    pub fn dma_fifo_full(&self) -> DmaFifoFullR {
        DmaFifoFullR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - one conversion complete in seq_queue if related seq_int_en is set
    #[inline(always)]
    pub fn seq_cvc(&self) -> SeqCvcR {
        SeqCvcR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - the whole sequence complete interrupt
    #[inline(always)]
    pub fn seq_cmpt(&self) -> SeqCmptR {
        SeqCmptR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set
    #[inline(always)]
    pub fn seq_dmaabt(&self) -> SeqDmaabtR {
        SeqDmaabtR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - No description available
    #[inline(always)]
    pub fn seq_hw_cflct(&self) -> SeqHwCflctR {
        SeqHwCflctR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - sequence queue conflict interrupt, set if HW or SW trigger received during conversion
    #[inline(always)]
    pub fn seq_sw_cflct(&self) -> SeqSwCflctR {
        SeqSwCflctR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel
    #[inline(always)]
    pub fn read_cflct(&self) -> ReadCflctR {
        ReadCflctR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - No description available
    #[inline(always)]
    pub fn trig_hw_cflct(&self) -> TrigHwCflctR {
        TrigHwCflctR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - No description available
    #[inline(always)]
    pub fn trig_sw_cflct(&self) -> TrigSwCflctR {
        TrigSwCflctR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - interrupt for one trigger conversion complete if enabled
    #[inline(always)]
    pub fn trig_cmpt(&self) -> TrigCmptR {
        TrigCmptR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - set if one chanel watch dog event triggered
    #[inline(always)]
    pub fn wdog(&mut self) -> WdogW<'_, IntStsSpec> {
        WdogW::new(self, 0)
    }
    ///Bit 21 - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr
    #[inline(always)]
    pub fn ahb_err(&mut self) -> AhbErrW<'_, IntStsSpec> {
        AhbErrW::new(self, 21)
    }
    ///Bit 22 - DMA fifo full interrupt, user need to check clock frequency if it's set.
    #[inline(always)]
    pub fn dma_fifo_full(&mut self) -> DmaFifoFullW<'_, IntStsSpec> {
        DmaFifoFullW::new(self, 22)
    }
    ///Bit 23 - one conversion complete in seq_queue if related seq_int_en is set
    #[inline(always)]
    pub fn seq_cvc(&mut self) -> SeqCvcW<'_, IntStsSpec> {
        SeqCvcW::new(self, 23)
    }
    ///Bit 24 - the whole sequence complete interrupt
    #[inline(always)]
    pub fn seq_cmpt(&mut self) -> SeqCmptW<'_, IntStsSpec> {
        SeqCmptW::new(self, 24)
    }
    ///Bit 25 - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set
    #[inline(always)]
    pub fn seq_dmaabt(&mut self) -> SeqDmaabtW<'_, IntStsSpec> {
        SeqDmaabtW::new(self, 25)
    }
    ///Bit 26 - No description available
    #[inline(always)]
    pub fn seq_hw_cflct(&mut self) -> SeqHwCflctW<'_, IntStsSpec> {
        SeqHwCflctW::new(self, 26)
    }
    ///Bit 27 - sequence queue conflict interrupt, set if HW or SW trigger received during conversion
    #[inline(always)]
    pub fn seq_sw_cflct(&mut self) -> SeqSwCflctW<'_, IntStsSpec> {
        SeqSwCflctW::new(self, 27)
    }
    ///Bit 28 - read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel
    #[inline(always)]
    pub fn read_cflct(&mut self) -> ReadCflctW<'_, IntStsSpec> {
        ReadCflctW::new(self, 28)
    }
    ///Bit 29 - No description available
    #[inline(always)]
    pub fn trig_hw_cflct(&mut self) -> TrigHwCflctW<'_, IntStsSpec> {
        TrigHwCflctW::new(self, 29)
    }
    ///Bit 30 - No description available
    #[inline(always)]
    pub fn trig_sw_cflct(&mut self) -> TrigSwCflctW<'_, IntStsSpec> {
        TrigSwCflctW::new(self, 30)
    }
    ///Bit 31 - interrupt for one trigger conversion complete if enabled
    #[inline(always)]
    pub fn trig_cmpt(&mut self) -> TrigCmptW<'_, IntStsSpec> {
        TrigCmptW::new(self, 31)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntStsSpec;
impl crate::RegisterSpec for IntStsSpec {
    type Ux = u32;
}
///`read()` method returns [`int_sts::R`](R) reader structure
impl crate::Readable for IntStsSpec {}
///`write(|w| ..)` method takes [`int_sts::W`](W) writer structure
impl crate::Writable for IntStsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets int_sts to value 0
impl crate::Resettable for IntStsSpec {}
