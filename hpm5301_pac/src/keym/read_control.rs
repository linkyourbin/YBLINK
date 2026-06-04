///Register `READ_CONTROL` reader
pub type R = crate::R<ReadControlSpec>;
///Register `READ_CONTROL` writer
pub type W = crate::W<ReadControlSpec>;
///Field `BLOCK_SMK_READ` reader - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
pub type BlockSmkReadR = crate::BitReader;
///Field `BLOCK_SMK_READ` writer - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
pub type BlockSmkReadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLOCK_PK_READ` reader - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
pub type BlockPkReadR = crate::BitReader;
///Field `BLOCK_PK_READ` writer - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
pub type BlockPkReadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
    #[inline(always)]
    pub fn block_smk_read(&self) -> BlockSmkReadR {
        BlockSmkReadR::new((self.bits & 1) != 0)
    }
    ///Bit 16 - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
    #[inline(always)]
    pub fn block_pk_read(&self) -> BlockPkReadR {
        BlockPkReadR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
    #[inline(always)]
    pub fn block_smk_read(&mut self) -> BlockSmkReadW<'_, ReadControlSpec> {
        BlockSmkReadW::new(self, 0)
    }
    ///Bit 16 - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out
    #[inline(always)]
    pub fn block_pk_read(&mut self) -> BlockPkReadW<'_, ReadControlSpec> {
        BlockPkReadW::new(self, 16)
    }
}
/**key read out control

You can [`read`](crate::Reg::read) this register and get [`read_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ReadControlSpec;
impl crate::RegisterSpec for ReadControlSpec {
    type Ux = u32;
}
///`read()` method returns [`read_control::R`](R) reader structure
impl crate::Readable for ReadControlSpec {}
///`write(|w| ..)` method takes [`read_control::W`](W) writer structure
impl crate::Writable for ReadControlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets READ_CONTROL to value 0
impl crate::Resettable for ReadControlSpec {}
