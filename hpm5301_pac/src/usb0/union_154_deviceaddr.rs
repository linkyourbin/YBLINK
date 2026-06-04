///Register `DEVICEADDR` reader
pub type R = crate::R<Union154DeviceaddrSpec>;
///Register `DEVICEADDR` writer
pub type W = crate::W<Union154DeviceaddrSpec>;
///Field `USBADRA` reader - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement.
pub type UsbadraR = crate::BitReader;
///Field `USBADRA` writer - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement.
pub type UsbadraW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBADR` reader - USBADR Device Address. These bits correspond to the USB device address
pub type UsbadrR = crate::FieldReader;
///Field `USBADR` writer - USBADR Device Address. These bits correspond to the USB device address
pub type UsbadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 24 - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement.
    #[inline(always)]
    pub fn usbadra(&self) -> UsbadraR {
        UsbadraR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:31 - USBADR Device Address. These bits correspond to the USB device address
    #[inline(always)]
    pub fn usbadr(&self) -> UsbadrR {
        UsbadrR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    ///Bit 24 - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement.
    #[inline(always)]
    pub fn usbadra(&mut self) -> UsbadraW<'_, Union154DeviceaddrSpec> {
        UsbadraW::new(self, 24)
    }
    ///Bits 25:31 - USBADR Device Address. These bits correspond to the USB device address
    #[inline(always)]
    pub fn usbadr(&mut self) -> UsbadrW<'_, Union154DeviceaddrSpec> {
        UsbadrW::new(self, 25)
    }
}
/**Device Address Register

You can [`read`](crate::Reg::read) this register and get [`union_154_deviceaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`union_154_deviceaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Union154DeviceaddrSpec;
impl crate::RegisterSpec for Union154DeviceaddrSpec {
    type Ux = u32;
}
///`read()` method returns [`union_154_deviceaddr::R`](R) reader structure
impl crate::Readable for Union154DeviceaddrSpec {}
///`write(|w| ..)` method takes [`union_154_deviceaddr::W`](W) writer structure
impl crate::Writable for Union154DeviceaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEVICEADDR to value 0
impl crate::Resettable for Union154DeviceaddrSpec {}
