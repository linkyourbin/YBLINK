///Register `seq_wr_addr` reader
pub type R = crate::R<SeqWrAddrSpec>;
///Register `seq_wr_addr` writer
pub type W = crate::W<SeqWrAddrSpec>;
///Field `SEQ_WR_POINTER` reader - HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4
pub type SeqWrPointerR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4
    #[inline(always)]
    pub fn seq_wr_pointer(&self) -> SeqWrPointerR {
        SeqWrPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`seq_wr_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_wr_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SeqWrAddrSpec;
impl crate::RegisterSpec for SeqWrAddrSpec {
    type Ux = u32;
}
///`read()` method returns [`seq_wr_addr::R`](R) reader structure
impl crate::Readable for SeqWrAddrSpec {}
///`write(|w| ..)` method takes [`seq_wr_addr::W`](W) writer structure
impl crate::Writable for SeqWrAddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets seq_wr_addr to value 0
impl crate::Resettable for SeqWrAddrSpec {}
