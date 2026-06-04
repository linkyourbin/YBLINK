///Register `USBCMD` reader
pub type R = crate::R<UsbcmdSpec>;
///Register `USBCMD` writer
pub type W = crate::W<UsbcmdSpec>;
///Field `RS` reader - RS Run/Stop (RS) - Read/Write. Default 0b. 1=Run. 0=Stop. Host operation mode: When set to '1b', the Controller proceeds with the execution of the schedule. The Controller continues execution as long as this bit is set to a one. When this bit is set to 0, the Host Controller completes the current transaction on the USB and then halts. The HC Halted bit in the status register indicates when the Controller has finished the transaction and has entered the stopped state. Software should not write a one to this field unless the controller is in the Halted state (that is, HCHalted in the USBSTS register is a one). Device operation mode: Writing a one to this bit will cause the controller to enable a pull-up on D+ and initiate an attach event. This control bit is not directly connected to the pull-up enable, as the pull-up will become disabled upon transitioning into high-speed mode. Software should use this bit to prevent an attach event before the controller has been properly initialized. Writing a 0 to this will cause a detach event.
pub type RsR = crate::BitReader;
///Field `RS` writer - RS Run/Stop (RS) - Read/Write. Default 0b. 1=Run. 0=Stop. Host operation mode: When set to '1b', the Controller proceeds with the execution of the schedule. The Controller continues execution as long as this bit is set to a one. When this bit is set to 0, the Host Controller completes the current transaction on the USB and then halts. The HC Halted bit in the status register indicates when the Controller has finished the transaction and has entered the stopped state. Software should not write a one to this field unless the controller is in the Halted state (that is, HCHalted in the USBSTS register is a one). Device operation mode: Writing a one to this bit will cause the controller to enable a pull-up on D+ and initiate an attach event. This control bit is not directly connected to the pull-up enable, as the pull-up will become disabled upon transitioning into high-speed mode. Software should use this bit to prevent an attach event before the controller has been properly initialized. Writing a 0 to this will cause a detach event.
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST` reader - RST Controller Reset (RESET) - Read/Write. Software uses this bit to reset the controller. This bit is set to zero by the Host/Device Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register. Host operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports. Software should not set this bit to a one when the HCHalted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior. Device operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Writing a one to this bit when the device is in the attached state is not recommended, because the effect on an attached host is undefined. In order to ensure that the device is not in an attached state before initiating a device controller reset, all primed endpoints should be flushed and the USBCMD Run/Stop bit should be set to 0.
pub type RstR = crate::BitReader;
///Field `RST` writer - RST Controller Reset (RESET) - Read/Write. Software uses this bit to reset the controller. This bit is set to zero by the Host/Device Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register. Host operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports. Software should not set this bit to a one when the HCHalted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior. Device operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Writing a one to this bit when the device is in the attached state is not recommended, because the effect on an attached host is undefined. In order to ensure that the device is not in an attached state before initiating a device controller reset, all primed endpoints should be flushed and the USBCMD Run/Stop bit should be set to 0.
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FS_1` reader - FS_1 See description at bit 15
pub type Fs1R = crate::FieldReader;
///Field `FS_1` writer - FS_1 See description at bit 15
pub type Fs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PSE` reader - PSE Periodic Schedule Enable- Read/Write. Default 0b. This bit controls whether the host controller skips processing the Periodic Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Periodic Schedule 1 - Use the PERIODICLISTBASE register to access the Periodic Schedule.
pub type PseR = crate::BitReader;
///Field `PSE` writer - PSE Periodic Schedule Enable- Read/Write. Default 0b. This bit controls whether the host controller skips processing the Periodic Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Periodic Schedule 1 - Use the PERIODICLISTBASE register to access the Periodic Schedule.
pub type PseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASE` reader - ASE Asynchronous Schedule Enable - Read/Write. Default 0b. This bit controls whether the host controller skips processing the Asynchronous Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Asynchronous Schedule. 1 - Use the ASYNCLISTADDR register to access the Asynchronous Schedule.
pub type AseR = crate::BitReader;
///Field `ASE` writer - ASE Asynchronous Schedule Enable - Read/Write. Default 0b. This bit controls whether the host controller skips processing the Asynchronous Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Asynchronous Schedule. 1 - Use the ASYNCLISTADDR register to access the Asynchronous Schedule.
pub type AseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAA` reader - IAA Interrupt on Async Advance Doorbell - Read/Write. This bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule states, it sets the Interrupt on Async Advance status bit in the USBSTS register. If the Interrupt on Sync Advance Enable bit in the USBINTR register is one, then the host controller will assert an interrupt at the next interrupt threshold. The host controller sets this bit to zero after it has set the Interrupt on Sync Advance status bit in the USBSTS register to one. Software should not write a one to this bit when the asynchronous schedule is inactive. Doing so will yield undefined results. This bit is only used in host mode. Writing a one to this bit when device mode is selected will have undefined results.
pub type IaaR = crate::BitReader;
///Field `IAA` writer - IAA Interrupt on Async Advance Doorbell - Read/Write. This bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule states, it sets the Interrupt on Async Advance status bit in the USBSTS register. If the Interrupt on Sync Advance Enable bit in the USBINTR register is one, then the host controller will assert an interrupt at the next interrupt threshold. The host controller sets this bit to zero after it has set the Interrupt on Sync Advance status bit in the USBSTS register to one. Software should not write a one to this bit when the asynchronous schedule is inactive. Doing so will yield undefined results. This bit is only used in host mode. Writing a one to this bit when device mode is selected will have undefined results.
pub type IaaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASP` reader - ASP Asynchronous Schedule Park Mode Count - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this field defaults to 3h and is R/W. Otherwise it defaults to zero and is Read-Only. It contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid values are 1h to 3h. Software must not write a zero to this bit when Park Mode Enable is a one as this will result in undefined behavior. This field is set to 3h in all controller core.
pub type AspR = crate::FieldReader;
///Field `ASP` writer - ASP Asynchronous Schedule Park Mode Count - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this field defaults to 3h and is R/W. Otherwise it defaults to zero and is Read-Only. It contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid values are 1h to 3h. Software must not write a zero to this bit when Park Mode Enable is a one as this will result in undefined behavior. This field is set to 3h in all controller core.
pub type AspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ASPE` reader - ASPE Asynchronous Schedule Park Mode Enable - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this bit defaults to a 1h and is R/W. Otherwise the bit must be a zero and is RO. Software uses this bit to enable or disable Park mode. When this bit is one, Park mode is enabled. When this bit is a zero, Park mode is disabled. NOTE: ASPE bit reset value: '0b' for OTG controller .
pub type AspeR = crate::BitReader;
///Field `ASPE` writer - ASPE Asynchronous Schedule Park Mode Enable - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this bit defaults to a 1h and is R/W. Otherwise the bit must be a zero and is RO. Software uses this bit to enable or disable Park mode. When this bit is one, Park mode is enabled. When this bit is a zero, Park mode is disabled. NOTE: ASPE bit reset value: '0b' for OTG controller .
pub type AspeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRM` writer - Asynchronous Schedule start- Write only， host mode only。 this bit is used to notify hostcontroller to start async schedule immediately.
pub type PrmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUTW` reader - SUTW Setup TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure that the setup data payload of 8 bytes is extracted from a QH by the DCD without being corrupted. If the setup lockout mode is off (SLOM bit in USB core register n_USBMODE, see USBMODE ) then there is a hazard when new setup data arrives while the DCD is copying the setup data payload from the QH for a previous setup packet. This bit is set and cleared by software. This bit would also be cleared by hardware when a hazard detected.
pub type SutwR = crate::BitReader;
///Field `SUTW` writer - SUTW Setup TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure that the setup data payload of 8 bytes is extracted from a QH by the DCD without being corrupted. If the setup lockout mode is off (SLOM bit in USB core register n_USBMODE, see USBMODE ) then there is a hazard when new setup data arrives while the DCD is copying the setup data payload from the QH for a previous setup packet. This bit is set and cleared by software. This bit would also be cleared by hardware when a hazard detected.
pub type SutwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATDTW` reader - ATDTW Add dTD TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure proper addition of a new dTD to an active (primed) endpoint's linked list. This bit is set and cleared by software. This bit would also be cleared by hardware when state machine is hazard region for which adding a dTD to a primed endpoint may go unrecognized.
pub type AtdtwR = crate::BitReader;
///Field `ATDTW` writer - ATDTW Add dTD TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure proper addition of a new dTD to an active (primed) endpoint's linked list. This bit is set and cleared by software. This bit would also be cleared by hardware when state machine is hazard region for which adding a dTD to a primed endpoint may go unrecognized.
pub type AtdtwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FS_2` reader - FS_2 Frame List Size - (Read/Write or Read Only). \[host mode only\] This field is Read/Write only if Programmable Frame List Flag in the HCCPARAMS registers is set to one. This field specifies the size of the frame list that controls which bits in the Frame Index Register should be used for the Frame List Current index. NOTE: This field is made up from USBCMD bits 15, 3 and 2. Value Meaning 0b000 - 1024 elements (4096 bytes) Default value 0b001 - 512 elements (2048 bytes) 0b010 - 256 elements (1024 bytes) 0b011 - 128 elements (512 bytes) 0b100 - 64 elements (256 bytes) 0b101 - 32 elements (128 bytes) 0b110 - 16 elements (64 bytes) 0b111 - 8 elements (32 bytes)
pub type Fs2R = crate::BitReader;
///Field `FS_2` writer - FS_2 Frame List Size - (Read/Write or Read Only). \[host mode only\] This field is Read/Write only if Programmable Frame List Flag in the HCCPARAMS registers is set to one. This field specifies the size of the frame list that controls which bits in the Frame Index Register should be used for the Frame List Current index. NOTE: This field is made up from USBCMD bits 15, 3 and 2. Value Meaning 0b000 - 1024 elements (4096 bytes) Default value 0b001 - 512 elements (2048 bytes) 0b010 - 256 elements (1024 bytes) 0b011 - 128 elements (512 bytes) 0b100 - 64 elements (256 bytes) 0b101 - 32 elements (128 bytes) 0b110 - 16 elements (64 bytes) 0b111 - 8 elements (32 bytes)
pub type Fs2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITC` reader - ITC Interrupt Threshold Control -Read/Write. The system software uses this field to set the maximum rate at which the host/device controller will issue interrupts. ITC contains the maximum interrupt interval measured in micro-frames. Valid values are shown below. Value Maximum Interrupt Interval 00000000 - Immediate (no threshold) 00000001 - 1 micro-frame 00000010 - 2 micro-frames 00000100 - 4 micro-frames 00001000 - 8 micro-frames 00010000 - 16 micro-frames 00100000 - 32 micro-frames 01000000 - 64 micro-frames
pub type ItcR = crate::FieldReader;
///Field `ITC` writer - ITC Interrupt Threshold Control -Read/Write. The system software uses this field to set the maximum rate at which the host/device controller will issue interrupts. ITC contains the maximum interrupt interval measured in micro-frames. Valid values are shown below. Value Maximum Interrupt Interval 00000000 - Immediate (no threshold) 00000001 - 1 micro-frame 00000010 - 2 micro-frames 00000100 - 4 micro-frames 00001000 - 8 micro-frames 00010000 - 16 micro-frames 00100000 - 32 micro-frames 01000000 - 64 micro-frames
pub type ItcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - RS Run/Stop (RS) - Read/Write. Default 0b. 1=Run. 0=Stop. Host operation mode: When set to '1b', the Controller proceeds with the execution of the schedule. The Controller continues execution as long as this bit is set to a one. When this bit is set to 0, the Host Controller completes the current transaction on the USB and then halts. The HC Halted bit in the status register indicates when the Controller has finished the transaction and has entered the stopped state. Software should not write a one to this field unless the controller is in the Halted state (that is, HCHalted in the USBSTS register is a one). Device operation mode: Writing a one to this bit will cause the controller to enable a pull-up on D+ and initiate an attach event. This control bit is not directly connected to the pull-up enable, as the pull-up will become disabled upon transitioning into high-speed mode. Software should use this bit to prevent an attach event before the controller has been properly initialized. Writing a 0 to this will cause a detach event.
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RST Controller Reset (RESET) - Read/Write. Software uses this bit to reset the controller. This bit is set to zero by the Host/Device Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register. Host operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports. Software should not set this bit to a one when the HCHalted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior. Device operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Writing a one to this bit when the device is in the attached state is not recommended, because the effect on an attached host is undefined. In order to ensure that the device is not in an attached state before initiating a device controller reset, all primed endpoints should be flushed and the USBCMD Run/Stop bit should be set to 0.
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - FS_1 See description at bit 15
    #[inline(always)]
    pub fn fs_1(&self) -> Fs1R {
        Fs1R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PSE Periodic Schedule Enable- Read/Write. Default 0b. This bit controls whether the host controller skips processing the Periodic Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Periodic Schedule 1 - Use the PERIODICLISTBASE register to access the Periodic Schedule.
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ASE Asynchronous Schedule Enable - Read/Write. Default 0b. This bit controls whether the host controller skips processing the Asynchronous Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Asynchronous Schedule. 1 - Use the ASYNCLISTADDR register to access the Asynchronous Schedule.
    #[inline(always)]
    pub fn ase(&self) -> AseR {
        AseR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IAA Interrupt on Async Advance Doorbell - Read/Write. This bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule states, it sets the Interrupt on Async Advance status bit in the USBSTS register. If the Interrupt on Sync Advance Enable bit in the USBINTR register is one, then the host controller will assert an interrupt at the next interrupt threshold. The host controller sets this bit to zero after it has set the Interrupt on Sync Advance status bit in the USBSTS register to one. Software should not write a one to this bit when the asynchronous schedule is inactive. Doing so will yield undefined results. This bit is only used in host mode. Writing a one to this bit when device mode is selected will have undefined results.
    #[inline(always)]
    pub fn iaa(&self) -> IaaR {
        IaaR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:9 - ASP Asynchronous Schedule Park Mode Count - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this field defaults to 3h and is R/W. Otherwise it defaults to zero and is Read-Only. It contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid values are 1h to 3h. Software must not write a zero to this bit when Park Mode Enable is a one as this will result in undefined behavior. This field is set to 3h in all controller core.
    #[inline(always)]
    pub fn asp(&self) -> AspR {
        AspR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - ASPE Asynchronous Schedule Park Mode Enable - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this bit defaults to a 1h and is R/W. Otherwise the bit must be a zero and is RO. Software uses this bit to enable or disable Park mode. When this bit is one, Park mode is enabled. When this bit is a zero, Park mode is disabled. NOTE: ASPE bit reset value: '0b' for OTG controller .
    #[inline(always)]
    pub fn aspe(&self) -> AspeR {
        AspeR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - SUTW Setup TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure that the setup data payload of 8 bytes is extracted from a QH by the DCD without being corrupted. If the setup lockout mode is off (SLOM bit in USB core register n_USBMODE, see USBMODE ) then there is a hazard when new setup data arrives while the DCD is copying the setup data payload from the QH for a previous setup packet. This bit is set and cleared by software. This bit would also be cleared by hardware when a hazard detected.
    #[inline(always)]
    pub fn sutw(&self) -> SutwR {
        SutwR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ATDTW Add dTD TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure proper addition of a new dTD to an active (primed) endpoint's linked list. This bit is set and cleared by software. This bit would also be cleared by hardware when state machine is hazard region for which adding a dTD to a primed endpoint may go unrecognized.
    #[inline(always)]
    pub fn atdtw(&self) -> AtdtwR {
        AtdtwR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - FS_2 Frame List Size - (Read/Write or Read Only). \[host mode only\] This field is Read/Write only if Programmable Frame List Flag in the HCCPARAMS registers is set to one. This field specifies the size of the frame list that controls which bits in the Frame Index Register should be used for the Frame List Current index. NOTE: This field is made up from USBCMD bits 15, 3 and 2. Value Meaning 0b000 - 1024 elements (4096 bytes) Default value 0b001 - 512 elements (2048 bytes) 0b010 - 256 elements (1024 bytes) 0b011 - 128 elements (512 bytes) 0b100 - 64 elements (256 bytes) 0b101 - 32 elements (128 bytes) 0b110 - 16 elements (64 bytes) 0b111 - 8 elements (32 bytes)
    #[inline(always)]
    pub fn fs_2(&self) -> Fs2R {
        Fs2R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - ITC Interrupt Threshold Control -Read/Write. The system software uses this field to set the maximum rate at which the host/device controller will issue interrupts. ITC contains the maximum interrupt interval measured in micro-frames. Valid values are shown below. Value Maximum Interrupt Interval 00000000 - Immediate (no threshold) 00000001 - 1 micro-frame 00000010 - 2 micro-frames 00000100 - 4 micro-frames 00001000 - 8 micro-frames 00010000 - 16 micro-frames 00100000 - 32 micro-frames 01000000 - 64 micro-frames
    #[inline(always)]
    pub fn itc(&self) -> ItcR {
        ItcR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - RS Run/Stop (RS) - Read/Write. Default 0b. 1=Run. 0=Stop. Host operation mode: When set to '1b', the Controller proceeds with the execution of the schedule. The Controller continues execution as long as this bit is set to a one. When this bit is set to 0, the Host Controller completes the current transaction on the USB and then halts. The HC Halted bit in the status register indicates when the Controller has finished the transaction and has entered the stopped state. Software should not write a one to this field unless the controller is in the Halted state (that is, HCHalted in the USBSTS register is a one). Device operation mode: Writing a one to this bit will cause the controller to enable a pull-up on D+ and initiate an attach event. This control bit is not directly connected to the pull-up enable, as the pull-up will become disabled upon transitioning into high-speed mode. Software should use this bit to prevent an attach event before the controller has been properly initialized. Writing a 0 to this will cause a detach event.
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<'_, UsbcmdSpec> {
        RsW::new(self, 0)
    }
    ///Bit 1 - RST Controller Reset (RESET) - Read/Write. Software uses this bit to reset the controller. This bit is set to zero by the Host/Device Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register. Host operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports. Software should not set this bit to a one when the HCHalted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior. Device operation mode: When software writes a one to this bit, the Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Writing a one to this bit when the device is in the attached state is not recommended, because the effect on an attached host is undefined. In order to ensure that the device is not in an attached state before initiating a device controller reset, all primed endpoints should be flushed and the USBCMD Run/Stop bit should be set to 0.
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<'_, UsbcmdSpec> {
        RstW::new(self, 1)
    }
    ///Bits 2:3 - FS_1 See description at bit 15
    #[inline(always)]
    pub fn fs_1(&mut self) -> Fs1W<'_, UsbcmdSpec> {
        Fs1W::new(self, 2)
    }
    ///Bit 4 - PSE Periodic Schedule Enable- Read/Write. Default 0b. This bit controls whether the host controller skips processing the Periodic Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Periodic Schedule 1 - Use the PERIODICLISTBASE register to access the Periodic Schedule.
    #[inline(always)]
    pub fn pse(&mut self) -> PseW<'_, UsbcmdSpec> {
        PseW::new(self, 4)
    }
    ///Bit 5 - ASE Asynchronous Schedule Enable - Read/Write. Default 0b. This bit controls whether the host controller skips processing the Asynchronous Schedule. Only the host controller uses this bit. Values Meaning 0 - Do not process the Asynchronous Schedule. 1 - Use the ASYNCLISTADDR register to access the Asynchronous Schedule.
    #[inline(always)]
    pub fn ase(&mut self) -> AseW<'_, UsbcmdSpec> {
        AseW::new(self, 5)
    }
    ///Bit 6 - IAA Interrupt on Async Advance Doorbell - Read/Write. This bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule. Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule states, it sets the Interrupt on Async Advance status bit in the USBSTS register. If the Interrupt on Sync Advance Enable bit in the USBINTR register is one, then the host controller will assert an interrupt at the next interrupt threshold. The host controller sets this bit to zero after it has set the Interrupt on Sync Advance status bit in the USBSTS register to one. Software should not write a one to this bit when the asynchronous schedule is inactive. Doing so will yield undefined results. This bit is only used in host mode. Writing a one to this bit when device mode is selected will have undefined results.
    #[inline(always)]
    pub fn iaa(&mut self) -> IaaW<'_, UsbcmdSpec> {
        IaaW::new(self, 6)
    }
    ///Bits 8:9 - ASP Asynchronous Schedule Park Mode Count - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this field defaults to 3h and is R/W. Otherwise it defaults to zero and is Read-Only. It contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid values are 1h to 3h. Software must not write a zero to this bit when Park Mode Enable is a one as this will result in undefined behavior. This field is set to 3h in all controller core.
    #[inline(always)]
    pub fn asp(&mut self) -> AspW<'_, UsbcmdSpec> {
        AspW::new(self, 8)
    }
    ///Bit 11 - ASPE Asynchronous Schedule Park Mode Enable - Read/Write. If the Asynchronous Park Capability bit in the HCCPARAMS register is a one, then this bit defaults to a 1h and is R/W. Otherwise the bit must be a zero and is RO. Software uses this bit to enable or disable Park mode. When this bit is one, Park mode is enabled. When this bit is a zero, Park mode is disabled. NOTE: ASPE bit reset value: '0b' for OTG controller .
    #[inline(always)]
    pub fn aspe(&mut self) -> AspeW<'_, UsbcmdSpec> {
        AspeW::new(self, 11)
    }
    ///Bit 12 - Asynchronous Schedule start- Write only， host mode only。 this bit is used to notify hostcontroller to start async schedule immediately.
    #[inline(always)]
    pub fn prm(&mut self) -> PrmW<'_, UsbcmdSpec> {
        PrmW::new(self, 12)
    }
    ///Bit 13 - SUTW Setup TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure that the setup data payload of 8 bytes is extracted from a QH by the DCD without being corrupted. If the setup lockout mode is off (SLOM bit in USB core register n_USBMODE, see USBMODE ) then there is a hazard when new setup data arrives while the DCD is copying the setup data payload from the QH for a previous setup packet. This bit is set and cleared by software. This bit would also be cleared by hardware when a hazard detected.
    #[inline(always)]
    pub fn sutw(&mut self) -> SutwW<'_, UsbcmdSpec> {
        SutwW::new(self, 13)
    }
    ///Bit 14 - ATDTW Add dTD TripWire - Read/Write. \[device mode only\] This bit is used as a semaphore to ensure proper addition of a new dTD to an active (primed) endpoint's linked list. This bit is set and cleared by software. This bit would also be cleared by hardware when state machine is hazard region for which adding a dTD to a primed endpoint may go unrecognized.
    #[inline(always)]
    pub fn atdtw(&mut self) -> AtdtwW<'_, UsbcmdSpec> {
        AtdtwW::new(self, 14)
    }
    ///Bit 15 - FS_2 Frame List Size - (Read/Write or Read Only). \[host mode only\] This field is Read/Write only if Programmable Frame List Flag in the HCCPARAMS registers is set to one. This field specifies the size of the frame list that controls which bits in the Frame Index Register should be used for the Frame List Current index. NOTE: This field is made up from USBCMD bits 15, 3 and 2. Value Meaning 0b000 - 1024 elements (4096 bytes) Default value 0b001 - 512 elements (2048 bytes) 0b010 - 256 elements (1024 bytes) 0b011 - 128 elements (512 bytes) 0b100 - 64 elements (256 bytes) 0b101 - 32 elements (128 bytes) 0b110 - 16 elements (64 bytes) 0b111 - 8 elements (32 bytes)
    #[inline(always)]
    pub fn fs_2(&mut self) -> Fs2W<'_, UsbcmdSpec> {
        Fs2W::new(self, 15)
    }
    ///Bits 16:23 - ITC Interrupt Threshold Control -Read/Write. The system software uses this field to set the maximum rate at which the host/device controller will issue interrupts. ITC contains the maximum interrupt interval measured in micro-frames. Valid values are shown below. Value Maximum Interrupt Interval 00000000 - Immediate (no threshold) 00000001 - 1 micro-frame 00000010 - 2 micro-frames 00000100 - 4 micro-frames 00001000 - 8 micro-frames 00010000 - 16 micro-frames 00100000 - 32 micro-frames 01000000 - 64 micro-frames
    #[inline(always)]
    pub fn itc(&mut self) -> ItcW<'_, UsbcmdSpec> {
        ItcW::new(self, 16)
    }
}
/**USB Command Register

You can [`read`](crate::Reg::read) this register and get [`usbcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbcmdSpec;
impl crate::RegisterSpec for UsbcmdSpec {
    type Ux = u32;
}
///`read()` method returns [`usbcmd::R`](R) reader structure
impl crate::Readable for UsbcmdSpec {}
///`write(|w| ..)` method takes [`usbcmd::W`](W) writer structure
impl crate::Writable for UsbcmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBCMD to value 0x0008_0000
impl crate::Resettable for UsbcmdSpec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
