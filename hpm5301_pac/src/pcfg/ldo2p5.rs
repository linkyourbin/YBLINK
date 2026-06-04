///Register `LDO2P5` reader
pub type R = crate::R<Ldo2p5Spec>;
///Register `LDO2P5` writer
pub type W = crate::W<Ldo2p5Spec>;
///Field `VOLT` reader - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV
pub type VoltR = crate::FieldReader<u16>;
///Field `VOLT` writer - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV
pub type VoltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `ENABLE` reader - LDO enable 0: turn off LDO 1: turn on LDO
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - LDO enable 0: turn off LDO 1: turn on LDO
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READY` reader - Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready
pub type ReadyR = crate::BitReader;
impl R {
    ///Bits 0:11 - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV
    #[inline(always)]
    pub fn volt(&self) -> VoltR {
        VoltR::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 28 - Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV
    #[inline(always)]
    pub fn volt(&mut self) -> VoltW<'_, Ldo2p5Spec> {
        VoltW::new(self, 0)
    }
    ///Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Ldo2p5Spec> {
        EnableW::new(self, 16)
    }
}
/**2.5V LDO config

You can [`read`](crate::Reg::read) this register and get [`ldo2p5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo2p5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ldo2p5Spec;
impl crate::RegisterSpec for Ldo2p5Spec {
    type Ux = u32;
}
///`read()` method returns [`ldo2p5::R`](R) reader structure
impl crate::Readable for Ldo2p5Spec {}
///`write(|w| ..)` method takes [`ldo2p5::W`](W) writer structure
impl crate::Writable for Ldo2p5Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LDO2P5 to value 0x09c4
impl crate::Resettable for Ldo2p5Spec {
    const RESET_VALUE: u32 = 0x09c4;
}
