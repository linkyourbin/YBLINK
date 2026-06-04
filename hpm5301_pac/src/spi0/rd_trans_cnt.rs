///Register `rd_trans_cnt` reader
pub type R = crate::R<RdTransCntSpec>;
///Register `rd_trans_cnt` writer
pub type W = crate::W<RdTransCntSpec>;
///Field `RDTRANCNT` reader - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
pub type RdtrancntR = crate::FieldReader<u32>;
///Field `RDTRANCNT` writer - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
pub type RdtrancntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
    #[inline(always)]
    pub fn rdtrancnt(&self) -> RdtrancntR {
        RdtrancntR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
    #[inline(always)]
    pub fn rdtrancnt(&mut self) -> RdtrancntW<'_, RdTransCntSpec> {
        RdtrancntW::new(self, 0)
    }
}
/**Transfer count for read data

You can [`read`](crate::Reg::read) this register and get [`rd_trans_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_trans_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdTransCntSpec;
impl crate::RegisterSpec for RdTransCntSpec {
    type Ux = u32;
}
///`read()` method returns [`rd_trans_cnt::R`](R) reader structure
impl crate::Readable for RdTransCntSpec {}
///`write(|w| ..)` method takes [`rd_trans_cnt::W`](W) writer structure
impl crate::Writable for RdTransCntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets rd_trans_cnt to value 0
impl crate::Resettable for RdTransCntSpec {}
