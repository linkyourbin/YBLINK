///Register `DLL` reader
pub type R = crate::R<Union20DllSpec>;
///Register `DLL` writer
pub type W = crate::W<Union20DllSpec>;
///Field `DLL` reader - Least significant byte of the Divisor Latch
pub type DllR = crate::FieldReader;
///Field `DLL` writer - Least significant byte of the Divisor Latch
pub type DllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Least significant byte of the Divisor Latch
    #[inline(always)]
    pub fn dll(&self) -> DllR {
        DllR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Least significant byte of the Divisor Latch
    #[inline(always)]
    pub fn dll(&mut self) -> DllW<'_, Union20DllSpec> {
        DllW::new(self, 0)
    }
}
/**Divisor Latch LSB (when DLAB = 1)

You can [`read`](crate::Reg::read) this register and get [`union_20_dll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_20_dll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union20DllSpec;
impl crate::RegisterSpec for Union20DllSpec {
    type Ux = u32;
}
///`read()` method returns [`union_20_dll::R`](R) reader structure
impl crate::Readable for Union20DllSpec {}
///`write(|w| ..)` method takes [`union_20_dll::W`](W) writer structure
impl crate::Writable for Union20DllSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLL to value 0x01
impl crate::Resettable for Union20DllSpec {
    const RESET_VALUE: u32 = 0x01;
}
