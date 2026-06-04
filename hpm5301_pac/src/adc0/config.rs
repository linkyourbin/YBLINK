///Register `CONFIG[%s]` reader
pub type R = crate::R<ConfigSpec>;
///Register `CONFIG[%s]` writer
pub type W = crate::W<ConfigSpec>;
///Field `CHAN0` reader - channel number for 1st conversion
pub type Chan0R = crate::FieldReader;
///Field `CHAN0` writer - channel number for 1st conversion
pub type Chan0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `INTEN0` reader - interrupt enable for 1st conversion
pub type Inten0R = crate::BitReader;
///Field `INTEN0` writer - interrupt enable for 1st conversion
pub type Inten0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QUEUE_EN` reader - preemption queue enable control
pub type QueueEnR = crate::BitReader;
///Field `QUEUE_EN` writer - preemption queue enable control
pub type QueueEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHAN1` reader - channel number for 2nd conversion
pub type Chan1R = crate::FieldReader;
///Field `CHAN1` writer - channel number for 2nd conversion
pub type Chan1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `INTEN1` reader - interrupt enable for 2nd conversion
pub type Inten1R = crate::BitReader;
///Field `INTEN1` writer - interrupt enable for 2nd conversion
pub type Inten1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHAN2` reader - channel number for 3rd conversion
pub type Chan2R = crate::FieldReader;
///Field `CHAN2` writer - channel number for 3rd conversion
pub type Chan2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `INTEN2` reader - interrupt enable for 3rd conversion
pub type Inten2R = crate::BitReader;
///Field `INTEN2` writer - interrupt enable for 3rd conversion
pub type Inten2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHAN3` reader - channel number for 4th conversion
pub type Chan3R = crate::FieldReader;
///Field `CHAN3` writer - channel number for 4th conversion
pub type Chan3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `INTEN3` reader - interrupt enable for 4th conversion
pub type Inten3R = crate::BitReader;
///Field `INTEN3` writer - interrupt enable for 4th conversion
pub type Inten3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIG_LEN` writer - length for current trigger, can up to 4 conversions for one trigger, from 0 to 3
pub type TrigLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:4 - channel number for 1st conversion
    #[inline(always)]
    pub fn chan0(&self) -> Chan0R {
        Chan0R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - interrupt enable for 1st conversion
    #[inline(always)]
    pub fn inten0(&self) -> Inten0R {
        Inten0R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - preemption queue enable control
    #[inline(always)]
    pub fn queue_en(&self) -> QueueEnR {
        QueueEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:12 - channel number for 2nd conversion
    #[inline(always)]
    pub fn chan1(&self) -> Chan1R {
        Chan1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 13 - interrupt enable for 2nd conversion
    #[inline(always)]
    pub fn inten1(&self) -> Inten1R {
        Inten1R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:20 - channel number for 3rd conversion
    #[inline(always)]
    pub fn chan2(&self) -> Chan2R {
        Chan2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 21 - interrupt enable for 3rd conversion
    #[inline(always)]
    pub fn inten2(&self) -> Inten2R {
        Inten2R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:28 - channel number for 4th conversion
    #[inline(always)]
    pub fn chan3(&self) -> Chan3R {
        Chan3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 29 - interrupt enable for 4th conversion
    #[inline(always)]
    pub fn inten3(&self) -> Inten3R {
        Inten3R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - channel number for 1st conversion
    #[inline(always)]
    pub fn chan0(&mut self) -> Chan0W<'_, ConfigSpec> {
        Chan0W::new(self, 0)
    }
    ///Bit 5 - interrupt enable for 1st conversion
    #[inline(always)]
    pub fn inten0(&mut self) -> Inten0W<'_, ConfigSpec> {
        Inten0W::new(self, 5)
    }
    ///Bit 6 - preemption queue enable control
    #[inline(always)]
    pub fn queue_en(&mut self) -> QueueEnW<'_, ConfigSpec> {
        QueueEnW::new(self, 6)
    }
    ///Bits 8:12 - channel number for 2nd conversion
    #[inline(always)]
    pub fn chan1(&mut self) -> Chan1W<'_, ConfigSpec> {
        Chan1W::new(self, 8)
    }
    ///Bit 13 - interrupt enable for 2nd conversion
    #[inline(always)]
    pub fn inten1(&mut self) -> Inten1W<'_, ConfigSpec> {
        Inten1W::new(self, 13)
    }
    ///Bits 16:20 - channel number for 3rd conversion
    #[inline(always)]
    pub fn chan2(&mut self) -> Chan2W<'_, ConfigSpec> {
        Chan2W::new(self, 16)
    }
    ///Bit 21 - interrupt enable for 3rd conversion
    #[inline(always)]
    pub fn inten2(&mut self) -> Inten2W<'_, ConfigSpec> {
        Inten2W::new(self, 21)
    }
    ///Bits 24:28 - channel number for 4th conversion
    #[inline(always)]
    pub fn chan3(&mut self) -> Chan3W<'_, ConfigSpec> {
        Chan3W::new(self, 24)
    }
    ///Bit 29 - interrupt enable for 4th conversion
    #[inline(always)]
    pub fn inten3(&mut self) -> Inten3W<'_, ConfigSpec> {
        Inten3W::new(self, 29)
    }
    ///Bits 30:31 - length for current trigger, can up to 4 conversions for one trigger, from 0 to 3
    #[inline(always)]
    pub fn trig_len(&mut self) -> TrigLenW<'_, ConfigSpec> {
        TrigLenW::new(self, 30)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
///`read()` method returns [`config::R`](R) reader structure
impl crate::Readable for ConfigSpec {}
///`write(|w| ..)` method takes [`config::W`](W) writer structure
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFIG[%s] to value 0
impl crate::Resettable for ConfigSpec {}
