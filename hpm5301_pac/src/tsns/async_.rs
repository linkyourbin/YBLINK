///Register `ASYNC` reader
pub type R = crate::R<AsyncSpec>;
///Register `ASYNC` writer
pub type W = crate::W<AsyncSpec>;
///Field `VALUE` reader - Value of async mode to compare
pub type ValueR = crate::FieldReader<u16>;
///Field `VALUE` writer - Value of async mode to compare
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `POLARITY` reader - Polarity of internal comparator
pub type PolarityR = crate::BitReader;
///Field `POLARITY` writer - Polarity of internal comparator
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNC_TYPE` reader - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than
pub type AsyncTypeR = crate::BitReader;
///Field `ASYNC_TYPE` writer - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than
pub type AsyncTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Value of async mode to compare
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 16 - Polarity of internal comparator
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than
    #[inline(always)]
    pub fn async_type(&self) -> AsyncTypeR {
        AsyncTypeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:10 - Value of async mode to compare
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, AsyncSpec> {
        ValueW::new(self, 0)
    }
    ///Bit 16 - Polarity of internal comparator
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, AsyncSpec> {
        PolarityW::new(self, 16)
    }
    ///Bit 24 - Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than
    #[inline(always)]
    pub fn async_type(&mut self) -> AsyncTypeW<'_, AsyncSpec> {
        AsyncTypeW::new(self, 24)
    }
}
/**Configuration in asynchronous mode

You can [`read`](crate::Reg::read) this register and get [`async_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AsyncSpec;
impl crate::RegisterSpec for AsyncSpec {
    type Ux = u32;
}
///`read()` method returns [`async_::R`](R) reader structure
impl crate::Readable for AsyncSpec {}
///`write(|w| ..)` method takes [`async_::W`](W) writer structure
impl crate::Writable for AsyncSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASYNC to value 0
impl crate::Resettable for AsyncSpec {}
