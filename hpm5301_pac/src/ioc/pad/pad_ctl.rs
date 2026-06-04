///Register `PAD_CTL` reader
pub type R = crate::R<PadCtlSpec>;
///Register `PAD_CTL` writer
pub type W = crate::W<PadCtlSpec>;
///Field `DS` reader - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm
pub type DsR = crate::FieldReader;
///Field `DS` writer - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPD` reader - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)
pub type SpdR = crate::FieldReader;
///Field `SPD` writer - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)
pub type SpdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SR` reader - slew rate 0: Slow slew rate 1: Fast slew rate
pub type SrR = crate::BitReader;
///Field `SR` writer - slew rate 0: Slow slew rate 1: Fast slew rate
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD` reader - open drain 0: open drain disable 1: open drain enable
pub type OdR = crate::BitReader;
///Field `OD` writer - open drain 0: open drain disable 1: open drain enable
pub type OdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KE` reader - keeper capability enable 0: keeper disable 1: keeper enable
pub type KeR = crate::BitReader;
///Field `KE` writer - keeper capability enable 0: keeper disable 1: keeper enable
pub type KeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PE` reader - pull enable 0: pull disable 1: pull enable
pub type PeR = crate::BitReader;
///Field `PE` writer - pull enable 0: pull disable 1: pull enable
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - pull select 0: pull down 1: pull up
pub type PsR = crate::BitReader;
///Field `PS` writer - pull select 0: pull down 1: pull up
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRS` reader - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm
pub type PrsR = crate::FieldReader;
///Field `PRS` writer - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HYS` reader - schmitt trigger enable 0: disable 1: enable
pub type HysR = crate::BitReader;
///Field `HYS` writer - schmitt trigger enable 0: disable 1: enable
pub type HysW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new((self.bits & 7) as u8)
    }
    ///Bits 4:5 - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - slew rate 0: Slow slew rate 1: Fast slew rate
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - open drain 0: open drain disable 1: open drain enable
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - keeper capability enable 0: keeper disable 1: keeper enable
    #[inline(always)]
    pub fn ke(&self) -> KeR {
        KeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - pull enable 0: pull disable 1: pull enable
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - pull select 0: pull down 1: pull up
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:21 - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 24 - schmitt trigger enable 0: disable 1: enable
    #[inline(always)]
    pub fn hys(&self) -> HysR {
        HysR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<'_, PadCtlSpec> {
        DsW::new(self, 0)
    }
    ///Bits 4:5 - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)
    #[inline(always)]
    pub fn spd(&mut self) -> SpdW<'_, PadCtlSpec> {
        SpdW::new(self, 4)
    }
    ///Bit 6 - slew rate 0: Slow slew rate 1: Fast slew rate
    #[inline(always)]
    pub fn sr(&mut self) -> SrW<'_, PadCtlSpec> {
        SrW::new(self, 6)
    }
    ///Bit 8 - open drain 0: open drain disable 1: open drain enable
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, PadCtlSpec> {
        OdW::new(self, 8)
    }
    ///Bit 16 - keeper capability enable 0: keeper disable 1: keeper enable
    #[inline(always)]
    pub fn ke(&mut self) -> KeW<'_, PadCtlSpec> {
        KeW::new(self, 16)
    }
    ///Bit 17 - pull enable 0: pull disable 1: pull enable
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, PadCtlSpec> {
        PeW::new(self, 17)
    }
    ///Bit 18 - pull select 0: pull down 1: pull up
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, PadCtlSpec> {
        PsW::new(self, 18)
    }
    ///Bits 20:21 - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, PadCtlSpec> {
        PrsW::new(self, 20)
    }
    ///Bit 24 - schmitt trigger enable 0: disable 1: enable
    #[inline(always)]
    pub fn hys(&mut self) -> HysW<'_, PadCtlSpec> {
        HysW::new(self, 24)
    }
}
/**PAD SETTINGS

You can [`read`](crate::Reg::read) this register and get [`pad_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PadCtlSpec;
impl crate::RegisterSpec for PadCtlSpec {
    type Ux = u32;
}
///`read()` method returns [`pad_ctl::R`](R) reader structure
impl crate::Readable for PadCtlSpec {}
///`write(|w| ..)` method takes [`pad_ctl::W`](W) writer structure
impl crate::Writable for PadCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PAD_CTL to value 0x0101_0056
impl crate::Resettable for PadCtlSpec {
    const RESET_VALUE: u32 = 0x0101_0056;
}
