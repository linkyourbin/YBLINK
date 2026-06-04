///Register `INT_FLAG` reader
pub type R = crate::R<IntFlagSpec>;
///Register `INT_FLAG` writer
pub type W = crate::W<IntFlagSpec>;
///Field `LOAD` reader - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded
pub type LoadR = crate::BitReader;
///Field `LOAD` writer - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READ` reader - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register
pub type ReadR = crate::BitReader;
///Field `READ` writer - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITE` reader - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse
pub type WriteR = crate::BitReader;
///Field `WRITE` writer - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, IntFlagSpec> {
        LoadW::new(self, 0)
    }
    ///Bit 1 - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register
    #[inline(always)]
    pub fn read(&mut self) -> ReadW<'_, IntFlagSpec> {
        ReadW::new(self, 1)
    }
    ///Bit 2 - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IntFlagSpec> {
        WriteW::new(self, 2)
    }
}
/**interrupt flag

You can [`read`](crate::Reg::read) this register and get [`int_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntFlagSpec;
impl crate::RegisterSpec for IntFlagSpec {
    type Ux = u32;
}
///`read()` method returns [`int_flag::R`](R) reader structure
impl crate::Readable for IntFlagSpec {}
///`write(|w| ..)` method takes [`int_flag::W`](W) writer structure
impl crate::Writable for IntFlagSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INT_FLAG to value 0
impl crate::Resettable for IntFlagSpec {}
