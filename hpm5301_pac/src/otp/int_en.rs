///Register `INT_EN` reader
pub type R = crate::R<IntEnSpec>;
///Register `INT_EN` writer
pub type W = crate::W<IntEnSpec>;
///Field `LOAD` reader - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable
pub type LoadR = crate::BitReader;
///Field `LOAD` writer - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READ` reader - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable
pub type ReadR = crate::BitReader;
///Field `READ` writer - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITE` reader - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable
pub type WriteR = crate::BitReader;
///Field `WRITE` writer - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, IntEnSpec> {
        LoadW::new(self, 0)
    }
    ///Bit 1 - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable
    #[inline(always)]
    pub fn read(&mut self) -> ReadW<'_, IntEnSpec> {
        ReadW::new(self, 1)
    }
    ///Bit 2 - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IntEnSpec> {
        WriteW::new(self, 2)
    }
}
/**interrupt enable

You can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u32;
}
///`read()` method returns [`int_en::R`](R) reader structure
impl crate::Readable for IntEnSpec {}
///`write(|w| ..)` method takes [`int_en::W`](W) writer structure
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INT_EN to value 0
impl crate::Resettable for IntEnSpec {}
