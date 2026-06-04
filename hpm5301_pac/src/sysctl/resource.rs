///Register `RESOURCE[%s]` reader
pub type R = crate::R<ResourceSpec>;
///Register `RESOURCE[%s]` writer
pub type W = crate::W<ResourceSpec>;
///Field `MODE` reader - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved
pub type ModeR = crate::FieldReader;
///Field `MODE` writer - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LOC_BUSY` reader - local busy 0: no change is pending for current node 1: current node is changing status
pub type LocBusyR = crate::BitReader;
///Field `GLB_BUSY` reader - global busy 0: no changes pending to any nodes 1: any of nodes is changing status
pub type GlbBusyR = crate::BitReader;
impl R {
    ///Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    ///Bit 30 - local busy 0: no change is pending for current node 1: current node is changing status
    #[inline(always)]
    pub fn loc_busy(&self) -> LocBusyR {
        LocBusyR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - global busy 0: no changes pending to any nodes 1: any of nodes is changing status
    #[inline(always)]
    pub fn glb_busy(&self) -> GlbBusyR {
        GlbBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ResourceSpec> {
        ModeW::new(self, 0)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`resource::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resource::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResourceSpec;
impl crate::RegisterSpec for ResourceSpec {
    type Ux = u32;
}
///`read()` method returns [`resource::R`](R) reader structure
impl crate::Readable for ResourceSpec {}
///`write(|w| ..)` method takes [`resource::W`](W) writer structure
impl crate::Writable for ResourceSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RESOURCE[%s] to value 0
impl crate::Resettable for ResourceSpec {}
