///Register `VALUE` reader
pub type R = crate::R<ValueSpec>;
///Register `VALUE` writer
pub type W = crate::W<ValueSpec>;
///Field `LINK` reader - retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: xtal_hold is kept on while cpu0 stop bit05: pll0_hold is kept on while cpu0 stop bit06: pll1_hold is kept on while cpu0 stop
pub type LinkR = crate::FieldReader<u16>;
///Field `LINK` writer - retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: xtal_hold is kept on while cpu0 stop bit05: pll0_hold is kept on while cpu0 stop bit06: pll1_hold is kept on while cpu0 stop
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: xtal_hold is kept on while cpu0 stop bit05: pll0_hold is kept on while cpu0 stop bit06: pll1_hold is kept on while cpu0 stop
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: xtal_hold is kept on while cpu0 stop bit05: pll0_hold is kept on while cpu0 stop bit06: pll1_hold is kept on while cpu0 stop
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, ValueSpec> {
        LinkW::new(self, 0)
    }
}
/**Retention Control

You can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
///`read()` method returns [`value::R`](R) reader structure
impl crate::Readable for ValueSpec {}
///`write(|w| ..)` method takes [`value::W`](W) writer structure
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VALUE to value 0
impl crate::Resettable for ValueSpec {}
