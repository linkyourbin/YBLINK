///Register `ENDPTNAKEN` reader
pub type R = crate::R<EndptnakenSpec>;
///Register `ENDPTNAKEN` writer
pub type W = crate::W<EndptnakenSpec>;
///Field `EPRNE` reader - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EprneR = crate::FieldReader<u16>;
///Field `EPRNE` writer - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EprneW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EPTNE` reader - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EptneR = crate::FieldReader<u16>;
///Field `EPTNE` writer - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
pub type EptneW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eprne(&self) -> EprneR {
        EprneR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eptne(&self) -> EptneR {
        EptneR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eprne(&mut self) -> EprneW<'_, EndptnakenSpec> {
        EprneW::new(self, 0)
    }
    ///Bits 16:31 - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \[N\] - Endpoint #\[N\], N is 0-7
    #[inline(always)]
    pub fn eptne(&mut self) -> EptneW<'_, EndptnakenSpec> {
        EptneW::new(self, 16)
    }
}
/**Endpoint NAK Enable Register

You can [`read`](crate::Reg::read) this register and get [`endptnaken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endptnaken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EndptnakenSpec;
impl crate::RegisterSpec for EndptnakenSpec {
    type Ux = u32;
}
///`read()` method returns [`endptnaken::R`](R) reader structure
impl crate::Readable for EndptnakenSpec {}
///`write(|w| ..)` method takes [`endptnaken::W`](W) writer structure
impl crate::Writable for EndptnakenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENDPTNAKEN to value 0
impl crate::Resettable for EndptnakenSpec {}
