///Register `GPTIMER0CTRL` reader
pub type R = crate::R<Gptimer0ctrlSpec>;
///Register `GPTIMER0CTRL` writer
pub type W = crate::W<Gptimer0ctrlSpec>;
///Field `GPTCNT` reader - GPTCNT General Purpose Timer Counter. This field is the count value of the countdown timer.
pub type GptcntR = crate::FieldReader<u32>;
///Field `GPTMODE` reader - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode
pub type GptmodeR = crate::BitReader;
///Field `GPTMODE` writer - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode
pub type GptmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPTRST` writer - GPTRST General Purpose Timer Reset 0 - No action 1 - Load counter value from GPTLD bits in n_GPTIMER0LD
pub type GptrstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPTRUN` reader - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run
pub type GptrunR = crate::BitReader;
///Field `GPTRUN` writer - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run
pub type GptrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:23 - GPTCNT General Purpose Timer Counter. This field is the count value of the countdown timer.
    #[inline(always)]
    pub fn gptcnt(&self) -> GptcntR {
        GptcntR::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode
    #[inline(always)]
    pub fn gptmode(&self) -> GptmodeR {
        GptmodeR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 31 - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run
    #[inline(always)]
    pub fn gptrun(&self) -> GptrunR {
        GptrunR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode
    #[inline(always)]
    pub fn gptmode(&mut self) -> GptmodeW<'_, Gptimer0ctrlSpec> {
        GptmodeW::new(self, 24)
    }
    ///Bit 30 - GPTRST General Purpose Timer Reset 0 - No action 1 - Load counter value from GPTLD bits in n_GPTIMER0LD
    #[inline(always)]
    pub fn gptrst(&mut self) -> GptrstW<'_, Gptimer0ctrlSpec> {
        GptrstW::new(self, 30)
    }
    ///Bit 31 - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run
    #[inline(always)]
    pub fn gptrun(&mut self) -> GptrunW<'_, Gptimer0ctrlSpec> {
        GptrunW::new(self, 31)
    }
}
/**General Purpose Timer #0 Controller Register

You can [`read`](crate::Reg::read) this register and get [`gptimer0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptimer0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Gptimer0ctrlSpec;
impl crate::RegisterSpec for Gptimer0ctrlSpec {
    type Ux = u32;
}
///`read()` method returns [`gptimer0ctrl::R`](R) reader structure
impl crate::Readable for Gptimer0ctrlSpec {}
///`write(|w| ..)` method takes [`gptimer0ctrl::W`](W) writer structure
impl crate::Writable for Gptimer0ctrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPTIMER0CTRL to value 0
impl crate::Resettable for Gptimer0ctrlSpec {}
