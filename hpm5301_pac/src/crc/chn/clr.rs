///Register `clr` reader
pub type R = crate::R<ClrSpec>;
///Register `clr` writer
pub type W = crate::W<ClrSpec>;
///Field `CLR` reader - write 1 to clr crc setting and result for its channel. always read 0.
pub type ClrR = crate::BitReader;
///Field `CLR` writer - write 1 to clr crc setting and result for its channel. always read 0.
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - write 1 to clr crc setting and result for its channel. always read 0.
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - write 1 to clr crc setting and result for its channel. always read 0.
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, ClrSpec> {
        ClrW::new(self, 0)
    }
}
/**chn&index0 clear crc result and setting

You can [`read`](crate::Reg::read) this register and get [`clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
///`read()` method returns [`clr::R`](R) reader structure
impl crate::Readable for ClrSpec {}
///`write(|w| ..)` method takes [`clr::W`](W) writer structure
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets clr to value 0
impl crate::Resettable for ClrSpec {}
