///Register `seq_high_cfg` reader
pub type R = crate::R<SeqHighCfgSpec>;
///Register `seq_high_cfg` writer
pub type W = crate::W<SeqHighCfgSpec>;
///Field `BUF_LEN_HIGH` reader - No description available
pub type BufLenHighR = crate::FieldReader<u16>;
///Field `BUF_LEN_HIGH` writer - No description available
pub type BufLenHighW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `STOP_POS_HIGH` reader - No description available
pub type StopPosHighR = crate::FieldReader<u16>;
///Field `STOP_POS_HIGH` writer - No description available
pub type StopPosHighW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - No description available
    #[inline(always)]
    pub fn buf_len_high(&self) -> BufLenHighR {
        BufLenHighR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:23 - No description available
    #[inline(always)]
    pub fn stop_pos_high(&self) -> StopPosHighR {
        StopPosHighR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - No description available
    #[inline(always)]
    pub fn buf_len_high(&mut self) -> BufLenHighW<'_, SeqHighCfgSpec> {
        BufLenHighW::new(self, 0)
    }
    ///Bits 12:23 - No description available
    #[inline(always)]
    pub fn stop_pos_high(&mut self) -> StopPosHighW<'_, SeqHighCfgSpec> {
        StopPosHighW::new(self, 12)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`seq_high_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_high_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SeqHighCfgSpec;
impl crate::RegisterSpec for SeqHighCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`seq_high_cfg::R`](R) reader structure
impl crate::Readable for SeqHighCfgSpec {}
///`write(|w| ..)` method takes [`seq_high_cfg::W`](W) writer structure
impl crate::Writable for SeqHighCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets seq_high_cfg to value 0
impl crate::Resettable for SeqHighCfgSpec {}
