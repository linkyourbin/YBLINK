///Register `ENDPTCTRL[%s]` reader
pub type R = crate::R<EndptctrlSpec>;
///Register `ENDPTCTRL[%s]` writer
pub type W = crate::W<EndptctrlSpec>;
///Field `RXS` reader - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \[Default\] 1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
pub type RxsR = crate::BitReader;
///Field `RXS` writer - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \[Default\] 1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
pub type RxsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXT` reader - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
pub type RxtR = crate::FieldReader;
///Field `RXT` writer - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
pub type RxtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXR` writer - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device.
pub type RxrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXE` reader - RXE RX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
pub type RxeR = crate::BitReader;
///Field `RXE` writer - RXE RX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXS` reader - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
pub type TxsR = crate::BitReader;
///Field `TXS` writer - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
pub type TxsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXT` reader - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
pub type TxtR = crate::FieldReader;
///Field `TXT` writer - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
pub type TxtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TXR` writer - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device.
pub type TxrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXE` reader - TXE TX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
pub type TxeR = crate::BitReader;
///Field `TXE` writer - TXE TX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \[Default\] 1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
    #[inline(always)]
    pub fn rxs(&self) -> RxsR {
        RxsR::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
    #[inline(always)]
    pub fn rxt(&self) -> RxtR {
        RxtR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 7 - RXE RX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
    #[inline(always)]
    pub fn txs(&self) -> TxsR {
        TxsR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:19 - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
    #[inline(always)]
    pub fn txt(&self) -> TxtR {
        TxtR::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 23 - TXE TX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \[Default\] 1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
    #[inline(always)]
    pub fn rxs(&mut self) -> RxsW<'_, EndptctrlSpec> {
        RxsW::new(self, 0)
    }
    ///Bits 2:3 - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
    #[inline(always)]
    pub fn rxt(&mut self) -> RxtW<'_, EndptctrlSpec> {
        RxtW::new(self, 2)
    }
    ///Bit 6 - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device.
    #[inline(always)]
    pub fn rxr(&mut self) -> RxrW<'_, EndptctrlSpec> {
        RxrW::new(self, 6)
    }
    ///Bit 7 - RXE RX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, EndptctrlSpec> {
        RxeW::new(self, 7)
    }
    ///Bit 16 - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \[CONTROL ENDPOINT TYPES ONLY\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit.
    #[inline(always)]
    pub fn txs(&mut self) -> TxsW<'_, EndptctrlSpec> {
        TxsW::new(self, 16)
    }
    ///Bits 18:19 - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
    #[inline(always)]
    pub fn txt(&mut self) -> TxtW<'_, EndptctrlSpec> {
        TxtW::new(self, 18)
    }
    ///Bit 22 - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device.
    #[inline(always)]
    pub fn txr(&mut self) -> TxrW<'_, EndptctrlSpec> {
        TxrW::new(self, 22)
    }
    ///Bit 23 - TXE TX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured.
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, EndptctrlSpec> {
        TxeW::new(self, 23)
    }
}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`endptctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EndptctrlSpec;
impl crate::RegisterSpec for EndptctrlSpec {
    type Ux = u32;
}
///`read()` method returns [`endptctrl::R`](R) reader structure
impl crate::Readable for EndptctrlSpec {}
///`write(|w| ..)` method takes [`endptctrl::W`](W) writer structure
impl crate::Writable for EndptctrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTCTRL[%s] to value 0
impl crate::Resettable for EndptctrlSpec {}
