///Register `FILTCFG[%s]` reader
pub type R = crate::R<FiltcfgSpec>;
///Register `FILTCFG[%s]` writer
pub type W = crate::W<FiltcfgSpec>;
///Field `FILTLEN_BASE` reader - This bitfields defines the filter counter length.
pub type FiltlenBaseR = crate::FieldReader<u16>;
///Field `FILTLEN_BASE` writer - This bitfields defines the filter counter length.
pub type FiltlenBaseW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `FILTLEN_SHIFT` reader - No description available
pub type FiltlenShiftR = crate::FieldReader;
///Field `FILTLEN_SHIFT` writer - No description available
pub type FiltlenShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SYNCEN` reader - set to enable sychronization input signal with TRGM clock
pub type SyncenR = crate::BitReader;
///Field `SYNCEN` writer - set to enable sychronization input signal with TRGM clock
pub type SyncenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode
pub type ModeR = crate::FieldReader;
///Field `MODE` writer - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OUTINV` reader - 1- Filter will invert the output 0- Filter will not invert the output
pub type OutinvR = crate::BitReader;
///Field `OUTINV` writer - 1- Filter will invert the output 0- Filter will not invert the output
pub type OutinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - This bitfields defines the filter counter length.
    #[inline(always)]
    pub fn filtlen_base(&self) -> FiltlenBaseR {
        FiltlenBaseR::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:11 - No description available
    #[inline(always)]
    pub fn filtlen_shift(&self) -> FiltlenShiftR {
        FiltlenShiftR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 12 - set to enable sychronization input signal with TRGM clock
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - 1- Filter will invert the output 0- Filter will not invert the output
    #[inline(always)]
    pub fn outinv(&self) -> OutinvR {
        OutinvR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - This bitfields defines the filter counter length.
    #[inline(always)]
    pub fn filtlen_base(&mut self) -> FiltlenBaseW<'_, FiltcfgSpec> {
        FiltlenBaseW::new(self, 0)
    }
    ///Bits 9:11 - No description available
    #[inline(always)]
    pub fn filtlen_shift(&mut self) -> FiltlenShiftW<'_, FiltcfgSpec> {
        FiltlenShiftW::new(self, 9)
    }
    ///Bit 12 - set to enable sychronization input signal with TRGM clock
    #[inline(always)]
    pub fn syncen(&mut self) -> SyncenW<'_, FiltcfgSpec> {
        SyncenW::new(self, 12)
    }
    ///Bits 13:15 - This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, FiltcfgSpec> {
        ModeW::new(self, 13)
    }
    ///Bit 16 - 1- Filter will invert the output 0- Filter will not invert the output
    #[inline(always)]
    pub fn outinv(&mut self) -> OutinvW<'_, FiltcfgSpec> {
        OutinvW::new(self, 16)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`filtcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filtcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FiltcfgSpec;
impl crate::RegisterSpec for FiltcfgSpec {
    type Ux = u32;
}
///`read()` method returns [`filtcfg::R`](R) reader structure
impl crate::Readable for FiltcfgSpec {}
///`write(|w| ..)` method takes [`filtcfg::W`](W) writer structure
impl crate::Writable for FiltcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FILTCFG[%s] to value 0
impl crate::Resettable for FiltcfgSpec {}
