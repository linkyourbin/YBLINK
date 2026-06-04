///Register `misc_setting` reader
pub type R = crate::R<MiscSettingSpec>;
///Register `misc_setting` writer
pub type W = crate::W<MiscSettingSpec>;
///Field `POLY_WIDTH` reader - crc data length
pub type PolyWidthR = crate::FieldReader;
///Field `POLY_WIDTH` writer - crc data length
pub type PolyWidthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `REV_IN` reader - 0: no wrap input bit order 1: wrap input bit order
pub type RevInR = crate::BitReader;
///Field `REV_IN` writer - 0: no wrap input bit order 1: wrap input bit order
pub type RevInW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REV_OUT` reader - 0: no wrap output bit order 1: wrap output bit order
pub type RevOutR = crate::BitReader;
///Field `REV_OUT` writer - 0: no wrap output bit order 1: wrap output bit order
pub type RevOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTE_REV` reader - 0: no wrap input byte order 1: wrap input byte order
pub type ByteRevR = crate::BitReader;
///Field `BYTE_REV` writer - 0: no wrap input byte order 1: wrap input byte order
pub type ByteRevW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - crc data length
    #[inline(always)]
    pub fn poly_width(&self) -> PolyWidthR {
        PolyWidthR::new((self.bits & 0x3f) as u8)
    }
    ///Bit 8 - 0: no wrap input bit order 1: wrap input bit order
    #[inline(always)]
    pub fn rev_in(&self) -> RevInR {
        RevInR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - 0: no wrap output bit order 1: wrap output bit order
    #[inline(always)]
    pub fn rev_out(&self) -> RevOutR {
        RevOutR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - 0: no wrap input byte order 1: wrap input byte order
    #[inline(always)]
    pub fn byte_rev(&self) -> ByteRevR {
        ByteRevR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - crc data length
    #[inline(always)]
    pub fn poly_width(&mut self) -> PolyWidthW<'_, MiscSettingSpec> {
        PolyWidthW::new(self, 0)
    }
    ///Bit 8 - 0: no wrap input bit order 1: wrap input bit order
    #[inline(always)]
    pub fn rev_in(&mut self) -> RevInW<'_, MiscSettingSpec> {
        RevInW::new(self, 8)
    }
    ///Bit 16 - 0: no wrap output bit order 1: wrap output bit order
    #[inline(always)]
    pub fn rev_out(&mut self) -> RevOutW<'_, MiscSettingSpec> {
        RevOutW::new(self, 16)
    }
    ///Bit 24 - 0: no wrap input byte order 1: wrap input byte order
    #[inline(always)]
    pub fn byte_rev(&mut self) -> ByteRevW<'_, MiscSettingSpec> {
        ByteRevW::new(self, 24)
    }
}
/**chn&index0 misc_setting

You can [`read`](crate::Reg::read) this register and get [`misc_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MiscSettingSpec;
impl crate::RegisterSpec for MiscSettingSpec {
    type Ux = u32;
}
///`read()` method returns [`misc_setting::R`](R) reader structure
impl crate::Readable for MiscSettingSpec {}
///`write(|w| ..)` method takes [`misc_setting::W`](W) writer structure
impl crate::Writable for MiscSettingSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets misc_setting to value 0
impl crate::Resettable for MiscSettingSpec {}
