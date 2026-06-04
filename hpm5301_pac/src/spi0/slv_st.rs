///Register `SlvSt` reader
pub type R = crate::R<SlvStSpec>;
///Register `SlvSt` writer
pub type W = crate::W<SlvStSpec>;
///Field `USR_STATUS` reader - User defined status flags
pub type UsrStatusR = crate::FieldReader<u16>;
///Field `USR_STATUS` writer - User defined status flags
pub type UsrStatusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `READY` reader - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0.
pub type ReadyR = crate::BitReader;
///Field `READY` writer - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0.
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVERRUN` reader - Data overrun occurs in the last transaction
pub type OverrunR = crate::BitReader;
///Field `OVERRUN` writer - Data overrun occurs in the last transaction
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNDERRUN` writer - Data underrun occurs in the last transaction
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - User defined status flags
    #[inline(always)]
    pub fn usr_status(&self) -> UsrStatusR {
        UsrStatusR::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0.
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Data overrun occurs in the last transaction
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - User defined status flags
    #[inline(always)]
    pub fn usr_status(&mut self) -> UsrStatusW<'_, SlvStSpec> {
        UsrStatusW::new(self, 0)
    }
    ///Bit 16 - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0.
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, SlvStSpec> {
        ReadyW::new(self, 16)
    }
    ///Bit 17 - Data overrun occurs in the last transaction
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, SlvStSpec> {
        OverrunW::new(self, 17)
    }
    ///Bit 18 - Data underrun occurs in the last transaction
    #[inline(always)]
    pub fn underrun(&mut self) -> UnderrunW<'_, SlvStSpec> {
        UnderrunW::new(self, 18)
    }
}
/**Slave Status Register

You can [`read`](crate::Reg::read) this register and get [`slv_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SlvStSpec;
impl crate::RegisterSpec for SlvStSpec {
    type Ux = u32;
}
///`read()` method returns [`slv_st::R`](R) reader structure
impl crate::Readable for SlvStSpec {}
///`write(|w| ..)` method takes [`slv_st::W`](W) writer structure
impl crate::Writable for SlvStSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SlvSt to value 0
impl crate::Resettable for SlvStSpec {}
