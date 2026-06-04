///Register `Data` reader
pub type R = crate::R<DataSpec>;
///Register `Data` writer
pub type W = crate::W<DataSpec>;
///Field `DATA` reader - Write this register to put one byte of data to the FIFO. Read this register to get one byte of data from the FIFO.
pub type DataR = crate::FieldReader;
///Field `DATA` writer - Write this register to put one byte of data to the FIFO. Read this register to get one byte of data from the FIFO.
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Write this register to put one byte of data to the FIFO. Read this register to get one byte of data from the FIFO.
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Write this register to put one byte of data to the FIFO. Read this register to get one byte of data from the FIFO.
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, DataSpec> {
        DataW::new(self, 0)
    }
}
/**Data Register

You can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
///`read()` method returns [`data::R`](R) reader structure
impl crate::Readable for DataSpec {}
///`write(|w| ..)` method takes [`data::W`](W) writer structure
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Data to value 0
impl crate::Resettable for DataSpec {}
