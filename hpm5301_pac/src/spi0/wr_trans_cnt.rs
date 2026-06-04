///Register `wr_trans_cnt` reader
pub type R = crate::R<WrTransCntSpec>;
///Register `wr_trans_cnt` writer
pub type W = crate::W<WrTransCntSpec>;
///Field `WRTRANCNT` reader - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
pub type WrtrancntR = crate::FieldReader<u32>;
///Field `WRTRANCNT` writer - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
pub type WrtrancntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
    #[inline(always)]
    pub fn wrtrancnt(&self) -> WrtrancntR {
        WrtrancntR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
    #[inline(always)]
    pub fn wrtrancnt(&mut self) -> WrtrancntW<'_, WrTransCntSpec> {
        WrtrancntW::new(self, 0)
    }
}
/**Transfer count for write data

You can [`read`](crate::Reg::read) this register and get [`wr_trans_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_trans_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WrTransCntSpec;
impl crate::RegisterSpec for WrTransCntSpec {
    type Ux = u32;
}
///`read()` method returns [`wr_trans_cnt::R`](R) reader structure
impl crate::Readable for WrTransCntSpec {}
///`write(|w| ..)` method takes [`wr_trans_cnt::W`](W) writer structure
impl crate::Writable for WrTransCntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets wr_trans_cnt to value 0
impl crate::Resettable for WrTransCntSpec {}
