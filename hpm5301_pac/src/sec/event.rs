///Register `EVENT` reader
pub type R = crate::R<EventSpec>;
///Register `EVENT` writer
pub type W = crate::W<EventSpec>;
///Field `PMIC_ESC_SEC` reader - PMIC is escalting secure event
pub type PmicEscSecR = crate::BitReader;
///Field `PMIC_ESC_NSC` reader - PMIC is escalating non-secure event
pub type PmicEscNscR = crate::BitReader;
///Field `EVENT` reader - local event statue, each bit represents one security event
pub type EventR = crate::FieldReader<u16>;
impl R {
    ///Bit 2 - PMIC is escalting secure event
    #[inline(always)]
    pub fn pmic_esc_sec(&self) -> PmicEscSecR {
        PmicEscSecR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PMIC is escalating non-secure event
    #[inline(always)]
    pub fn pmic_esc_nsc(&self) -> PmicEscNscR {
        PmicEscNscR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 16:31 - local event statue, each bit represents one security event
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
/**Event and escalate status

You can [`read`](crate::Reg::read) this register and get [`event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EventSpec;
impl crate::RegisterSpec for EventSpec {
    type Ux = u32;
}
///`read()` method returns [`event::R`](R) reader structure
impl crate::Readable for EventSpec {}
///`write(|w| ..)` method takes [`event::W`](W) writer structure
impl crate::Writable for EventSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EVENT to value 0
impl crate::Resettable for EventSpec {}
