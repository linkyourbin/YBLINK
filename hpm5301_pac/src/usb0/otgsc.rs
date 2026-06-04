///Register `OTGSC` reader
pub type R = crate::R<OtgscSpec>;
///Register `OTGSC` writer
pub type W = crate::W<OtgscSpec>;
///Field `VD` reader - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor.
pub type VdR = crate::BitReader;
///Field `VD` writer - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor.
pub type VdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC` reader - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP.
pub type VcR = crate::BitReader;
///Field `VC` writer - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP.
pub type VcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDPU` reader - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \[default\]. When this bit is 0, the ID input will not be sampled.
pub type IdpuR = crate::BitReader;
///Field `IDPU` writer - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \[default\]. When this bit is 0, the ID input will not be sampled.
pub type IdpuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ID` reader - ID USB ID - Read Only. 0 = A device, 1 = B device
pub type IdR = crate::BitReader;
///Field `AVV` reader - AVV A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold.
pub type AvvR = crate::BitReader;
///Field `ASV` reader - ASV A Session Valid - Read Only. Indicates VBus is above the A session valid threshold.
pub type AsvR = crate::BitReader;
///Field `IDIS` reader - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit.
pub type IdisR = crate::BitReader;
///Field `IDIS` writer - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit.
pub type IdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVVIS` reader - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit.
pub type AvvisR = crate::BitReader;
///Field `AVVIS` writer - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit.
pub type AvvisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASVIS` reader - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit.
pub type AsvisR = crate::BitReader;
///Field `ASVIS` writer - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit.
pub type AsvisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDIE` reader - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt.
pub type IdieR = crate::BitReader;
///Field `IDIE` writer - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt.
pub type IdieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVVIE` reader - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt.
pub type AvvieR = crate::BitReader;
///Field `AVVIE` writer - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt.
pub type AvvieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASVIE` reader - ASVIE A Session Valid Interrupt Enable - Read/Write.
pub type AsvieR = crate::BitReader;
///Field `ASVIE` writer - ASVIE A Session Valid Interrupt Enable - Read/Write.
pub type AsvieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor.
    #[inline(always)]
    pub fn vd(&self) -> VdR {
        VdR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP.
    #[inline(always)]
    pub fn vc(&self) -> VcR {
        VcR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \[default\]. When this bit is 0, the ID input will not be sampled.
    #[inline(always)]
    pub fn idpu(&self) -> IdpuR {
        IdpuR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - ID USB ID - Read Only. 0 = A device, 1 = B device
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AVV A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold.
    #[inline(always)]
    pub fn avv(&self) -> AvvR {
        AvvR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ASV A Session Valid - Read Only. Indicates VBus is above the A session valid threshold.
    #[inline(always)]
    pub fn asv(&self) -> AsvR {
        AsvR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit.
    #[inline(always)]
    pub fn idis(&self) -> IdisR {
        IdisR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit.
    #[inline(always)]
    pub fn avvis(&self) -> AvvisR {
        AvvisR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit.
    #[inline(always)]
    pub fn asvis(&self) -> AsvisR {
        AsvisR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt.
    #[inline(always)]
    pub fn idie(&self) -> IdieR {
        IdieR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt.
    #[inline(always)]
    pub fn avvie(&self) -> AvvieR {
        AvvieR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ASVIE A Session Valid Interrupt Enable - Read/Write.
    #[inline(always)]
    pub fn asvie(&self) -> AsvieR {
        AsvieR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor.
    #[inline(always)]
    pub fn vd(&mut self) -> VdW<'_, OtgscSpec> {
        VdW::new(self, 0)
    }
    ///Bit 1 - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP.
    #[inline(always)]
    pub fn vc(&mut self) -> VcW<'_, OtgscSpec> {
        VcW::new(self, 1)
    }
    ///Bit 5 - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \[default\]. When this bit is 0, the ID input will not be sampled.
    #[inline(always)]
    pub fn idpu(&mut self) -> IdpuW<'_, OtgscSpec> {
        IdpuW::new(self, 5)
    }
    ///Bit 16 - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit.
    #[inline(always)]
    pub fn idis(&mut self) -> IdisW<'_, OtgscSpec> {
        IdisW::new(self, 16)
    }
    ///Bit 17 - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit.
    #[inline(always)]
    pub fn avvis(&mut self) -> AvvisW<'_, OtgscSpec> {
        AvvisW::new(self, 17)
    }
    ///Bit 18 - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit.
    #[inline(always)]
    pub fn asvis(&mut self) -> AsvisW<'_, OtgscSpec> {
        AsvisW::new(self, 18)
    }
    ///Bit 24 - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt.
    #[inline(always)]
    pub fn idie(&mut self) -> IdieW<'_, OtgscSpec> {
        IdieW::new(self, 24)
    }
    ///Bit 25 - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt.
    #[inline(always)]
    pub fn avvie(&mut self) -> AvvieW<'_, OtgscSpec> {
        AvvieW::new(self, 25)
    }
    ///Bit 26 - ASVIE A Session Valid Interrupt Enable - Read/Write.
    #[inline(always)]
    pub fn asvie(&mut self) -> AsvieW<'_, OtgscSpec> {
        AsvieW::new(self, 26)
    }
}
/**On-The-Go Status & control Register

You can [`read`](crate::Reg::read) this register and get [`otgsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OtgscSpec;
impl crate::RegisterSpec for OtgscSpec {
    type Ux = u32;
}
///`read()` method returns [`otgsc::R`](R) reader structure
impl crate::Readable for OtgscSpec {}
///`write(|w| ..)` method takes [`otgsc::W`](W) writer structure
impl crate::Writable for OtgscSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTGSC to value 0
impl crate::Resettable for OtgscSpec {}
