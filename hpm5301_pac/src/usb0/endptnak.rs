///Register `ENDPTNAK` reader
pub type R = crate::R<EndptnakSpec>;
///Register `ENDPTNAK` writer
pub type W = crate::W<EndptnakSpec>;
///Field `EPRN` reader - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EprnR = crate::FieldReader<u16>;
///Field `EPRN` writer - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EprnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EPTN` reader - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EptnR = crate::FieldReader<u16>;
///Field `EPTN` writer - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EptnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eprn(&self) -> EprnR {
        EprnR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eptn(&self) -> EptnR {
        EptnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eprn(&mut self) -> EprnW<'_, EndptnakSpec> {
        EprnW::new(self, 0)
    }
    ///Bits 16:31 - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eptn(&mut self) -> EptnW<'_, EndptnakSpec> {
        EptnW::new(self, 16)
    }
}
/**Endpoint NAK Register

You can [`read`](crate::Reg::read) this register and get [`endptnak::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptnak::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EndptnakSpec;
impl crate::RegisterSpec for EndptnakSpec {
    type Ux = u32;
}
///`read()` method returns [`endptnak::R`](R) reader structure
impl crate::Readable for EndptnakSpec {}
///`write(|w| ..)` method takes [`endptnak::W`](W) writer structure
impl crate::Writable for EndptnakSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTNAK to value 0
impl crate::Resettable for EndptnakSpec {}
