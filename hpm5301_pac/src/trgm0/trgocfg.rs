///Register `TRGOCFG[%s]` reader
pub type R = crate::R<TrgocfgSpec>;
///Register `TRGOCFG[%s]` writer
pub type W = crate::W<TrgocfgSpec>;
///Field `TRIGOSEL` reader - This bitfield selects one of the TRGM inputs as output.
pub type TrigoselR = crate::FieldReader;
///Field `TRIGOSEL` writer - This bitfield selects one of the TRGM inputs as output.
pub type TrigoselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `REDG2PEN` reader - 1- The selected input signal rising edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
pub type Redg2penR = crate::BitReader;
///Field `REDG2PEN` writer - 1- The selected input signal rising edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
pub type Redg2penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEDG2PEN` reader - 1- The selected input signal falling edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
pub type Fedg2penR = crate::BitReader;
///Field `FEDG2PEN` writer - 1- The selected input signal falling edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
pub type Fedg2penW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTINV` reader - 1- Invert the output
pub type OutinvR = crate::BitReader;
///Field `OUTINV` writer - 1- Invert the output
pub type OutinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - This bitfield selects one of the TRGM inputs as output.
    #[inline(always)]
    pub fn trigosel(&self) -> TrigoselR {
        TrigoselR::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - 1- The selected input signal rising edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
    #[inline(always)]
    pub fn redg2pen(&self) -> Redg2penR {
        Redg2penR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 1- The selected input signal falling edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
    #[inline(always)]
    pub fn fedg2pen(&self) -> Fedg2penR {
        Fedg2penR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 1- Invert the output
    #[inline(always)]
    pub fn outinv(&self) -> OutinvR {
        OutinvR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - This bitfield selects one of the TRGM inputs as output.
    #[inline(always)]
    pub fn trigosel(&mut self) -> TrigoselW<'_, TrgocfgSpec> {
        TrigoselW::new(self, 0)
    }
    ///Bit 9 - 1- The selected input signal rising edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
    #[inline(always)]
    pub fn redg2pen(&mut self) -> Redg2penW<'_, TrgocfgSpec> {
        Redg2penW::new(self, 9)
    }
    ///Bit 10 - 1- The selected input signal falling edge will be convert to an pulse on output. The output pulse can be stably used within the motor control system. When connecting the signal outside the motor system, due to the asynchronization of the clock systems, the clock frequency and signal active length need to be considered.
    #[inline(always)]
    pub fn fedg2pen(&mut self) -> Fedg2penW<'_, TrgocfgSpec> {
        Fedg2penW::new(self, 10)
    }
    ///Bit 11 - 1- Invert the output
    #[inline(always)]
    pub fn outinv(&mut self) -> OutinvW<'_, TrgocfgSpec> {
        OutinvW::new(self, 11)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`trgocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrgocfgSpec;
impl crate::RegisterSpec for TrgocfgSpec {
    type Ux = u32;
}
///`read()` method returns [`trgocfg::R`](R) reader structure
impl crate::Readable for TrgocfgSpec {}
///`write(|w| ..)` method takes [`trgocfg::W`](W) writer structure
impl crate::Writable for TrgocfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRGOCFG[%s] to value 0
impl crate::Resettable for TrgocfgSpec {}
