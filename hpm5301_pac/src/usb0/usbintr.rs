///Register `USBINTR` reader
pub type R = crate::R<UsbintrSpec>;
///Register `USBINTR` writer
pub type W = crate::W<UsbintrSpec>;
///Field `UE` reader - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type UeR = crate::BitReader;
///Field `UE` writer - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEE` reader - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type UeeR = crate::BitReader;
///Field `UEE` writer - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type UeeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCE` reader - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type PceR = crate::BitReader;
///Field `PCE` writer - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRE` reader - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
pub type FreR = crate::BitReader;
///Field `FRE` writer - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
pub type FreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEE` reader - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
pub type SeeR = crate::BitReader;
///Field `SEE` writer - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
pub type SeeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AAE` reader - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
pub type AaeR = crate::BitReader;
///Field `AAE` writer - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
pub type AaeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `URE` reader - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
pub type UreR = crate::BitReader;
///Field `URE` writer - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
pub type UreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRE` reader - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type SreR = crate::BitReader;
///Field `SRE` writer - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type SreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLE` reader - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
pub type SleR = crate::BitReader;
///Field `SLE` writer - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
pub type SleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAKE` reader - NAKE NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type NakeR = crate::BitReader;
///Field `UAIE` reader - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
pub type UaieR = crate::BitReader;
///Field `UAIE` writer - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
pub type UaieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPIE` reader - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
pub type UpieR = crate::BitReader;
///Field `UPIE` writer - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE0` reader - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type Tie0R = crate::BitReader;
///Field `TIE0` writer - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type Tie0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE1` reader - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type Tie1R = crate::BitReader;
///Field `TIE1` writer - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt.
pub type Tie1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn uee(&self) -> UeeR {
        UeeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
    #[inline(always)]
    pub fn fre(&self) -> FreR {
        FreR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
    #[inline(always)]
    pub fn see(&self) -> SeeR {
        SeeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
    #[inline(always)]
    pub fn aae(&self) -> AaeR {
        AaeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
    #[inline(always)]
    pub fn ure(&self) -> UreR {
        UreR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn sre(&self) -> SreR {
        SreR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
    #[inline(always)]
    pub fn sle(&self) -> SleR {
        SleR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - NAKE NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn nake(&self) -> NakeR {
        NakeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
    #[inline(always)]
    pub fn uaie(&self) -> UaieR {
        UaieR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn tie0(&self) -> Tie0R {
        Tie0R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn tie1(&self) -> Tie1R {
        Tie1R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<'_, UsbintrSpec> {
        UeW::new(self, 0)
    }
    ///Bit 1 - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn uee(&mut self) -> UeeW<'_, UsbintrSpec> {
        UeeW::new(self, 1)
    }
    ///Bit 2 - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, UsbintrSpec> {
        PceW::new(self, 2)
    }
    ///Bit 3 - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
    #[inline(always)]
    pub fn fre(&mut self) -> FreW<'_, UsbintrSpec> {
        FreW::new(self, 3)
    }
    ///Bit 4 - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
    #[inline(always)]
    pub fn see(&mut self) -> SeeW<'_, UsbintrSpec> {
        SeeW::new(self, 4)
    }
    ///Bit 5 - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode.
    #[inline(always)]
    pub fn aae(&mut self) -> AaeW<'_, UsbintrSpec> {
        AaeW::new(self, 5)
    }
    ///Bit 6 - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
    #[inline(always)]
    pub fn ure(&mut self) -> UreW<'_, UsbintrSpec> {
        UreW::new(self, 6)
    }
    ///Bit 7 - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn sre(&mut self) -> SreW<'_, UsbintrSpec> {
        SreW::new(self, 7)
    }
    ///Bit 8 - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode.
    #[inline(always)]
    pub fn sle(&mut self) -> SleW<'_, UsbintrSpec> {
        SleW::new(self, 8)
    }
    ///Bit 18 - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
    #[inline(always)]
    pub fn uaie(&mut self) -> UaieW<'_, UsbintrSpec> {
        UaieW::new(self, 18)
    }
    ///Bit 19 - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold.
    #[inline(always)]
    pub fn upie(&mut self) -> UpieW<'_, UsbintrSpec> {
        UpieW::new(self, 19)
    }
    ///Bit 24 - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn tie0(&mut self) -> Tie0W<'_, UsbintrSpec> {
        Tie0W::new(self, 24)
    }
    ///Bit 25 - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt.
    #[inline(always)]
    pub fn tie1(&mut self) -> Tie1W<'_, UsbintrSpec> {
        Tie1W::new(self, 25)
    }
}
/**Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`usbintr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbintrSpec;
impl crate::RegisterSpec for UsbintrSpec {
    type Ux = u32;
}
///`read()` method returns [`usbintr::R`](R) reader structure
impl crate::Readable for UsbintrSpec {}
///`write(|w| ..)` method takes [`usbintr::W`](W) writer structure
impl crate::Writable for UsbintrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBINTR to value 0
impl crate::Resettable for UsbintrSpec {}
