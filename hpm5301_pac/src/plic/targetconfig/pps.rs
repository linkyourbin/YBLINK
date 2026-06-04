///Register `PPS` reader
pub type R = crate::R<PpsSpec>;
///Register `PPS` writer
pub type W = crate::W<PpsSpec>;
///Field `PRIORITY_PREEMPTED` reader - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt.
pub type PriorityPreemptedR = crate::FieldReader<u32>;
///Field `PRIORITY_PREEMPTED` writer - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt.
pub type PriorityPreemptedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt.
    #[inline(always)]
    pub fn priority_preempted(&self) -> PriorityPreemptedR {
        PriorityPreemptedR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt.
    #[inline(always)]
    pub fn priority_preempted(&mut self) -> PriorityPreemptedW<'_, PpsSpec> {
        PriorityPreemptedW::new(self, 0)
    }
}
/**Preempted priority stack

You can [`read`](crate::Reg::read) this register and get [`pps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PpsSpec;
impl crate::RegisterSpec for PpsSpec {
    type Ux = u32;
}
///`read()` method returns [`pps::R`](R) reader structure
impl crate::Readable for PpsSpec {}
///`write(|w| ..)` method takes [`pps::W`](W) writer structure
impl crate::Writable for PpsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PPS to value 0
impl crate::Resettable for PpsSpec {}
