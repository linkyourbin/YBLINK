///Register `init_data` reader
pub type R = crate::R<InitDataSpec>;
///Register `init_data` writer
pub type W = crate::W<InitDataSpec>;
///Field `INIT_DATA` reader - initial data of CRC
pub type InitDataR = crate::FieldReader<u32>;
///Field `INIT_DATA` writer - initial data of CRC
pub type InitDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - initial data of CRC
    #[inline(always)]
    pub fn init_data(&self) -> InitDataR {
        InitDataR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - initial data of CRC
    #[inline(always)]
    pub fn init_data(&mut self) -> InitDataW<'_, InitDataSpec> {
        InitDataW::new(self, 0)
    }
}
/**chn&index0 init_data

You can [`read`](crate::Reg::read) this register and get [`init_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct InitDataSpec;
impl crate::RegisterSpec for InitDataSpec {
    type Ux = u32;
}
///`read()` method returns [`init_data::R`](R) reader structure
impl crate::Readable for InitDataSpec {}
///`write(|w| ..)` method takes [`init_data::W`](W) writer structure
impl crate::Writable for InitDataSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets init_data to value 0
impl crate::Resettable for InitDataSpec {}
