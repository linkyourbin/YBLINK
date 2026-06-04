///Register `CLOCK[%s]` reader
pub type R = crate::R<ClockSpec>;
///Register `CLOCK[%s]` writer
pub type W = crate::W<ClockSpec>;
///Field `DIV` reader - clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256
pub type DivR = crate::FieldReader;
///Field `DIV` writer - clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MUX` reader - current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll1_clk2 7:pll1_clk3
pub type MuxR = crate::FieldReader;
///Field `MUX` writer - current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll1_clk2 7:pll1_clk3
pub type MuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PRESERVE` reader - preserve function against global select 0: select global clock setting 1: not select global clock setting
pub type PreserveR = crate::BitReader;
///Field `PRESERVE` writer - preserve function against global select 0: select global clock setting 1: not select global clock setting
pub type PreserveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOC_BUSY` reader - local busy 0: a change is pending for current node 1: current node is changing status
pub type LocBusyR = crate::BitReader;
///Field `GLB_BUSY` reader - global busy 0: no changes pending to any clock 1: any of nodes is changing status
pub type GlbBusyR = crate::BitReader;
impl R {
    ///Bits 0:7 - clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll1_clk2 7:pll1_clk3
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 28 - preserve function against global select 0: select global clock setting 1: not select global clock setting
    #[inline(always)]
    pub fn preserve(&self) -> PreserveR {
        PreserveR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - local busy 0: a change is pending for current node 1: current node is changing status
    #[inline(always)]
    pub fn loc_busy(&self) -> LocBusyR {
        LocBusyR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - global busy 0: no changes pending to any clock 1: any of nodes is changing status
    #[inline(always)]
    pub fn glb_busy(&self) -> GlbBusyR {
        GlbBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, ClockSpec> {
        DivW::new(self, 0)
    }
    ///Bits 8:10 - current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll1_clk2 7:pll1_clk3
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<'_, ClockSpec> {
        MuxW::new(self, 8)
    }
    ///Bit 28 - preserve function against global select 0: select global clock setting 1: not select global clock setting
    #[inline(always)]
    pub fn preserve(&mut self) -> PreserveW<'_, ClockSpec> {
        PreserveW::new(self, 28)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ClockSpec;
impl crate::RegisterSpec for ClockSpec {
    type Ux = u32;
}
///`read()` method returns [`clock::R`](R) reader structure
impl crate::Readable for ClockSpec {}
///`write(|w| ..)` method takes [`clock::W`](W) writer structure
impl crate::Writable for ClockSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLOCK[%s] to value 0
impl crate::Resettable for ClockSpec {}
