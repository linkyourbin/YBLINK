///Register `PORTSC1` reader
pub type R = crate::R<Portsc1Spec>;
///Register `PORTSC1` writer
pub type W = crate::W<Portsc1Spec>;
///Field `CCS` reader - CCS Current Connect Status-Read Only. In Host Mode: 1=Device is present on port. 0=No device is present. Default = 0. This value reflects the current state of the port, and may not correspond directly to the event that caused the Connect Status Change bit (Bit 1) to be set. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: 1=Attached. 0=Not Attached. Default=0. A one indicates that the device successfully attached and is operating in either high speed or full speed as indicated by the High Speed Port bit in this register. A zero indicates that the device did not attach successfully or was forcibly disconnected by the software writing a zero to the Run bit in the USBCMD register. It does not state the device being disconnected or Suspended.
pub type CcsR = crate::BitReader;
///Field `CCS` writer - CCS Current Connect Status-Read Only. In Host Mode: 1=Device is present on port. 0=No device is present. Default = 0. This value reflects the current state of the port, and may not correspond directly to the event that caused the Connect Status Change bit (Bit 1) to be set. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: 1=Attached. 0=Not Attached. Default=0. A one indicates that the device successfully attached and is operating in either high speed or full speed as indicated by the High Speed Port bit in this register. A zero indicates that the device did not attach successfully or was forcibly disconnected by the software writing a zero to the Run bit in the USBCMD register. It does not state the device being disconnected or Suspended.
pub type CcsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSC` reader - CSC Connect Status Change-R/WC. 1 =Change in Current Connect Status. 0=No change. Default 0. In Host Mode: Indicates a change has occurred in the port's Current Connect Status. The host/device controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit (that is, the bit will remain set). Software clears this bit by writing a one to it. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: This bit is undefined in device controller mode.
pub type CscR = crate::BitReader;
///Field `CSC` writer - CSC Connect Status Change-R/WC. 1 =Change in Current Connect Status. 0=No change. Default 0. In Host Mode: Indicates a change has occurred in the port's Current Connect Status. The host/device controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit (that is, the bit will remain set). Software clears this bit by writing a one to it. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: This bit is undefined in device controller mode.
pub type CscW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PE` reader - PE Port Enabled/Disabled-Read/Write. 1=Enable. 0=Disable. Default 0. In Host Mode: Ports can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. Ports can be disabled by either a fault condition (disconnect event or other fault condition) or by the host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events. When the port is disabled, (0b) downstream propagation of data is blocked except for reset. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: The device port is always enabled, so this bit is always '1b'.
pub type PeR = crate::BitReader;
///Field `PE` writer - PE Port Enabled/Disabled-Read/Write. 1=Enable. 0=Disable. Default 0. In Host Mode: Ports can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. Ports can be disabled by either a fault condition (disconnect event or other fault condition) or by the host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events. When the port is disabled, (0b) downstream propagation of data is blocked except for reset. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: The device port is always enabled, so this bit is always '1b'.
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEC` reader - PEC Port Enable/Disable Change-R/WC. 1=Port enabled/disabled status has changed. 0=No change. Default = 0. In Host Mode: For the root hub, this bit is set to a one only when a port is disabled due to disconnect on the port or due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification). Software clears this by writing a one to it. This field is zero if Port Power(PORTSC1) is zero. In Device mode: The device port is always enabled, so this bit is always '0b'.
pub type PecR = crate::BitReader;
///Field `PEC` writer - PEC Port Enable/Disable Change-R/WC. 1=Port enabled/disabled status has changed. 0=No change. Default = 0. In Host Mode: For the root hub, this bit is set to a one only when a port is disabled due to disconnect on the port or due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification). Software clears this by writing a one to it. This field is zero if Port Power(PORTSC1) is zero. In Device mode: The device port is always enabled, so this bit is always '0b'.
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCA` reader - OCA Over-current Active-Read Only. Default 0. This bit will automatically transition from one to zero when the over current condition is removed. 0 - This port does not have an over-current condition. 1 - This port currently has an over-current condition
pub type OcaR = crate::BitReader;
///Field `OCC` reader - OCC Over-current Change-R/WC. Default=0. This bit is set '1b' by hardware when there is a change to Over-current Active. Software can clear this bit by writing a one to this bit position.
pub type OccR = crate::BitReader;
///Field `OCC` writer - OCC Over-current Change-R/WC. Default=0. This bit is set '1b' by hardware when there is a change to Over-current Active. Software can clear this bit by writing a one to this bit position.
pub type OccW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPR` reader - FPR Force Port Resume -Read/Write. 1= Resume detected/driven on port. 0=No resume (K-state) detected driven on port. Default = 0. In Host Mode: Software sets this bit to one to drive resume signaling. The Host Controller sets this bit to one if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to one. This bit will automatically change to zero after the resume sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the resume duration is timed in the driver. Note that when the Host controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this bit remains a one. This bit will remain a one until the port has switched to the high-speed idle. Writing a zero has no effect because the port controller will time the resume operation, clear the bit the port control state switches to HS or FS idle. This field is zero if Port Power(PORTSC1) is zero in host mode. This bit is not-EHCI compatible. In Device mode: After the device has been in Suspend State for 5ms or more, software must set this bit to one to drive resume signaling before clearing. The Device Controller will set this bit to one if a J-to-K transition is detected while the port is in the Suspend state. The bit will be cleared when the device returns to normal operation. Also, when this bit wil be cleared because a K-to-J transition detected, the Port Change Detect bit in the USBSTS register is also set to one.
pub type FprR = crate::BitReader;
///Field `FPR` writer - FPR Force Port Resume -Read/Write. 1= Resume detected/driven on port. 0=No resume (K-state) detected driven on port. Default = 0. In Host Mode: Software sets this bit to one to drive resume signaling. The Host Controller sets this bit to one if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to one. This bit will automatically change to zero after the resume sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the resume duration is timed in the driver. Note that when the Host controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this bit remains a one. This bit will remain a one until the port has switched to the high-speed idle. Writing a zero has no effect because the port controller will time the resume operation, clear the bit the port control state switches to HS or FS idle. This field is zero if Port Power(PORTSC1) is zero in host mode. This bit is not-EHCI compatible. In Device mode: After the device has been in Suspend State for 5ms or more, software must set this bit to one to drive resume signaling before clearing. The Device Controller will set this bit to one if a J-to-K transition is detected while the port is in the Suspend state. The bit will be cleared when the device returns to normal operation. Also, when this bit wil be cleared because a K-to-J transition detected, the Port Change Detect bit in the USBSTS register is also set to one.
pub type FprW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSP` reader - SUSP Suspend - Read/Write or Read Only. Default = 0b. 1=Port in suspend state. 0=Port not in suspend state. In Host Mode: Read/Write. Port Enabled Bit and Suspend bit of this register define the port states as follows: Bits \[Port Enabled, Suspend\] Port State 0x Disable 10 Enable 11 Suspend When in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Note that the bit status does not change until the port is suspended and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB. The host controller will unconditionally set this bit to zero when software sets the Force Port Resume bit to zero. The host controller ignores a write of zero to this bit. If host software sets this bit to a one when the port is not enabled (that is, Port enabled bit is a zero) the results are undefined. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: Read Only. In device mode this bit is a read only status bit.
pub type SuspR = crate::BitReader;
///Field `SUSP` writer - SUSP Suspend - Read/Write or Read Only. Default = 0b. 1=Port in suspend state. 0=Port not in suspend state. In Host Mode: Read/Write. Port Enabled Bit and Suspend bit of this register define the port states as follows: Bits \[Port Enabled, Suspend\] Port State 0x Disable 10 Enable 11 Suspend When in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Note that the bit status does not change until the port is suspended and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB. The host controller will unconditionally set this bit to zero when software sets the Force Port Resume bit to zero. The host controller ignores a write of zero to this bit. If host software sets this bit to a one when the port is not enabled (that is, Port enabled bit is a zero) the results are undefined. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: Read Only. In device mode this bit is a read only status bit.
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR` reader - PR Port Reset - Read/Write or Read Only. Default = 0b. In Host Mode: Read/Write. 1=Port is in Reset. 0=Port is not in Reset. Default 0. When software writes a one to this bit the bus-reset sequence as defined in the USB Specification Revision 2.0 is started. This bit will automatically change to zero after the reset sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the reset duration is timed in the driver. In Device Mode: This bit is a read only status bit. Device reset from the USB bus is also indicated in the USBSTS register.
pub type PrR = crate::BitReader;
///Field `PR` writer - PR Port Reset - Read/Write or Read Only. Default = 0b. In Host Mode: Read/Write. 1=Port is in Reset. 0=Port is not in Reset. Default 0. When software writes a one to this bit the bus-reset sequence as defined in the USB Specification Revision 2.0 is started. This bit will automatically change to zero after the reset sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the reset duration is timed in the driver. In Device Mode: This bit is a read only status bit. Device reset from the USB bus is also indicated in the USBSTS register.
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSP` reader - HSP High-Speed Port - Read Only. Default = 0b. When the bit is one, the host/device connected to the port is in high-speed mode and if set to zero, the host/device connected to the port is not in a high-speed mode. NOTE: HSP is redundant with PSPD(bit 27, 26) but remained for compatibility.
pub type HspR = crate::BitReader;
///Field `LS` reader - LS Line Status-Read Only. These bits reflect the current logical levels of the D+ (bit 11) and D- (bit 10) signal lines. In host mode, the use of linestate by the host controller driver is not necessary (unlike EHCI), because the port controller state machine and the port routing manage the connection of LS and FS. In device mode, the use of linestate by the device controller driver is not necessary. The encoding of the bits are: Bits \[11:10\] Meaning 00 - SE0 01 - K-state 10 - J-state 11 - Undefined
pub type LsR = crate::FieldReader;
///Field `PP` reader - PP Port Power (PP)-Read/Write or Read Only. The function of this bit depends on the value of the Port Power Switching (PPC) field in the HCSPARAMS register. The behavior is as follows: PPC PP Operation 0 1b Read Only - Host controller does not have port power control switches. Each port is hard-wired to power. 1 1b/0b - Read/Write. OTG controller requires port power control switches. This bit represents the current setting of the switch (0=off, 1=on). When power is not available on a port (that is, PP equals a 0), the port is non-functional and will not report attaches, detaches, etc. When an over-current condition is detected on a powered port and PPC is a one, the PP bit in each affected port may be transitional by the host controller driver from a one to a zero (removing power from the port). This feature is implemented in all controller cores (PPC = 1).
pub type PpR = crate::BitReader;
///Field `PP` writer - PP Port Power (PP)-Read/Write or Read Only. The function of this bit depends on the value of the Port Power Switching (PPC) field in the HCSPARAMS register. The behavior is as follows: PPC PP Operation 0 1b Read Only - Host controller does not have port power control switches. Each port is hard-wired to power. 1 1b/0b - Read/Write. OTG controller requires port power control switches. This bit represents the current setting of the switch (0=off, 1=on). When power is not available on a port (that is, PP equals a 0), the port is non-functional and will not report attaches, detaches, etc. When an over-current condition is detected on a powered port and PPC is a one, the PP bit in each affected port may be transitional by the host controller driver from a one to a zero (removing power from the port). This feature is implemented in all controller cores (PPC = 1).
pub type PpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTC` reader - PTC Port Test Control - Read/Write. Default = 0000b. Refer to Port Test Mode for the operational model for using these test modes and the USB Specification Revision 2.0, Chapter 7 for details on each test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_{HS/FS/LS} values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. NOTE: Low speed operations are not supported as a peripheral device. Any other value than zero indicates that the port is operating in test mode. Value Specific Test 0000 - TEST_MODE_DISABLE 0001 - J_STATE 0010 - K_STATE 0011 - SE0 (host) / NAK (device) 0100 - Packet 0101 - FORCE_ENABLE_HS 0110 - FORCE_ENABLE_FS 0111 - FORCE_ENABLE_LS 1000-1111 - Reserved
pub type PtcR = crate::FieldReader;
///Field `PTC` writer - PTC Port Test Control - Read/Write. Default = 0000b. Refer to Port Test Mode for the operational model for using these test modes and the USB Specification Revision 2.0, Chapter 7 for details on each test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_{HS/FS/LS} values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. NOTE: Low speed operations are not supported as a peripheral device. Any other value than zero indicates that the port is operating in test mode. Value Specific Test 0000 - TEST_MODE_DISABLE 0001 - J_STATE 0010 - K_STATE 0011 - SE0 (host) / NAK (device) 0100 - Packet 0101 - FORCE_ENABLE_HS 0110 - FORCE_ENABLE_FS 0111 - FORCE_ENABLE_LS 1000-1111 - Reserved
pub type PtcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WKCN` reader - WKCN Wake on Connect Enable (WKCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device connects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
pub type WkcnR = crate::BitReader;
///Field `WKCN` writer - WKCN Wake on Connect Enable (WKCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device connects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
pub type WkcnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKDC` reader - WKDC Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device disconnects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
pub type WkdcR = crate::BitReader;
///Field `WKDC` writer - WKDC Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device disconnects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
pub type WkdcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKOC` reader - WKOC Wake on Over-current Enable (WKOC_E) - Read/Write. Default = 0b. Writing this bit to a one enables the port to be sensitive to over-current conditions as wake-up events. This field is zero if Port Power(PORTSC1) is zero.
pub type WkocR = crate::BitReader;
///Field `WKOC` writer - WKOC Wake on Over-current Enable (WKOC_E) - Read/Write. Default = 0b. Writing this bit to a one enables the port to be sensitive to over-current conditions as wake-up events. This field is zero if Port Power(PORTSC1) is zero.
pub type WkocW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHCD` reader - PHCD PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write. Default = 0b. When this bit is set to '1b', the PHY clock is disabled. Reading this bit will indicate the status of the PHY clock. NOTE: The PHY clock cannot be disabled if it is being used as the system clock. In device mode, The PHY can be put into Low Power Suspend when the device is not running (USBCMD Run/Stop=0b) or the host has signalled suspend (PORTSC1 SUSPEND=1b). PHY Low power suspend will be cleared automatically when the host initials resume. Before forcing a resume from the device, the device controller driver must clear this bit. In host mode, the PHY can be put into Low Power Suspend when the downstream device has been put into suspend mode or when no downstream device is connected. Low power suspend is completely under the control of software. 0 - Enable PHY clock 1 - Disable PHY clock
pub type PhcdR = crate::BitReader;
///Field `PHCD` writer - PHCD PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write. Default = 0b. When this bit is set to '1b', the PHY clock is disabled. Reading this bit will indicate the status of the PHY clock. NOTE: The PHY clock cannot be disabled if it is being used as the system clock. In device mode, The PHY can be put into Low Power Suspend when the device is not running (USBCMD Run/Stop=0b) or the host has signalled suspend (PORTSC1 SUSPEND=1b). PHY Low power suspend will be cleared automatically when the host initials resume. Before forcing a resume from the device, the device controller driver must clear this bit. In host mode, the PHY can be put into Low Power Suspend when the downstream device has been put into suspend mode or when no downstream device is connected. Low power suspend is completely under the control of software. 0 - Enable PHY clock 1 - Disable PHY clock
pub type PhcdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PFSC` reader - PFSC Port Force Full Speed Connect - Read/Write. Default = 0b. When this bit is set to '1b', the port will be forced to only connect at Full Speed, It disables the chirp sequence that allows the port to identify itself as High Speed. 0 - Normal operation 1 - Forced to full speed
pub type PfscR = crate::BitReader;
///Field `PFSC` writer - PFSC Port Force Full Speed Connect - Read/Write. Default = 0b. When this bit is set to '1b', the port will be forced to only connect at Full Speed, It disables the chirp sequence that allows the port to identify itself as High Speed. 0 - Normal operation 1 - Forced to full speed
pub type PfscW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSPD` reader - PSPD Port Speed - Read Only. This register field indicates the speed at which the port is operating. 00 - Full Speed 01 - Low Speed 10 - High Speed 11 - Undefined
pub type PspdR = crate::FieldReader;
///Field `PTW` reader - PTW Parallel Transceiver Width This bit has no effect if serial interface engine is used. 0 - Select the 8-bit UTMI interface \[60MHz\] 1 - Select the 16-bit UTMI interface \[30MHz\]
pub type PtwR = crate::BitReader;
///Field `PTW` writer - PTW Parallel Transceiver Width This bit has no effect if serial interface engine is used. 0 - Select the 8-bit UTMI interface \[60MHz\] 1 - Select the 16-bit UTMI interface \[30MHz\]
pub type PtwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STS` reader - STS Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals. When this bit is set '1b', serial interface engine will be used instead of parallel interface signals.
pub type StsR = crate::BitReader;
///Field `STS` writer - STS Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals. When this bit is set '1b', serial interface engine will be used instead of parallel interface signals.
pub type StsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CCS Current Connect Status-Read Only. In Host Mode: 1=Device is present on port. 0=No device is present. Default = 0. This value reflects the current state of the port, and may not correspond directly to the event that caused the Connect Status Change bit (Bit 1) to be set. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: 1=Attached. 0=Not Attached. Default=0. A one indicates that the device successfully attached and is operating in either high speed or full speed as indicated by the High Speed Port bit in this register. A zero indicates that the device did not attach successfully or was forcibly disconnected by the software writing a zero to the Run bit in the USBCMD register. It does not state the device being disconnected or Suspended.
    #[inline(always)]
    pub fn ccs(&self) -> CcsR {
        CcsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CSC Connect Status Change-R/WC. 1 =Change in Current Connect Status. 0=No change. Default 0. In Host Mode: Indicates a change has occurred in the port's Current Connect Status. The host/device controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit (that is, the bit will remain set). Software clears this bit by writing a one to it. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: This bit is undefined in device controller mode.
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PE Port Enabled/Disabled-Read/Write. 1=Enable. 0=Disable. Default 0. In Host Mode: Ports can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. Ports can be disabled by either a fault condition (disconnect event or other fault condition) or by the host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events. When the port is disabled, (0b) downstream propagation of data is blocked except for reset. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: The device port is always enabled, so this bit is always '1b'.
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PEC Port Enable/Disable Change-R/WC. 1=Port enabled/disabled status has changed. 0=No change. Default = 0. In Host Mode: For the root hub, this bit is set to a one only when a port is disabled due to disconnect on the port or due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification). Software clears this by writing a one to it. This field is zero if Port Power(PORTSC1) is zero. In Device mode: The device port is always enabled, so this bit is always '0b'.
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OCA Over-current Active-Read Only. Default 0. This bit will automatically transition from one to zero when the over current condition is removed. 0 - This port does not have an over-current condition. 1 - This port currently has an over-current condition
    #[inline(always)]
    pub fn oca(&self) -> OcaR {
        OcaR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OCC Over-current Change-R/WC. Default=0. This bit is set '1b' by hardware when there is a change to Over-current Active. Software can clear this bit by writing a one to this bit position.
    #[inline(always)]
    pub fn occ(&self) -> OccR {
        OccR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FPR Force Port Resume -Read/Write. 1= Resume detected/driven on port. 0=No resume (K-state) detected driven on port. Default = 0. In Host Mode: Software sets this bit to one to drive resume signaling. The Host Controller sets this bit to one if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to one. This bit will automatically change to zero after the resume sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the resume duration is timed in the driver. Note that when the Host controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this bit remains a one. This bit will remain a one until the port has switched to the high-speed idle. Writing a zero has no effect because the port controller will time the resume operation, clear the bit the port control state switches to HS or FS idle. This field is zero if Port Power(PORTSC1) is zero in host mode. This bit is not-EHCI compatible. In Device mode: After the device has been in Suspend State for 5ms or more, software must set this bit to one to drive resume signaling before clearing. The Device Controller will set this bit to one if a J-to-K transition is detected while the port is in the Suspend state. The bit will be cleared when the device returns to normal operation. Also, when this bit wil be cleared because a K-to-J transition detected, the Port Change Detect bit in the USBSTS register is also set to one.
    #[inline(always)]
    pub fn fpr(&self) -> FprR {
        FprR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SUSP Suspend - Read/Write or Read Only. Default = 0b. 1=Port in suspend state. 0=Port not in suspend state. In Host Mode: Read/Write. Port Enabled Bit and Suspend bit of this register define the port states as follows: Bits \[Port Enabled, Suspend\] Port State 0x Disable 10 Enable 11 Suspend When in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Note that the bit status does not change until the port is suspended and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB. The host controller will unconditionally set this bit to zero when software sets the Force Port Resume bit to zero. The host controller ignores a write of zero to this bit. If host software sets this bit to a one when the port is not enabled (that is, Port enabled bit is a zero) the results are undefined. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: Read Only. In device mode this bit is a read only status bit.
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PR Port Reset - Read/Write or Read Only. Default = 0b. In Host Mode: Read/Write. 1=Port is in Reset. 0=Port is not in Reset. Default 0. When software writes a one to this bit the bus-reset sequence as defined in the USB Specification Revision 2.0 is started. This bit will automatically change to zero after the reset sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the reset duration is timed in the driver. In Device Mode: This bit is a read only status bit. Device reset from the USB bus is also indicated in the USBSTS register.
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSP High-Speed Port - Read Only. Default = 0b. When the bit is one, the host/device connected to the port is in high-speed mode and if set to zero, the host/device connected to the port is not in a high-speed mode. NOTE: HSP is redundant with PSPD(bit 27, 26) but remained for compatibility.
    #[inline(always)]
    pub fn hsp(&self) -> HspR {
        HspR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - LS Line Status-Read Only. These bits reflect the current logical levels of the D+ (bit 11) and D- (bit 10) signal lines. In host mode, the use of linestate by the host controller driver is not necessary (unlike EHCI), because the port controller state machine and the port routing manage the connection of LS and FS. In device mode, the use of linestate by the device controller driver is not necessary. The encoding of the bits are: Bits \[11:10\] Meaning 00 - SE0 01 - K-state 10 - J-state 11 - Undefined
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - PP Port Power (PP)-Read/Write or Read Only. The function of this bit depends on the value of the Port Power Switching (PPC) field in the HCSPARAMS register. The behavior is as follows: PPC PP Operation 0 1b Read Only - Host controller does not have port power control switches. Each port is hard-wired to power. 1 1b/0b - Read/Write. OTG controller requires port power control switches. This bit represents the current setting of the switch (0=off, 1=on). When power is not available on a port (that is, PP equals a 0), the port is non-functional and will not report attaches, detaches, etc. When an over-current condition is detected on a powered port and PPC is a one, the PP bit in each affected port may be transitional by the host controller driver from a one to a zero (removing power from the port). This feature is implemented in all controller cores (PPC = 1).
    #[inline(always)]
    pub fn pp(&self) -> PpR {
        PpR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:19 - PTC Port Test Control - Read/Write. Default = 0000b. Refer to Port Test Mode for the operational model for using these test modes and the USB Specification Revision 2.0, Chapter 7 for details on each test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_{HS/FS/LS} values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. NOTE: Low speed operations are not supported as a peripheral device. Any other value than zero indicates that the port is operating in test mode. Value Specific Test 0000 - TEST_MODE_DISABLE 0001 - J_STATE 0010 - K_STATE 0011 - SE0 (host) / NAK (device) 0100 - Packet 0101 - FORCE_ENABLE_HS 0110 - FORCE_ENABLE_FS 0111 - FORCE_ENABLE_LS 1000-1111 - Reserved
    #[inline(always)]
    pub fn ptc(&self) -> PtcR {
        PtcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - WKCN Wake on Connect Enable (WKCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device connects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
    #[inline(always)]
    pub fn wkcn(&self) -> WkcnR {
        WkcnR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - WKDC Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device disconnects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
    #[inline(always)]
    pub fn wkdc(&self) -> WkdcR {
        WkdcR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - WKOC Wake on Over-current Enable (WKOC_E) - Read/Write. Default = 0b. Writing this bit to a one enables the port to be sensitive to over-current conditions as wake-up events. This field is zero if Port Power(PORTSC1) is zero.
    #[inline(always)]
    pub fn wkoc(&self) -> WkocR {
        WkocR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PHCD PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write. Default = 0b. When this bit is set to '1b', the PHY clock is disabled. Reading this bit will indicate the status of the PHY clock. NOTE: The PHY clock cannot be disabled if it is being used as the system clock. In device mode, The PHY can be put into Low Power Suspend when the device is not running (USBCMD Run/Stop=0b) or the host has signalled suspend (PORTSC1 SUSPEND=1b). PHY Low power suspend will be cleared automatically when the host initials resume. Before forcing a resume from the device, the device controller driver must clear this bit. In host mode, the PHY can be put into Low Power Suspend when the downstream device has been put into suspend mode or when no downstream device is connected. Low power suspend is completely under the control of software. 0 - Enable PHY clock 1 - Disable PHY clock
    #[inline(always)]
    pub fn phcd(&self) -> PhcdR {
        PhcdR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - PFSC Port Force Full Speed Connect - Read/Write. Default = 0b. When this bit is set to '1b', the port will be forced to only connect at Full Speed, It disables the chirp sequence that allows the port to identify itself as High Speed. 0 - Normal operation 1 - Forced to full speed
    #[inline(always)]
    pub fn pfsc(&self) -> PfscR {
        PfscR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 26:27 - PSPD Port Speed - Read Only. This register field indicates the speed at which the port is operating. 00 - Full Speed 01 - Low Speed 10 - High Speed 11 - Undefined
    #[inline(always)]
    pub fn pspd(&self) -> PspdR {
        PspdR::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bit 28 - PTW Parallel Transceiver Width This bit has no effect if serial interface engine is used. 0 - Select the 8-bit UTMI interface \[60MHz\] 1 - Select the 16-bit UTMI interface \[30MHz\]
    #[inline(always)]
    pub fn ptw(&self) -> PtwR {
        PtwR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - STS Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals. When this bit is set '1b', serial interface engine will be used instead of parallel interface signals.
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CCS Current Connect Status-Read Only. In Host Mode: 1=Device is present on port. 0=No device is present. Default = 0. This value reflects the current state of the port, and may not correspond directly to the event that caused the Connect Status Change bit (Bit 1) to be set. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: 1=Attached. 0=Not Attached. Default=0. A one indicates that the device successfully attached and is operating in either high speed or full speed as indicated by the High Speed Port bit in this register. A zero indicates that the device did not attach successfully or was forcibly disconnected by the software writing a zero to the Run bit in the USBCMD register. It does not state the device being disconnected or Suspended.
    #[inline(always)]
    pub fn ccs(&mut self) -> CcsW<'_, Portsc1Spec> {
        CcsW::new(self, 0)
    }
    ///Bit 1 - CSC Connect Status Change-R/WC. 1 =Change in Current Connect Status. 0=No change. Default 0. In Host Mode: Indicates a change has occurred in the port's Current Connect Status. The host/device controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be 'setting' an already-set bit (that is, the bit will remain set). Software clears this bit by writing a one to it. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: This bit is undefined in device controller mode.
    #[inline(always)]
    pub fn csc(&mut self) -> CscW<'_, Portsc1Spec> {
        CscW::new(self, 1)
    }
    ///Bit 2 - PE Port Enabled/Disabled-Read/Write. 1=Enable. 0=Disable. Default 0. In Host Mode: Ports can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. Ports can be disabled by either a fault condition (disconnect event or other fault condition) or by the host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events. When the port is disabled, (0b) downstream propagation of data is blocked except for reset. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: The device port is always enabled, so this bit is always '1b'.
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, Portsc1Spec> {
        PeW::new(self, 2)
    }
    ///Bit 3 - PEC Port Enable/Disable Change-R/WC. 1=Port enabled/disabled status has changed. 0=No change. Default = 0. In Host Mode: For the root hub, this bit is set to a one only when a port is disabled due to disconnect on the port or due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification). Software clears this by writing a one to it. This field is zero if Port Power(PORTSC1) is zero. In Device mode: The device port is always enabled, so this bit is always '0b'.
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<'_, Portsc1Spec> {
        PecW::new(self, 3)
    }
    ///Bit 5 - OCC Over-current Change-R/WC. Default=0. This bit is set '1b' by hardware when there is a change to Over-current Active. Software can clear this bit by writing a one to this bit position.
    #[inline(always)]
    pub fn occ(&mut self) -> OccW<'_, Portsc1Spec> {
        OccW::new(self, 5)
    }
    ///Bit 6 - FPR Force Port Resume -Read/Write. 1= Resume detected/driven on port. 0=No resume (K-state) detected driven on port. Default = 0. In Host Mode: Software sets this bit to one to drive resume signaling. The Host Controller sets this bit to one if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to one. This bit will automatically change to zero after the resume sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the resume duration is timed in the driver. Note that when the Host controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed 'K') is driven on the port as long as this bit remains a one. This bit will remain a one until the port has switched to the high-speed idle. Writing a zero has no effect because the port controller will time the resume operation, clear the bit the port control state switches to HS or FS idle. This field is zero if Port Power(PORTSC1) is zero in host mode. This bit is not-EHCI compatible. In Device mode: After the device has been in Suspend State for 5ms or more, software must set this bit to one to drive resume signaling before clearing. The Device Controller will set this bit to one if a J-to-K transition is detected while the port is in the Suspend state. The bit will be cleared when the device returns to normal operation. Also, when this bit wil be cleared because a K-to-J transition detected, the Port Change Detect bit in the USBSTS register is also set to one.
    #[inline(always)]
    pub fn fpr(&mut self) -> FprW<'_, Portsc1Spec> {
        FprW::new(self, 6)
    }
    ///Bit 7 - SUSP Suspend - Read/Write or Read Only. Default = 0b. 1=Port in suspend state. 0=Port not in suspend state. In Host Mode: Read/Write. Port Enabled Bit and Suspend bit of this register define the port states as follows: Bits \[Port Enabled, Suspend\] Port State 0x Disable 10 Enable 11 Suspend When in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Note that the bit status does not change until the port is suspended and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB. The host controller will unconditionally set this bit to zero when software sets the Force Port Resume bit to zero. The host controller ignores a write of zero to this bit. If host software sets this bit to a one when the port is not enabled (that is, Port enabled bit is a zero) the results are undefined. This field is zero if Port Power(PORTSC1) is zero in host mode. In Device Mode: Read Only. In device mode this bit is a read only status bit.
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<'_, Portsc1Spec> {
        SuspW::new(self, 7)
    }
    ///Bit 8 - PR Port Reset - Read/Write or Read Only. Default = 0b. In Host Mode: Read/Write. 1=Port is in Reset. 0=Port is not in Reset. Default 0. When software writes a one to this bit the bus-reset sequence as defined in the USB Specification Revision 2.0 is started. This bit will automatically change to zero after the reset sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the reset duration is timed in the driver. In Device Mode: This bit is a read only status bit. Device reset from the USB bus is also indicated in the USBSTS register.
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, Portsc1Spec> {
        PrW::new(self, 8)
    }
    ///Bit 12 - PP Port Power (PP)-Read/Write or Read Only. The function of this bit depends on the value of the Port Power Switching (PPC) field in the HCSPARAMS register. The behavior is as follows: PPC PP Operation 0 1b Read Only - Host controller does not have port power control switches. Each port is hard-wired to power. 1 1b/0b - Read/Write. OTG controller requires port power control switches. This bit represents the current setting of the switch (0=off, 1=on). When power is not available on a port (that is, PP equals a 0), the port is non-functional and will not report attaches, detaches, etc. When an over-current condition is detected on a powered port and PPC is a one, the PP bit in each affected port may be transitional by the host controller driver from a one to a zero (removing power from the port). This feature is implemented in all controller cores (PPC = 1).
    #[inline(always)]
    pub fn pp(&mut self) -> PpW<'_, Portsc1Spec> {
        PpW::new(self, 12)
    }
    ///Bits 16:19 - PTC Port Test Control - Read/Write. Default = 0000b. Refer to Port Test Mode for the operational model for using these test modes and the USB Specification Revision 2.0, Chapter 7 for details on each test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_{HS/FS/LS} values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. NOTE: Low speed operations are not supported as a peripheral device. Any other value than zero indicates that the port is operating in test mode. Value Specific Test 0000 - TEST_MODE_DISABLE 0001 - J_STATE 0010 - K_STATE 0011 - SE0 (host) / NAK (device) 0100 - Packet 0101 - FORCE_ENABLE_HS 0110 - FORCE_ENABLE_FS 0111 - FORCE_ENABLE_LS 1000-1111 - Reserved
    #[inline(always)]
    pub fn ptc(&mut self) -> PtcW<'_, Portsc1Spec> {
        PtcW::new(self, 16)
    }
    ///Bit 20 - WKCN Wake on Connect Enable (WKCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device connects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
    #[inline(always)]
    pub fn wkcn(&mut self) -> WkcnW<'_, Portsc1Spec> {
        WkcnW::new(self, 20)
    }
    ///Bit 21 - WKDC Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write. Default=0b. Writing this bit to a one enables the port to be sensitive to device disconnects as wake-up events. This field is zero if Port Power(PORTSC1) is zero or in device mode.
    #[inline(always)]
    pub fn wkdc(&mut self) -> WkdcW<'_, Portsc1Spec> {
        WkdcW::new(self, 21)
    }
    ///Bit 22 - WKOC Wake on Over-current Enable (WKOC_E) - Read/Write. Default = 0b. Writing this bit to a one enables the port to be sensitive to over-current conditions as wake-up events. This field is zero if Port Power(PORTSC1) is zero.
    #[inline(always)]
    pub fn wkoc(&mut self) -> WkocW<'_, Portsc1Spec> {
        WkocW::new(self, 22)
    }
    ///Bit 23 - PHCD PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write. Default = 0b. When this bit is set to '1b', the PHY clock is disabled. Reading this bit will indicate the status of the PHY clock. NOTE: The PHY clock cannot be disabled if it is being used as the system clock. In device mode, The PHY can be put into Low Power Suspend when the device is not running (USBCMD Run/Stop=0b) or the host has signalled suspend (PORTSC1 SUSPEND=1b). PHY Low power suspend will be cleared automatically when the host initials resume. Before forcing a resume from the device, the device controller driver must clear this bit. In host mode, the PHY can be put into Low Power Suspend when the downstream device has been put into suspend mode or when no downstream device is connected. Low power suspend is completely under the control of software. 0 - Enable PHY clock 1 - Disable PHY clock
    #[inline(always)]
    pub fn phcd(&mut self) -> PhcdW<'_, Portsc1Spec> {
        PhcdW::new(self, 23)
    }
    ///Bit 24 - PFSC Port Force Full Speed Connect - Read/Write. Default = 0b. When this bit is set to '1b', the port will be forced to only connect at Full Speed, It disables the chirp sequence that allows the port to identify itself as High Speed. 0 - Normal operation 1 - Forced to full speed
    #[inline(always)]
    pub fn pfsc(&mut self) -> PfscW<'_, Portsc1Spec> {
        PfscW::new(self, 24)
    }
    ///Bit 28 - PTW Parallel Transceiver Width This bit has no effect if serial interface engine is used. 0 - Select the 8-bit UTMI interface \[60MHz\] 1 - Select the 16-bit UTMI interface \[30MHz\]
    #[inline(always)]
    pub fn ptw(&mut self) -> PtwW<'_, Portsc1Spec> {
        PtwW::new(self, 28)
    }
    ///Bit 29 - STS Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals. When this bit is set '1b', serial interface engine will be used instead of parallel interface signals.
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<'_, Portsc1Spec> {
        StsW::new(self, 29)
    }
}
/**Port Status & Control

You can [`read`](crate::Reg::read) this register and get [`portsc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portsc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Portsc1Spec;
impl crate::RegisterSpec for Portsc1Spec {
    type Ux = u32;
}
///`read()` method returns [`portsc1::R`](R) reader structure
impl crate::Readable for Portsc1Spec {}
///`write(|w| ..)` method takes [`portsc1::W`](W) writer structure
impl crate::Writable for Portsc1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PORTSC1 to value 0
impl crate::Resettable for Portsc1Spec {}
