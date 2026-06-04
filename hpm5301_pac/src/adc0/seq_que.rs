///Register `SEQ_QUE[%s]` reader
pub type R = crate::R<SeqQueSpec>;
///Register `SEQ_QUE[%s]` writer
pub type W = crate::W<SeqQueSpec>;
///Field `CHAN_NUM_4_0` reader - channel number for current conversion
pub type ChanNum4_0R = crate::FieldReader;
///Field `CHAN_NUM_4_0` writer - channel number for current conversion
pub type ChanNum4_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SEQ_INT_EN` reader - interrupt enable for current conversion
pub type SeqIntEnR = crate::BitReader;
///Field `SEQ_INT_EN` writer - interrupt enable for current conversion
pub type SeqIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - channel number for current conversion
    #[inline(always)]
    pub fn chan_num_4_0(&self) -> ChanNum4_0R {
        ChanNum4_0R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - interrupt enable for current conversion
    #[inline(always)]
    pub fn seq_int_en(&self) -> SeqIntEnR {
        SeqIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - channel number for current conversion
    #[inline(always)]
    pub fn chan_num_4_0(&mut self) -> ChanNum4_0W<'_, SeqQueSpec> {
        ChanNum4_0W::new(self, 0)
    }
    ///Bit 5 - interrupt enable for current conversion
    #[inline(always)]
    pub fn seq_int_en(&mut self) -> SeqIntEnW<'_, SeqQueSpec> {
        SeqIntEnW::new(self, 5)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`seq_que::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_que::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SeqQueSpec;
impl crate::RegisterSpec for SeqQueSpec {
    type Ux = u32;
}
///`read()` method returns [`seq_que::R`](R) reader structure
impl crate::Readable for SeqQueSpec {}
///`write(|w| ..)` method takes [`seq_que::W`](W) writer structure
impl crate::Writable for SeqQueSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEQ_QUE[%s] to value 0
impl crate::Resettable for SeqQueSpec {}
