///Register `PRIORITY[%s]` reader
pub type R = crate::R<PrioritySpec>;
///Register `PRIORITY[%s]` writer
pub type W = crate::W<PrioritySpec>;
///Field `PRIORITY` reader - Interrupt source priority. The valid range of this field is 0-31. 0: Never interrupt 1-31: Interrupt source priority. The larger the value, the higher the priority.
pub type PriorityR = crate::FieldReader<u32>;
///Field `PRIORITY` writer - Interrupt source priority. The valid range of this field is 0-31. 0: Never interrupt 1-31: Interrupt source priority. The larger the value, the higher the priority.
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Interrupt source priority. The valid range of this field is 0-31. 0: Never interrupt 1-31: Interrupt source priority. The larger the value, the higher the priority.
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Interrupt source priority. The valid range of this field is 0-31. 0: Never interrupt 1-31: Interrupt source priority. The larger the value, the higher the priority.
    #[inline(always)]
    pub fn priority(&mut self) -> PriorityW<'_, PrioritySpec> {
        PriorityW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PrioritySpec;
impl crate::RegisterSpec for PrioritySpec {
    type Ux = u32;
}
///`read()` method returns [`priority::R`](R) reader structure
impl crate::Readable for PrioritySpec {}
///`write(|w| ..)` method takes [`priority::W`](W) writer structure
impl crate::Writable for PrioritySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIORITY[%s] to value 0x01
impl crate::Resettable for PrioritySpec {
    const RESET_VALUE: u32 = 0x01;
}
