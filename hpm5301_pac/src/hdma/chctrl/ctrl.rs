///Register `Ctrl` reader
pub type R = crate::R<CtrlSpec>;
///Register `Ctrl` writer
pub type W = crate::W<CtrlSpec>;
///Field `ENABLE` reader - Channel enable bit 0x0: Disable 0x1: Enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - Channel enable bit 0x0: Disable 0x1: Enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTTCMASK` reader - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt
pub type InttcmaskR = crate::BitReader;
///Field `INTTCMASK` writer - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt
pub type InttcmaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTERRMASK` reader - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt
pub type InterrmaskR = crate::BitReader;
///Field `INTERRMASK` writer - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt
pub type InterrmaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTABTMASK` reader - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt
pub type IntabtmaskR = crate::BitReader;
///Field `INTABTMASK` writer - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt
pub type IntabtmaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTHALFCNTMASK` reader - Channel half interrupt mask 0x0: Allow the half interrupt to be triggered 0x1: Disable the half interrupt
pub type InthalfcntmaskR = crate::BitReader;
///Field `INTHALFCNTMASK` writer - Channel half interrupt mask 0x0: Allow the half interrupt to be triggered 0x1: Disable the half interrupt
pub type InthalfcntmaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSTADDRCTRL` reader - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
pub type DstaddrctrlR = crate::FieldReader;
///Field `DSTADDRCTRL` writer - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
pub type DstaddrctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRCADDRCTRL` reader - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
pub type SrcaddrctrlR = crate::FieldReader;
///Field `SRCADDRCTRL` writer - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
pub type SrcaddrctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DSTMODE` reader - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode the difference bewteen Source/Destination handshake mode is: the dma block will response hardware request after read in Source handshake mode; the dma block will response hardware request after write in Destination handshake mode; NOTE: can't set SrcMode and DstMode at same time, otherwise result unknown.
pub type DstmodeR = crate::BitReader;
///Field `DSTMODE` writer - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode the difference bewteen Source/Destination handshake mode is: the dma block will response hardware request after read in Source handshake mode; the dma block will response hardware request after write in Destination handshake mode; NOTE: can't set SrcMode and DstMode at same time, otherwise result unknown.
pub type DstmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRCMODE` reader - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode Normal mode is enabled and started by software set Enable bit; Handshake mode is enabled by software set Enable bit, started by hardware dma request from DMAMUX block
pub type SrcmodeR = crate::BitReader;
///Field `SRCMODE` writer - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode Normal mode is enabled and started by software set Enable bit; Handshake mode is enabled by software set Enable bit, started by hardware dma request from DMAMUX block
pub type SrcmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSTWIDTH` reader - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
pub type DstwidthR = crate::FieldReader;
///Field `DSTWIDTH` writer - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
pub type DstwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SRCWIDTH` reader - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
pub type SrcwidthR = crate::FieldReader;
///Field `SRCWIDTH` writer - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
pub type SrcwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SRCBURSTSIZE` reader - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb - 0xf: Reserved, setting this field with a reserved value triggers the error exception
pub type SrcburstsizeR = crate::FieldReader;
///Field `SRCBURSTSIZE` writer - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb - 0xf: Reserved, setting this field with a reserved value triggers the error exception
pub type SrcburstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BURSTOPT` reader - set to change burst_size definition
pub type BurstoptR = crate::BitReader;
///Field `BURSTOPT` writer - set to change burst_size definition
pub type BurstoptW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIORITY` reader - Channel priority level 0x0: Lower priority 0x1: Higher priority
pub type PriorityR = crate::BitReader;
///Field `PRIORITY` writer - Channel priority level 0x0: Lower priority 0x1: Higher priority
pub type PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HANDSHAKEOPT` reader - 0: one request to transfer one burst 1: one request to transfer all the data defined in ch_tts
pub type HandshakeoptR = crate::BitReader;
///Field `HANDSHAKEOPT` writer - 0: one request to transfer one burst 1: one request to transfer all the data defined in ch_tts
pub type HandshakeoptW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INFINITELOOP` reader - set to loop current config infinitely
pub type InfiniteloopR = crate::BitReader;
///Field `INFINITELOOP` writer - set to loop current config infinitely
pub type InfiniteloopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Channel enable bit 0x0: Disable 0x1: Enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt
    #[inline(always)]
    pub fn inttcmask(&self) -> InttcmaskR {
        InttcmaskR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt
    #[inline(always)]
    pub fn interrmask(&self) -> InterrmaskR {
        InterrmaskR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt
    #[inline(always)]
    pub fn intabtmask(&self) -> IntabtmaskR {
        IntabtmaskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel half interrupt mask 0x0: Allow the half interrupt to be triggered 0x1: Disable the half interrupt
    #[inline(always)]
    pub fn inthalfcntmask(&self) -> InthalfcntmaskR {
        InthalfcntmaskR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 12:13 - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
    #[inline(always)]
    pub fn dstaddrctrl(&self) -> DstaddrctrlR {
        DstaddrctrlR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
    #[inline(always)]
    pub fn srcaddrctrl(&self) -> SrcaddrctrlR {
        SrcaddrctrlR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode the difference bewteen Source/Destination handshake mode is: the dma block will response hardware request after read in Source handshake mode; the dma block will response hardware request after write in Destination handshake mode; NOTE: can't set SrcMode and DstMode at same time, otherwise result unknown.
    #[inline(always)]
    pub fn dstmode(&self) -> DstmodeR {
        DstmodeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode Normal mode is enabled and started by software set Enable bit; Handshake mode is enabled by software set Enable bit, started by hardware dma request from DMAMUX block
    #[inline(always)]
    pub fn srcmode(&self) -> SrcmodeR {
        SrcmodeR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:20 - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
    #[inline(always)]
    pub fn dstwidth(&self) -> DstwidthR {
        DstwidthR::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
    #[inline(always)]
    pub fn srcwidth(&self) -> SrcwidthR {
        SrcwidthR::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:27 - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb - 0xf: Reserved, setting this field with a reserved value triggers the error exception
    #[inline(always)]
    pub fn srcburstsize(&self) -> SrcburstsizeR {
        SrcburstsizeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - set to change burst_size definition
    #[inline(always)]
    pub fn burstopt(&self) -> BurstoptR {
        BurstoptR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Channel priority level 0x0: Lower priority 0x1: Higher priority
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - 0: one request to transfer one burst 1: one request to transfer all the data defined in ch_tts
    #[inline(always)]
    pub fn handshakeopt(&self) -> HandshakeoptR {
        HandshakeoptR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - set to loop current config infinitely
    #[inline(always)]
    pub fn infiniteloop(&self) -> InfiniteloopR {
        InfiniteloopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Channel enable bit 0x0: Disable 0x1: Enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlSpec> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt
    #[inline(always)]
    pub fn inttcmask(&mut self) -> InttcmaskW<'_, CtrlSpec> {
        InttcmaskW::new(self, 1)
    }
    ///Bit 2 - Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt
    #[inline(always)]
    pub fn interrmask(&mut self) -> InterrmaskW<'_, CtrlSpec> {
        InterrmaskW::new(self, 2)
    }
    ///Bit 3 - Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt
    #[inline(always)]
    pub fn intabtmask(&mut self) -> IntabtmaskW<'_, CtrlSpec> {
        IntabtmaskW::new(self, 3)
    }
    ///Bit 4 - Channel half interrupt mask 0x0: Allow the half interrupt to be triggered 0x1: Disable the half interrupt
    #[inline(always)]
    pub fn inthalfcntmask(&mut self) -> InthalfcntmaskW<'_, CtrlSpec> {
        InthalfcntmaskW::new(self, 4)
    }
    ///Bits 12:13 - Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
    #[inline(always)]
    pub fn dstaddrctrl(&mut self) -> DstaddrctrlW<'_, CtrlSpec> {
        DstaddrctrlW::new(self, 12)
    }
    ///Bits 14:15 - Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception
    #[inline(always)]
    pub fn srcaddrctrl(&mut self) -> SrcaddrctrlW<'_, CtrlSpec> {
        SrcaddrctrlW::new(self, 14)
    }
    ///Bit 16 - Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode the difference bewteen Source/Destination handshake mode is: the dma block will response hardware request after read in Source handshake mode; the dma block will response hardware request after write in Destination handshake mode; NOTE: can't set SrcMode and DstMode at same time, otherwise result unknown.
    #[inline(always)]
    pub fn dstmode(&mut self) -> DstmodeW<'_, CtrlSpec> {
        DstmodeW::new(self, 16)
    }
    ///Bit 17 - Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode Normal mode is enabled and started by software set Enable bit; Handshake mode is enabled by software set Enable bit, started by hardware dma request from DMAMUX block
    #[inline(always)]
    pub fn srcmode(&mut self) -> SrcmodeW<'_, CtrlSpec> {
        SrcmodeW::new(self, 17)
    }
    ///Bits 18:20 - Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
    #[inline(always)]
    pub fn dstwidth(&mut self) -> DstwidthW<'_, CtrlSpec> {
        DstwidthW::new(self, 18)
    }
    ///Bits 21:23 - Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception
    #[inline(always)]
    pub fn srcwidth(&mut self) -> SrcwidthW<'_, CtrlSpec> {
        SrcwidthW::new(self, 21)
    }
    ///Bits 24:27 - Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb - 0xf: Reserved, setting this field with a reserved value triggers the error exception
    #[inline(always)]
    pub fn srcburstsize(&mut self) -> SrcburstsizeW<'_, CtrlSpec> {
        SrcburstsizeW::new(self, 24)
    }
    ///Bit 28 - set to change burst_size definition
    #[inline(always)]
    pub fn burstopt(&mut self) -> BurstoptW<'_, CtrlSpec> {
        BurstoptW::new(self, 28)
    }
    ///Bit 29 - Channel priority level 0x0: Lower priority 0x1: Higher priority
    #[inline(always)]
    pub fn priority(&mut self) -> PriorityW<'_, CtrlSpec> {
        PriorityW::new(self, 29)
    }
    ///Bit 30 - 0: one request to transfer one burst 1: one request to transfer all the data defined in ch_tts
    #[inline(always)]
    pub fn handshakeopt(&mut self) -> HandshakeoptW<'_, CtrlSpec> {
        HandshakeoptW::new(self, 30)
    }
    ///Bit 31 - set to loop current config infinitely
    #[inline(always)]
    pub fn infiniteloop(&mut self) -> InfiniteloopW<'_, CtrlSpec> {
        InfiniteloopW::new(self, 31)
    }
}
/**Channel &index0 Control Register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CtrlSpec {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Ctrl to value 0
impl crate::Resettable for CtrlSpec {}
