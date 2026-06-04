///Register `trg_sw_sta` reader
pub type R = crate::R<TrgSwStaSpec>;
///Register `trg_sw_sta` writer
pub type W = crate::W<TrgSwStaSpec>;
///Field `TRIG_SW_INDEX` reader - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c
pub type TrigSwIndexR = crate::FieldReader;
///Field `TRIG_SW_INDEX` writer - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c
pub type TrigSwIndexW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRG_SW_STA` reader - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it.
pub type TrgSwStaR = crate::BitReader;
///Field `TRG_SW_STA` writer - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it.
pub type TrgSwStaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c
    #[inline(always)]
    pub fn trig_sw_index(&self) -> TrigSwIndexR {
        TrigSwIndexR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it.
    #[inline(always)]
    pub fn trg_sw_sta(&self) -> TrgSwStaR {
        TrgSwStaR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c
    #[inline(always)]
    pub fn trig_sw_index(&mut self) -> TrigSwIndexW<'_, TrgSwStaSpec> {
        TrigSwIndexW::new(self, 0)
    }
    ///Bit 4 - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it.
    #[inline(always)]
    pub fn trg_sw_sta(&mut self) -> TrgSwStaW<'_, TrgSwStaSpec> {
        TrgSwStaW::new(self, 4)
    }
}
/**No description available

You can [`read`](crate::Reg::read) this register and get [`trg_sw_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg_sw_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrgSwStaSpec;
impl crate::RegisterSpec for TrgSwStaSpec {
    type Ux = u32;
}
///`read()` method returns [`trg_sw_sta::R`](R) reader structure
impl crate::Readable for TrgSwStaSpec {}
///`write(|w| ..)` method takes [`trg_sw_sta::W`](W) writer structure
impl crate::Writable for TrgSwStaSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets trg_sw_sta to value 0
impl crate::Resettable for TrgSwStaSpec {}
