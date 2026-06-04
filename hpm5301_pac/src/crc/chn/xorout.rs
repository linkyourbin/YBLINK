///Register `xorout` reader
pub type R = crate::R<XoroutSpec>;
///Register `xorout` writer
pub type W = crate::W<XoroutSpec>;
///Field `XOROUT` reader - XOR for CRC result
pub type XoroutR = crate::FieldReader<u32>;
///Field `XOROUT` writer - XOR for CRC result
pub type XoroutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - XOR for CRC result
    #[inline(always)]
    pub fn xorout(&self) -> XoroutR {
        XoroutR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - XOR for CRC result
    #[inline(always)]
    pub fn xorout(&mut self) -> XoroutW<'_, XoroutSpec> {
        XoroutW::new(self, 0)
    }
}
/**chn&index0 xorout

You can [`read`](crate::Reg::read) this register and get [`xorout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xorout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XoroutSpec;
impl crate::RegisterSpec for XoroutSpec {
    type Ux = u32;
}
///`read()` method returns [`xorout::R`](R) reader structure
impl crate::Readable for XoroutSpec {}
///`write(|w| ..)` method takes [`xorout::W`](W) writer structure
impl crate::Writable for XoroutSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets xorout to value 0
impl crate::Resettable for XoroutSpec {}
