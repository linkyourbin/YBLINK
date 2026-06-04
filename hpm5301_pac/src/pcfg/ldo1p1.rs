///Register `LDO1P1` reader
pub type R = crate::R<Ldo1p1Spec>;
///Register `LDO1P1` writer
pub type W = crate::W<Ldo1p1Spec>;
///Field `VOLT` reader - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV
pub type VoltR = crate::FieldReader<u16>;
///Field `VOLT` writer - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV
pub type VoltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV
    #[inline(always)]
    pub fn volt(&self) -> VoltR {
        VoltR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV
    #[inline(always)]
    pub fn volt(&mut self) -> VoltW<'_, Ldo1p1Spec> {
        VoltW::new(self, 0)
    }
}
/**1V LDO config

You can [`read`](crate::Reg::read) this register and get [`ldo1p1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo1p1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ldo1p1Spec;
impl crate::RegisterSpec for Ldo1p1Spec {
    type Ux = u32;
}
///`read()` method returns [`ldo1p1::R`](R) reader structure
impl crate::Readable for Ldo1p1Spec {}
///`write(|w| ..)` method takes [`ldo1p1::W`](W) writer structure
impl crate::Writable for Ldo1p1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LDO1P1 to value 0x044c
impl crate::Resettable for Ldo1p1Spec {
    const RESET_VALUE: u32 = 0x044c;
}
