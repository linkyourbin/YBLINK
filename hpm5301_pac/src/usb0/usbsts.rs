///Register `USBSTS` reader
pub type R = crate::R<UsbstsSpec>;
///Register `USBSTS` writer
pub type W = crate::W<UsbstsSpec>;
///Field `UI` reader - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes.
pub type UiR = crate::BitReader;
///Field `UI` writer - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes.
pub type UiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEI` reader - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set.
pub type UeiR = crate::BitReader;
///Field `UEI` writer - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set.
pub type UeiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCI` reader - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively.
pub type PciR = crate::BitReader;
///Field `PCI` writer - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively.
pub type PciW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRI` reader - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \[13\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \[12\] toggles. Only used in host operation mode.
pub type FriR = crate::BitReader;
///Field `FRI` writer - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \[13\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \[12\] toggles. Only used in host operation mode.
pub type FriW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEI` reader - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\[1:0\]=ERROR)
pub type SeiR = crate::BitReader;
///Field `SEI` writer - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\[1:0\]=ERROR)
pub type SeiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AAI` reader - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode.
pub type AaiR = crate::BitReader;
///Field `AAI` writer - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode.
pub type AaiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `URI` reader - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode.
pub type UriR = crate::BitReader;
///Field `URI` writer - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode.
pub type UriW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRI` reader - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it.
pub type SriR = crate::BitReader;
///Field `SRI` writer - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it.
pub type SriW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLI` reader - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode.
pub type SliR = crate::BitReader;
///Field `SLI` writer - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode.
pub type SliW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HCH` reader - HCH HCHaIted - Read Only. This bit is a zero whenever the Run/Stop bit is a one. The Controller sets this bit to one after it has stopped executing because of the Run/Stop bit being set to 0, either by software or by the Controller hardware (for example, an internal error). Only used in the host operation mode. Default value is '0b' for OTG core . This is because OTG core is not operating as host in default. Please see CM bit in USB_n_USBMODE register. NOTE: HCH bit reset value: '0b' for OTG controller core .
pub type HchR = crate::BitReader;
///Field `RCL` reader - RCL Reclamation - Read Only. This is a read-only status bit used to detect an empty asynchronous schedule. Only used in the host operation mode.
pub type RclR = crate::BitReader;
///Field `PS` reader - PS Periodic Schedule Status - Read Only. This bit reports the current real status of the Periodic Schedule. When set to zero the periodic schedule is disabled, and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Periodic Schedule when software transitions the Periodic Schedule Enable bit in the USBCMD register. When this bit and the Periodic Schedule Enable bit are the same value, the Periodic Schedule is either enabled (1) or disabled (0). Only used in the host operation mode.
pub type PsR = crate::BitReader;
///Field `AS` reader - AS Asynchronous Schedule Status - Read Only. This bit reports the current real status of the Asynchronous Schedule. When set to zero the asynchronous schedule status is disabled and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (1) or disabled (0). Only used in the host operation mode.
pub type AsR = crate::BitReader;
///Field `NAKI` reader - NAKI NAK Interrupt Bit--RO. This bit is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK bit and corresponding TX/RX Endpoint NAK Enable bit are set. This bit is automatically cleared by hardware when all Enabled TX/RX Endpoint NAK bits are cleared.
pub type NakiR = crate::BitReader;
///Field `UAI` reader - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero
pub type UaiR = crate::BitReader;
///Field `UAI` writer - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero
pub type UaiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPI` reader - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero.
pub type UpiR = crate::BitReader;
///Field `UPI` writer - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero.
pub type UpiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TI0` reader - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it.
pub type Ti0R = crate::BitReader;
///Field `TI0` writer - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it.
pub type Ti0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TI1` reader - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it.
pub type Ti1R = crate::BitReader;
///Field `TI1` writer - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it.
pub type Ti1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes.
    #[inline(always)]
    pub fn ui(&self) -> UiR {
        UiR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set.
    #[inline(always)]
    pub fn uei(&self) -> UeiR {
        UeiR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively.
    #[inline(always)]
    pub fn pci(&self) -> PciR {
        PciR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \[13\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \[12\] toggles. Only used in host operation mode.
    #[inline(always)]
    pub fn fri(&self) -> FriR {
        FriR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\[1:0\]=ERROR)
    #[inline(always)]
    pub fn sei(&self) -> SeiR {
        SeiR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode.
    #[inline(always)]
    pub fn aai(&self) -> AaiR {
        AaiR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode.
    #[inline(always)]
    pub fn uri(&self) -> UriR {
        UriR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it.
    #[inline(always)]
    pub fn sri(&self) -> SriR {
        SriR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode.
    #[inline(always)]
    pub fn sli(&self) -> SliR {
        SliR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - HCH HCHaIted - Read Only. This bit is a zero whenever the Run/Stop bit is a one. The Controller sets this bit to one after it has stopped executing because of the Run/Stop bit being set to 0, either by software or by the Controller hardware (for example, an internal error). Only used in the host operation mode. Default value is '0b' for OTG core . This is because OTG core is not operating as host in default. Please see CM bit in USB_n_USBMODE register. NOTE: HCH bit reset value: '0b' for OTG controller core .
    #[inline(always)]
    pub fn hch(&self) -> HchR {
        HchR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RCL Reclamation - Read Only. This is a read-only status bit used to detect an empty asynchronous schedule. Only used in the host operation mode.
    #[inline(always)]
    pub fn rcl(&self) -> RclR {
        RclR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PS Periodic Schedule Status - Read Only. This bit reports the current real status of the Periodic Schedule. When set to zero the periodic schedule is disabled, and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Periodic Schedule when software transitions the Periodic Schedule Enable bit in the USBCMD register. When this bit and the Periodic Schedule Enable bit are the same value, the Periodic Schedule is either enabled (1) or disabled (0). Only used in the host operation mode.
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AS Asynchronous Schedule Status - Read Only. This bit reports the current real status of the Asynchronous Schedule. When set to zero the asynchronous schedule status is disabled and if set to one the status is enabled. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (1) or disabled (0). Only used in the host operation mode.
    #[inline(always)]
    pub fn as_(&self) -> AsR {
        AsR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - NAKI NAK Interrupt Bit--RO. This bit is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK bit and corresponding TX/RX Endpoint NAK Enable bit are set. This bit is automatically cleared by hardware when all Enabled TX/RX Endpoint NAK bits are cleared.
    #[inline(always)]
    pub fn naki(&self) -> NakiR {
        NakiR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero
    #[inline(always)]
    pub fn uai(&self) -> UaiR {
        UaiR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero.
    #[inline(always)]
    pub fn upi(&self) -> UpiR {
        UpiR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it.
    #[inline(always)]
    pub fn ti0(&self) -> Ti0R {
        Ti0R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it.
    #[inline(always)]
    pub fn ti1(&self) -> Ti1R {
        Ti1R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - UI USB Interrupt (USBINT) - R/WC. This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes.
    #[inline(always)]
    pub fn ui(&mut self) -> UiW<'_, UsbstsSpec> {
        UiW::new(self, 0)
    }
    ///Bit 1 - UEI USB Error Interrupt (USBERRINT) - R/WC. When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set.
    #[inline(always)]
    pub fn uei(&mut self) -> UeiW<'_, UsbstsSpec> {
        UeiW::new(self, 1)
    }
    ///Bit 2 - PCI Port Change Detect - R/WC. The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port. The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit and the DCSuspend bits Respectively.
    #[inline(always)]
    pub fn pci(&mut self) -> PciW<'_, UsbstsSpec> {
        PciW::new(self, 2)
    }
    ///Bit 3 - FRI Frame List Rollover - R/WC. The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example. If the frame list size (as programmed in the Frame List Size field of the USB_n_USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX \[13\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FHINDEX \[12\] toggles. Only used in host operation mode.
    #[inline(always)]
    pub fn fri(&mut self) -> FriW<'_, UsbstsSpec> {
        FriW::new(self, 3)
    }
    ///Bit 4 - System Error – RWC. Default = 0b. In the BVCI implementation of the USBHS core, this bit is not used, and will always be cleared to '0b'. In the AMBA implementation, this bit will be set to '1b' when an Error response is seen by the master interface (HRESP\[1:0\]=ERROR)
    #[inline(always)]
    pub fn sei(&mut self) -> SeiW<'_, UsbstsSpec> {
        SeiW::new(self, 4)
    }
    ///Bit 5 - AAI Interrupt on Async Advance - R/WC. System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the n_USBCMD register. This status bit indicates the assertion of that interrupt source. Only used in host operation mode.
    #[inline(always)]
    pub fn aai(&mut self) -> AaiW<'_, UsbstsSpec> {
        AaiW::new(self, 5)
    }
    ///Bit 6 - URI USB Reset Received - R/WC. When the device controller detects a USB Reset and enters the default state, this bit will be set to a one. Software can write a 1 to this bit to clear the USB Reset Received status bit. Only used in device operation mode.
    #[inline(always)]
    pub fn uri(&mut self) -> UriW<'_, UsbstsSpec> {
        UriW::new(self, 6)
    }
    ///Bit 7 - SRI SOF Received - R/WC. When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1ms in device FS mode and every 125ms in HS mode and will be synchronized to the actual SOF that is received. Because the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp. In host mode, this bit will be set every 125us and can be used by host controller driver as a time base. Software writes a 1 to this bit to clear it.
    #[inline(always)]
    pub fn sri(&mut self) -> SriW<'_, UsbstsSpec> {
        SriW::new(self, 7)
    }
    ///Bit 8 - SLI DCSuspend - R/WC. When a controller enters a suspend state from an active state, this bit will be set to a one. The device controller clears the bit upon exiting from a suspend state. Only used in device operation mode.
    #[inline(always)]
    pub fn sli(&mut self) -> SliW<'_, UsbstsSpec> {
        SliW::new(self, 8)
    }
    ///Bit 18 - USB Host Asynchronous Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set AND the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero
    #[inline(always)]
    pub fn uai(&mut self) -> UaiW<'_, UsbstsSpec> {
        UaiW::new(self, 18)
    }
    ///Bit 19 - USB Host Periodic Interrupt – RWC. Default = 0b. This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than expected. This bit is not used by the device controller and will always be zero.
    #[inline(always)]
    pub fn upi(&mut self) -> UpiW<'_, UsbstsSpec> {
        UpiW::new(self, 19)
    }
    ///Bit 24 - TI0 General Purpose Timer Interrupt 0(GPTINT0)--R/WC. This bit is set when the counter in the GPTIMER0CTRL register transitions to zero, writing a one to this bit clears it.
    #[inline(always)]
    pub fn ti0(&mut self) -> Ti0W<'_, UsbstsSpec> {
        Ti0W::new(self, 24)
    }
    ///Bit 25 - TI1 General Purpose Timer Interrupt 1(GPTINT1)--R/WC. This bit is set when the counter in the GPTIMER1CTRL register transitions to zero, writing a one to this bit will clear it.
    #[inline(always)]
    pub fn ti1(&mut self) -> Ti1W<'_, UsbstsSpec> {
        Ti1W::new(self, 25)
    }
}
/**USB Status Register

You can [`read`](crate::Reg::read) this register and get [`usbsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbstsSpec;
impl crate::RegisterSpec for UsbstsSpec {
    type Ux = u32;
}
///`read()` method returns [`usbsts::R`](R) reader structure
impl crate::Readable for UsbstsSpec {}
///`write(|w| ..)` method takes [`usbsts::W`](W) writer structure
impl crate::Writable for UsbstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBSTS to value 0
impl crate::Resettable for UsbstsSpec {}
