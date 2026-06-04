///Register `RNG` reader
pub type R = crate::R<RngSpec>;
///Register `RNG` writer
pub type W = crate::W<RngSpec>;
///Field `RNG_XOR` reader - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG
pub type RngXorR = crate::BitReader;
///Field `RNG_XOR` writer - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG
pub type RngXorW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLOCK_RNG_XOR` reader - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software
pub type BlockRngXorR = crate::BitReader;
///Field `BLOCK_RNG_XOR` writer - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software
pub type BlockRngXorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG
    #[inline(always)]
    pub fn rng_xor(&self) -> RngXorR {
        RngXorR::new((self.bits & 1) != 0)
    }
    ///Bit 16 - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software
    #[inline(always)]
    pub fn block_rng_xor(&self) -> BlockRngXorR {
        BlockRngXorR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG
    #[inline(always)]
    pub fn rng_xor(&mut self) -> RngXorW<'_, RngSpec> {
        RngXorW::new(self, 0)
    }
    ///Bit 16 - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software
    #[inline(always)]
    pub fn block_rng_xor(&mut self) -> BlockRngXorW<'_, RngSpec> {
        BlockRngXorW::new(self, 16)
    }
}
/**Random number interface behavior

You can [`read`](crate::Reg::read) this register and get [`rng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RngSpec;
impl crate::RegisterSpec for RngSpec {
    type Ux = u32;
}
///`read()` method returns [`rng::R`](R) reader structure
impl crate::Readable for RngSpec {}
///`write(|w| ..)` method takes [`rng::W`](W) writer structure
impl crate::Writable for RngSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RNG to value 0
impl crate::Resettable for RngSpec {}
