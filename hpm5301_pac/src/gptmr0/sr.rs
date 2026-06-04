///Register `SR` reader
pub type R = crate::R<SrSpec>;
///Register `SR` writer
pub type W = crate::W<SrSpec>;
///Field `CH0RLDF` writer - channel 1 counter reload flag
pub type Ch0rldfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0CAPF` writer - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
pub type Ch0capfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0CMP0F` writer - channel 1 compare value 1 match flag
pub type Ch0cmp0fW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0CMP1F` writer - channel 1 compare value 1 match flag
pub type Ch0cmp1fW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1RLDF` writer - channel 1 counter reload flag
pub type Ch1rldfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1CAPF` writer - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
pub type Ch1capfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1CMP0F` writer - channel 1 compare value 1 match flag
pub type Ch1cmp0fW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1CMP1F` writer - channel 1 compare value 1 match flag
pub type Ch1cmp1fW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2RLDF` writer - channel 2 counter reload flag
pub type Ch2rldfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2CAPF` writer - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
pub type Ch2capfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2CMP0F` writer - channel 2 compare value 1 match flag
pub type Ch2cmp0fW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2CMP1F` writer - channel 2 compare value 1 match flag
pub type Ch2cmp1fW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3RLDF` writer - channel 3 counter reload flag
pub type Ch3rldfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3CAPF` writer - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
pub type Ch3capfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3CMP0F` writer - channel 3 compare value 1 match flag
pub type Ch3cmp0fW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3CMP1F` writer - channel 3 compare value 1 match flag
pub type Ch3cmp1fW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bit 0 - channel 1 counter reload flag
    #[inline(always)]
    pub fn ch0rldf(&mut self) -> Ch0rldfW<'_, SrSpec> {
        Ch0rldfW::new(self, 0)
    }
    ///Bit 1 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
    #[inline(always)]
    pub fn ch0capf(&mut self) -> Ch0capfW<'_, SrSpec> {
        Ch0capfW::new(self, 1)
    }
    ///Bit 2 - channel 1 compare value 1 match flag
    #[inline(always)]
    pub fn ch0cmp0f(&mut self) -> Ch0cmp0fW<'_, SrSpec> {
        Ch0cmp0fW::new(self, 2)
    }
    ///Bit 3 - channel 1 compare value 1 match flag
    #[inline(always)]
    pub fn ch0cmp1f(&mut self) -> Ch0cmp1fW<'_, SrSpec> {
        Ch0cmp1fW::new(self, 3)
    }
    ///Bit 4 - channel 1 counter reload flag
    #[inline(always)]
    pub fn ch1rldf(&mut self) -> Ch1rldfW<'_, SrSpec> {
        Ch1rldfW::new(self, 4)
    }
    ///Bit 5 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
    #[inline(always)]
    pub fn ch1capf(&mut self) -> Ch1capfW<'_, SrSpec> {
        Ch1capfW::new(self, 5)
    }
    ///Bit 6 - channel 1 compare value 1 match flag
    #[inline(always)]
    pub fn ch1cmp0f(&mut self) -> Ch1cmp0fW<'_, SrSpec> {
        Ch1cmp0fW::new(self, 6)
    }
    ///Bit 7 - channel 1 compare value 1 match flag
    #[inline(always)]
    pub fn ch1cmp1f(&mut self) -> Ch1cmp1fW<'_, SrSpec> {
        Ch1cmp1fW::new(self, 7)
    }
    ///Bit 8 - channel 2 counter reload flag
    #[inline(always)]
    pub fn ch2rldf(&mut self) -> Ch2rldfW<'_, SrSpec> {
        Ch2rldfW::new(self, 8)
    }
    ///Bit 9 - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
    #[inline(always)]
    pub fn ch2capf(&mut self) -> Ch2capfW<'_, SrSpec> {
        Ch2capfW::new(self, 9)
    }
    ///Bit 10 - channel 2 compare value 1 match flag
    #[inline(always)]
    pub fn ch2cmp0f(&mut self) -> Ch2cmp0fW<'_, SrSpec> {
        Ch2cmp0fW::new(self, 10)
    }
    ///Bit 11 - channel 2 compare value 1 match flag
    #[inline(always)]
    pub fn ch2cmp1f(&mut self) -> Ch2cmp1fW<'_, SrSpec> {
        Ch2cmp1fW::new(self, 11)
    }
    ///Bit 12 - channel 3 counter reload flag
    #[inline(always)]
    pub fn ch3rldf(&mut self) -> Ch3rldfW<'_, SrSpec> {
        Ch3rldfW::new(self, 12)
    }
    ///Bit 13 - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge.
    #[inline(always)]
    pub fn ch3capf(&mut self) -> Ch3capfW<'_, SrSpec> {
        Ch3capfW::new(self, 13)
    }
    ///Bit 14 - channel 3 compare value 1 match flag
    #[inline(always)]
    pub fn ch3cmp0f(&mut self) -> Ch3cmp0fW<'_, SrSpec> {
        Ch3cmp0fW::new(self, 14)
    }
    ///Bit 15 - channel 3 compare value 1 match flag
    #[inline(always)]
    pub fn ch3cmp1f(&mut self) -> Ch3cmp1fW<'_, SrSpec> {
        Ch3cmp1fW::new(self, 15)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SrSpec {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SrSpec {}
