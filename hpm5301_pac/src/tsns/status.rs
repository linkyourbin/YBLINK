///Register `STATUS` reader
pub type R = crate::R<StatusSpec>;
///Register `STATUS` writer
pub type W = crate::W<StatusSpec>;
///Field `TRIGGER` writer - Software trigger for sensing in trigger mode, trigger will be ignored if in sensing or other mode
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VALID` reader - indicate value in T is valid or not 0: not valid 1:valid
pub type ValidR = crate::BitReader;
impl R {
    ///Bit 31 - indicate value in T is valid or not 0: not valid 1:valid
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software trigger for sensing in trigger mode, trigger will be ignored if in sensing or other mode
    #[inline(always)]
    pub fn trigger(&mut self) -> TriggerW<'_, StatusSpec> {
        TriggerW::new(self, 0)
    }
}
/**Status

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for StatusSpec {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for StatusSpec {}
