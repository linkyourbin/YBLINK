///Register `ADVAN` reader
pub type R = crate::R<AdvanSpec>;
///Register `ADVAN` writer
pub type W = crate::W<AdvanSpec>;
///Field `POS_ONLY` reader - use positive compare polarity only
pub type PosOnlyR = crate::BitReader;
///Field `POS_ONLY` writer - use positive compare polarity only
pub type PosOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NEG_ONLY` reader - use negative compare polarity only
pub type NegOnlyR = crate::BitReader;
///Field `NEG_ONLY` writer - use negative compare polarity only
pub type NegOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAMPLING` reader - temperature sampling is working
pub type SamplingR = crate::BitReader;
///Field `ACTIVE_IRQ` reader - interrupt status of active mode
pub type ActiveIrqR = crate::BitReader;
///Field `ASYNC_IRQ` reader - interrupt status of asynchronous mode
pub type AsyncIrqR = crate::BitReader;
impl R {
    ///Bit 0 - use positive compare polarity only
    #[inline(always)]
    pub fn pos_only(&self) -> PosOnlyR {
        PosOnlyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - use negative compare polarity only
    #[inline(always)]
    pub fn neg_only(&self) -> NegOnlyR {
        NegOnlyR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - temperature sampling is working
    #[inline(always)]
    pub fn sampling(&self) -> SamplingR {
        SamplingR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - interrupt status of active mode
    #[inline(always)]
    pub fn active_irq(&self) -> ActiveIrqR {
        ActiveIrqR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - interrupt status of asynchronous mode
    #[inline(always)]
    pub fn async_irq(&self) -> AsyncIrqR {
        AsyncIrqR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - use positive compare polarity only
    #[inline(always)]
    pub fn pos_only(&mut self) -> PosOnlyW<'_, AdvanSpec> {
        PosOnlyW::new(self, 0)
    }
    ///Bit 1 - use negative compare polarity only
    #[inline(always)]
    pub fn neg_only(&mut self) -> NegOnlyW<'_, AdvanSpec> {
        NegOnlyW::new(self, 1)
    }
}
/**Advance configuration

You can [`read`](crate::Reg::read) this register and get [`advan::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`advan::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdvanSpec;
impl crate::RegisterSpec for AdvanSpec {
    type Ux = u32;
}
///`read()` method returns [`advan::R`](R) reader structure
impl crate::Readable for AdvanSpec {}
///`write(|w| ..)` method takes [`advan::W`](W) writer structure
impl crate::Writable for AdvanSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADVAN to value 0
impl crate::Resettable for AdvanSpec {}
