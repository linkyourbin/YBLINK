///Register `FRINDEX` reader
pub type R = crate::R<FrindexSpec>;
///Register `FRINDEX` writer
pub type W = crate::W<FrindexSpec>;
///Field `FRINDEX` reader - FRINDEX Frame Index. The value, in this register, increments at the end of each time frame (micro-frame). Bits \[N: 3\] are used for the Frame List current index. This means that each location of the frame list is accessed 8 times (frames or micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register, when used in host mode. USBCMD \[Frame List Size\] Number Elements N In device mode the value is the current frame number of the last frame transmitted. It is not used as an index. In either mode bits 2:0 indicate the current microframe. The bit field values description below is represented as (Frame List Size) Number Elements N. 00000000000000 - (1024) 12 00000000000001 - (512) 11 00000000000010 - (256) 10 00000000000011 - (128) 9 00000000000100 - (64) 8 00000000000101 - (32) 7 00000000000110 - (16) 6 00000000000111 - (8) 5
pub type FrindexR = crate::FieldReader<u16>;
///Field `FRINDEX` writer - FRINDEX Frame Index. The value, in this register, increments at the end of each time frame (micro-frame). Bits \[N: 3\] are used for the Frame List current index. This means that each location of the frame list is accessed 8 times (frames or micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register, when used in host mode. USBCMD \[Frame List Size\] Number Elements N In device mode the value is the current frame number of the last frame transmitted. It is not used as an index. In either mode bits 2:0 indicate the current microframe. The bit field values description below is represented as (Frame List Size) Number Elements N. 00000000000000 - (1024) 12 00000000000001 - (512) 11 00000000000010 - (256) 10 00000000000011 - (128) 9 00000000000100 - (64) 8 00000000000101 - (32) 7 00000000000110 - (16) 6 00000000000111 - (8) 5
pub type FrindexW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - FRINDEX Frame Index. The value, in this register, increments at the end of each time frame (micro-frame). Bits \[N: 3\] are used for the Frame List current index. This means that each location of the frame list is accessed 8 times (frames or micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register, when used in host mode. USBCMD \[Frame List Size\] Number Elements N In device mode the value is the current frame number of the last frame transmitted. It is not used as an index. In either mode bits 2:0 indicate the current microframe. The bit field values description below is represented as (Frame List Size) Number Elements N. 00000000000000 - (1024) 12 00000000000001 - (512) 11 00000000000010 - (256) 10 00000000000011 - (128) 9 00000000000100 - (64) 8 00000000000101 - (32) 7 00000000000110 - (16) 6 00000000000111 - (8) 5
    #[inline(always)]
    pub fn frindex(&self) -> FrindexR {
        FrindexR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - FRINDEX Frame Index. The value, in this register, increments at the end of each time frame (micro-frame). Bits \[N: 3\] are used for the Frame List current index. This means that each location of the frame list is accessed 8 times (frames or micro-frames) before moving to the next index. The following illustrates values of N based on the value of the Frame List Size field in the USBCMD register, when used in host mode. USBCMD \[Frame List Size\] Number Elements N In device mode the value is the current frame number of the last frame transmitted. It is not used as an index. In either mode bits 2:0 indicate the current microframe. The bit field values description below is represented as (Frame List Size) Number Elements N. 00000000000000 - (1024) 12 00000000000001 - (512) 11 00000000000010 - (256) 10 00000000000011 - (128) 9 00000000000100 - (64) 8 00000000000101 - (32) 7 00000000000110 - (16) 6 00000000000111 - (8) 5
    #[inline(always)]
    pub fn frindex(&mut self) -> FrindexW<'_, FrindexSpec> {
        FrindexW::new(self, 0)
    }
}
/**USB Frame Index Register

You can [`read`](crate::Reg::read) this register and get [`frindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FrindexSpec;
impl crate::RegisterSpec for FrindexSpec {
    type Ux = u32;
}
///`read()` method returns [`frindex::R`](R) reader structure
impl crate::Readable for FrindexSpec {}
///`write(|w| ..)` method takes [`frindex::W`](W) writer structure
impl crate::Writable for FrindexSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRINDEX to value 0
impl crate::Resettable for FrindexSpec {}
