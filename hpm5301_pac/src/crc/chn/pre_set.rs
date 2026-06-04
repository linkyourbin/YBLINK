///Register `pre_set` reader
pub type R = crate::R<PreSetSpec>;
///Register `pre_set` writer
pub type W = crate::W<PreSetSpec>;
///Field `PRE_SET` reader - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb
pub type PreSetR = crate::FieldReader;
///Field `PRE_SET` writer - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb
pub type PreSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb
    #[inline(always)]
    pub fn pre_set(&self) -> PreSetR {
        PreSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb
    #[inline(always)]
    pub fn pre_set(&mut self) -> PreSetW<'_, PreSetSpec> {
        PreSetW::new(self, 0)
    }
}
/**&index0 pre set for crc setting

You can [`read`](crate::Reg::read) this register and get [`pre_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PreSetSpec;
impl crate::RegisterSpec for PreSetSpec {
    type Ux = u32;
}
///`read()` method returns [`pre_set::R`](R) reader structure
impl crate::Readable for PreSetSpec {}
///`write(|w| ..)` method takes [`pre_set::W`](W) writer structure
impl crate::Writable for PreSetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets pre_set to value 0
impl crate::Resettable for PreSetSpec {}
