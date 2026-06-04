///Register `IRQEN` reader
pub type R = crate::R<IrqenSpec>;
///Register `IRQEN` writer
pub type W = crate::W<IrqenSpec>;
///Field `CH0RLDEN` reader - 1- generate interrupt request when ch0rldf flag is set
pub type Ch0rldenR = crate::BitReader;
///Field `CH0RLDEN` writer - 1- generate interrupt request when ch0rldf flag is set
pub type Ch0rldenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0CAPEN` reader - 1- generate interrupt request when ch0capf flag is set
pub type Ch0capenR = crate::BitReader;
///Field `CH0CAPEN` writer - 1- generate interrupt request when ch0capf flag is set
pub type Ch0capenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0CMP0EN` reader - 1- generate interrupt request when ch0cmp0f flag is set
pub type Ch0cmp0enR = crate::BitReader;
///Field `CH0CMP0EN` writer - 1- generate interrupt request when ch0cmp0f flag is set
pub type Ch0cmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0CMP1EN` reader - 1- generate interrupt request when ch0cmp1f flag is set
pub type Ch0cmp1enR = crate::BitReader;
///Field `CH0CMP1EN` writer - 1- generate interrupt request when ch0cmp1f flag is set
pub type Ch0cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1RLDEN` reader - 1- generate interrupt request when ch1rldf flag is set
pub type Ch1rldenR = crate::BitReader;
///Field `CH1RLDEN` writer - 1- generate interrupt request when ch1rldf flag is set
pub type Ch1rldenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1CAPEN` reader - 1- generate interrupt request when ch1capf flag is set
pub type Ch1capenR = crate::BitReader;
///Field `CH1CAPEN` writer - 1- generate interrupt request when ch1capf flag is set
pub type Ch1capenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1CMP0EN` reader - 1- generate interrupt request when ch1cmp0f flag is set
pub type Ch1cmp0enR = crate::BitReader;
///Field `CH1CMP0EN` writer - 1- generate interrupt request when ch1cmp0f flag is set
pub type Ch1cmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1CMP1EN` reader - 1- generate interrupt request when ch1cmp1f flag is set
pub type Ch1cmp1enR = crate::BitReader;
///Field `CH1CMP1EN` writer - 1- generate interrupt request when ch1cmp1f flag is set
pub type Ch1cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2RLDEN` reader - 1- generate interrupt request when ch2rldf flag is set
pub type Ch2rldenR = crate::BitReader;
///Field `CH2RLDEN` writer - 1- generate interrupt request when ch2rldf flag is set
pub type Ch2rldenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2CAPEN` reader - 1- generate interrupt request when ch2capf flag is set
pub type Ch2capenR = crate::BitReader;
///Field `CH2CAPEN` writer - 1- generate interrupt request when ch2capf flag is set
pub type Ch2capenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2CMP0EN` reader - 1- generate interrupt request when ch2cmp0f flag is set
pub type Ch2cmp0enR = crate::BitReader;
///Field `CH2CMP0EN` writer - 1- generate interrupt request when ch2cmp0f flag is set
pub type Ch2cmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2CMP1EN` reader - 1- generate interrupt request when ch2cmp1f flag is set
pub type Ch2cmp1enR = crate::BitReader;
///Field `CH2CMP1EN` writer - 1- generate interrupt request when ch2cmp1f flag is set
pub type Ch2cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3RLDEN` reader - 1- generate interrupt request when ch3rldf flag is set
pub type Ch3rldenR = crate::BitReader;
///Field `CH3RLDEN` writer - 1- generate interrupt request when ch3rldf flag is set
pub type Ch3rldenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3CAPEN` reader - 1- generate interrupt request when ch3capf flag is set
pub type Ch3capenR = crate::BitReader;
///Field `CH3CAPEN` writer - 1- generate interrupt request when ch3capf flag is set
pub type Ch3capenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3CMP0EN` reader - 1- generate interrupt request when ch3cmp0f flag is set
pub type Ch3cmp0enR = crate::BitReader;
///Field `CH3CMP0EN` writer - 1- generate interrupt request when ch3cmp0f flag is set
pub type Ch3cmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3CMP1EN` reader - 1- generate interrupt request when ch3cmp1f flag is set
pub type Ch3cmp1enR = crate::BitReader;
///Field `CH3CMP1EN` writer - 1- generate interrupt request when ch3cmp1f flag is set
pub type Ch3cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1- generate interrupt request when ch0rldf flag is set
    #[inline(always)]
    pub fn ch0rlden(&self) -> Ch0rldenR {
        Ch0rldenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1- generate interrupt request when ch0capf flag is set
    #[inline(always)]
    pub fn ch0capen(&self) -> Ch0capenR {
        Ch0capenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 1- generate interrupt request when ch0cmp0f flag is set
    #[inline(always)]
    pub fn ch0cmp0en(&self) -> Ch0cmp0enR {
        Ch0cmp0enR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1- generate interrupt request when ch0cmp1f flag is set
    #[inline(always)]
    pub fn ch0cmp1en(&self) -> Ch0cmp1enR {
        Ch0cmp1enR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 1- generate interrupt request when ch1rldf flag is set
    #[inline(always)]
    pub fn ch1rlden(&self) -> Ch1rldenR {
        Ch1rldenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 1- generate interrupt request when ch1capf flag is set
    #[inline(always)]
    pub fn ch1capen(&self) -> Ch1capenR {
        Ch1capenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1- generate interrupt request when ch1cmp0f flag is set
    #[inline(always)]
    pub fn ch1cmp0en(&self) -> Ch1cmp0enR {
        Ch1cmp0enR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1- generate interrupt request when ch1cmp1f flag is set
    #[inline(always)]
    pub fn ch1cmp1en(&self) -> Ch1cmp1enR {
        Ch1cmp1enR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 1- generate interrupt request when ch2rldf flag is set
    #[inline(always)]
    pub fn ch2rlden(&self) -> Ch2rldenR {
        Ch2rldenR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 1- generate interrupt request when ch2capf flag is set
    #[inline(always)]
    pub fn ch2capen(&self) -> Ch2capenR {
        Ch2capenR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 1- generate interrupt request when ch2cmp0f flag is set
    #[inline(always)]
    pub fn ch2cmp0en(&self) -> Ch2cmp0enR {
        Ch2cmp0enR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 1- generate interrupt request when ch2cmp1f flag is set
    #[inline(always)]
    pub fn ch2cmp1en(&self) -> Ch2cmp1enR {
        Ch2cmp1enR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 1- generate interrupt request when ch3rldf flag is set
    #[inline(always)]
    pub fn ch3rlden(&self) -> Ch3rldenR {
        Ch3rldenR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - 1- generate interrupt request when ch3capf flag is set
    #[inline(always)]
    pub fn ch3capen(&self) -> Ch3capenR {
        Ch3capenR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - 1- generate interrupt request when ch3cmp0f flag is set
    #[inline(always)]
    pub fn ch3cmp0en(&self) -> Ch3cmp0enR {
        Ch3cmp0enR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - 1- generate interrupt request when ch3cmp1f flag is set
    #[inline(always)]
    pub fn ch3cmp1en(&self) -> Ch3cmp1enR {
        Ch3cmp1enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - 1- generate interrupt request when ch0rldf flag is set
    #[inline(always)]
    pub fn ch0rlden(&mut self) -> Ch0rldenW<'_, IrqenSpec> {
        Ch0rldenW::new(self, 0)
    }
    ///Bit 1 - 1- generate interrupt request when ch0capf flag is set
    #[inline(always)]
    pub fn ch0capen(&mut self) -> Ch0capenW<'_, IrqenSpec> {
        Ch0capenW::new(self, 1)
    }
    ///Bit 2 - 1- generate interrupt request when ch0cmp0f flag is set
    #[inline(always)]
    pub fn ch0cmp0en(&mut self) -> Ch0cmp0enW<'_, IrqenSpec> {
        Ch0cmp0enW::new(self, 2)
    }
    ///Bit 3 - 1- generate interrupt request when ch0cmp1f flag is set
    #[inline(always)]
    pub fn ch0cmp1en(&mut self) -> Ch0cmp1enW<'_, IrqenSpec> {
        Ch0cmp1enW::new(self, 3)
    }
    ///Bit 4 - 1- generate interrupt request when ch1rldf flag is set
    #[inline(always)]
    pub fn ch1rlden(&mut self) -> Ch1rldenW<'_, IrqenSpec> {
        Ch1rldenW::new(self, 4)
    }
    ///Bit 5 - 1- generate interrupt request when ch1capf flag is set
    #[inline(always)]
    pub fn ch1capen(&mut self) -> Ch1capenW<'_, IrqenSpec> {
        Ch1capenW::new(self, 5)
    }
    ///Bit 6 - 1- generate interrupt request when ch1cmp0f flag is set
    #[inline(always)]
    pub fn ch1cmp0en(&mut self) -> Ch1cmp0enW<'_, IrqenSpec> {
        Ch1cmp0enW::new(self, 6)
    }
    ///Bit 7 - 1- generate interrupt request when ch1cmp1f flag is set
    #[inline(always)]
    pub fn ch1cmp1en(&mut self) -> Ch1cmp1enW<'_, IrqenSpec> {
        Ch1cmp1enW::new(self, 7)
    }
    ///Bit 8 - 1- generate interrupt request when ch2rldf flag is set
    #[inline(always)]
    pub fn ch2rlden(&mut self) -> Ch2rldenW<'_, IrqenSpec> {
        Ch2rldenW::new(self, 8)
    }
    ///Bit 9 - 1- generate interrupt request when ch2capf flag is set
    #[inline(always)]
    pub fn ch2capen(&mut self) -> Ch2capenW<'_, IrqenSpec> {
        Ch2capenW::new(self, 9)
    }
    ///Bit 10 - 1- generate interrupt request when ch2cmp0f flag is set
    #[inline(always)]
    pub fn ch2cmp0en(&mut self) -> Ch2cmp0enW<'_, IrqenSpec> {
        Ch2cmp0enW::new(self, 10)
    }
    ///Bit 11 - 1- generate interrupt request when ch2cmp1f flag is set
    #[inline(always)]
    pub fn ch2cmp1en(&mut self) -> Ch2cmp1enW<'_, IrqenSpec> {
        Ch2cmp1enW::new(self, 11)
    }
    ///Bit 12 - 1- generate interrupt request when ch3rldf flag is set
    #[inline(always)]
    pub fn ch3rlden(&mut self) -> Ch3rldenW<'_, IrqenSpec> {
        Ch3rldenW::new(self, 12)
    }
    ///Bit 13 - 1- generate interrupt request when ch3capf flag is set
    #[inline(always)]
    pub fn ch3capen(&mut self) -> Ch3capenW<'_, IrqenSpec> {
        Ch3capenW::new(self, 13)
    }
    ///Bit 14 - 1- generate interrupt request when ch3cmp0f flag is set
    #[inline(always)]
    pub fn ch3cmp0en(&mut self) -> Ch3cmp0enW<'_, IrqenSpec> {
        Ch3cmp0enW::new(self, 14)
    }
    ///Bit 15 - 1- generate interrupt request when ch3cmp1f flag is set
    #[inline(always)]
    pub fn ch3cmp1en(&mut self) -> Ch3cmp1enW<'_, IrqenSpec> {
        Ch3cmp1enW::new(self, 15)
    }
}
/**Interrupt request enable register

You can [`read`](crate::Reg::read) this register and get [`irqen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IrqenSpec;
impl crate::RegisterSpec for IrqenSpec {
    type Ux = u32;
}
///`read()` method returns [`irqen::R`](R) reader structure
impl crate::Readable for IrqenSpec {}
///`write(|w| ..)` method takes [`irqen::W`](W) writer structure
impl crate::Writable for IrqenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQEN to value 0
impl crate::Resettable for IrqenSpec {}
