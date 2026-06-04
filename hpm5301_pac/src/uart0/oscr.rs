///Register `OSCR` reader
pub type R = crate::R<OscrSpec>;
///Register `OSCR` writer
pub type W = crate::W<OscrSpec>;
///Field `OSC` reader - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC
pub type OscR = crate::FieldReader;
///Field `OSC` writer - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC
pub type OscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC
    #[inline(always)]
    pub fn osc(&self) -> OscR {
        OscR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC
    #[inline(always)]
    pub fn osc(&mut self) -> OscW<'_, OscrSpec> {
        OscW::new(self, 0)
    }
}
/**Over Sample Control Register

You can [`read`](crate::Reg::read) this register and get [`oscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OscrSpec;
impl crate::RegisterSpec for OscrSpec {
    type Ux = u32;
}
///`read()` method returns [`oscr::R`](R) reader structure
impl crate::Readable for OscrSpec {}
///`write(|w| ..)` method takes [`oscr::W`](W) writer structure
impl crate::Writable for OscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSCR to value 0x10
impl crate::Resettable for OscrSpec {
    const RESET_VALUE: u32 = 0x10;
}
