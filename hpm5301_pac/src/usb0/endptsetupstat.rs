///Register `ENDPTSETUPSTAT` reader
pub type R = crate::R<EndptsetupstatSpec>;
///Register `ENDPTSETUPSTAT` writer
pub type W = crate::W<EndptsetupstatSpec>;
///Field `ENDPTSETUPSTAT` reader - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode.
pub type EndptsetupstatR = crate::FieldReader<u16>;
///Field `ENDPTSETUPSTAT` writer - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode.
pub type EndptsetupstatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode.
    #[inline(always)]
    pub fn endptsetupstat(&self) -> EndptsetupstatR {
        EndptsetupstatR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode.
    #[inline(always)]
    pub fn endptsetupstat(&mut self) -> EndptsetupstatW<'_, EndptsetupstatSpec> {
        EndptsetupstatW::new(self, 0)
    }
}
/**Endpoint Setup Status Register

You can [`read`](crate::Reg::read) this register and get [`endptsetupstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptsetupstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EndptsetupstatSpec;
impl crate::RegisterSpec for EndptsetupstatSpec {
    type Ux = u32;
}
///`read()` method returns [`endptsetupstat::R`](R) reader structure
impl crate::Readable for EndptsetupstatSpec {}
///`write(|w| ..)` method takes [`endptsetupstat::W`](W) writer structure
impl crate::Writable for EndptsetupstatSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTSETUPSTAT to value 0
impl crate::Resettable for EndptsetupstatSpec {}
