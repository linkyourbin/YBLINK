///Register `LOAD_COMP` reader
pub type R = crate::R<LoadCompSpec>;
///Register `LOAD_COMP` writer
pub type W = crate::W<LoadCompSpec>;
///Field `COMPLETE` reader - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3
pub type CompleteR = crate::FieldReader;
///Field `COMPLETE` writer - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3
pub type CompleteW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3
    #[inline(always)]
    pub fn complete(&self) -> CompleteR {
        CompleteR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3
    #[inline(always)]
    pub fn complete(&mut self) -> CompleteW<'_, LoadCompSpec> {
        CompleteW::new(self, 0)
    }
}
/**LOAD complete

You can [`read`](crate::Reg::read) this register and get [`load_comp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_comp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LoadCompSpec;
impl crate::RegisterSpec for LoadCompSpec {
    type Ux = u32;
}
///`read()` method returns [`load_comp::R`](R) reader structure
impl crate::Readable for LoadCompSpec {}
///`write(|w| ..)` method takes [`load_comp::W`](W) writer structure
impl crate::Writable for LoadCompSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOAD_COMP to value 0x07
impl crate::Resettable for LoadCompSpec {
    const RESET_VALUE: u32 = 0x07;
}
