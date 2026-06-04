///Register `ENDPTCOMPLETE` reader
pub type R = crate::R<EndptcompleteSpec>;
///Register `ENDPTCOMPLETE` writer
pub type W = crate::W<EndptcompleteSpec>;
///Field `ERCE` reader - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\[N\] - Endpoint #N, N is in 0..7
pub type ErceR = crate::FieldReader<u16>;
///Field `ERCE` writer - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\[N\] - Endpoint #N, N is in 0..7
pub type ErceW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ETCE` reader - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\[N\] - Endpoint #N, N is in 0..7
pub type EtceR = crate::FieldReader<u16>;
///Field `ETCE` writer - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\[N\] - Endpoint #N, N is in 0..7
pub type EtceW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn erce(&self) -> ErceR {
        ErceR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn etce(&self) -> EtceR {
        EtceR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn erce(&mut self) -> ErceW<'_, EndptcompleteSpec> {
        ErceW::new(self, 0)
    }
    ///Bits 16:31 - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\[N\] - Endpoint #N, N is in 0..7
    #[inline(always)]
    pub fn etce(&mut self) -> EtceW<'_, EndptcompleteSpec> {
        EtceW::new(self, 16)
    }
}
/**Endpoint Complete Register

You can [`read`](crate::Reg::read) this register and get [`endptcomplete::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptcomplete::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EndptcompleteSpec;
impl crate::RegisterSpec for EndptcompleteSpec {
    type Ux = u32;
}
///`read()` method returns [`endptcomplete::R`](R) reader structure
impl crate::Readable for EndptcompleteSpec {}
///`write(|w| ..)` method takes [`endptcomplete::W`](W) writer structure
impl crate::Writable for EndptcompleteSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTCOMPLETE to value 0
impl crate::Resettable for EndptcompleteSpec {}
