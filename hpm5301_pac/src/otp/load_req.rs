///Register `LOAD_REQ` reader
pub type R = crate::R<LoadReqSpec>;
///Register `LOAD_REQ` writer
pub type W = crate::W<LoadReqSpec>;
///Field `REQUEST` reader - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3
pub type RequestR = crate::FieldReader;
///Field `REQUEST` writer - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3
pub type RequestW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3
    #[inline(always)]
    pub fn request(&self) -> RequestR {
        RequestR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3
    #[inline(always)]
    pub fn request(&mut self) -> RequestW<'_, LoadReqSpec> {
        RequestW::new(self, 0)
    }
}
/**LOAD Request

You can [`read`](crate::Reg::read) this register and get [`load_req::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_req::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LoadReqSpec;
impl crate::RegisterSpec for LoadReqSpec {
    type Ux = u32;
}
///`read()` method returns [`load_req::R`](R) reader structure
impl crate::Readable for LoadReqSpec {}
///`write(|w| ..)` method takes [`load_req::W`](W) writer structure
impl crate::Writable for LoadReqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOAD_REQ to value 0x07
impl crate::Resettable for LoadReqSpec {
    const RESET_VALUE: u32 = 0x07;
}
