///Register `RBR` reader
pub type R = crate::R<Union20RbrSpec>;
///Register `RBR` writer
pub type W = crate::W<Union20RbrSpec>;
///Field `RBR` reader - Receive data read port
pub type RbrR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Receive data read port
    #[inline(always)]
    pub fn rbr(&self) -> RbrR {
        RbrR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
/**Receiver Buffer Register (when DLAB = 0)

You can [`read`](crate::Reg::read) this register and get [`union_20_rbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_20_rbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union20RbrSpec;
impl crate::RegisterSpec for Union20RbrSpec {
    type Ux = u32;
}
///`read()` method returns [`union_20_rbr::R`](R) reader structure
impl crate::Readable for Union20RbrSpec {}
///`write(|w| ..)` method takes [`union_20_rbr::W`](W) writer structure
impl crate::Writable for Union20RbrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RBR to value 0
impl crate::Resettable for Union20RbrSpec {}
