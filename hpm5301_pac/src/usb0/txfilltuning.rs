///Register `TXFILLTUNING` reader
pub type R = crate::R<TxfilltuningSpec>;
///Register `TXFILLTUNING` writer
pub type W = crate::W<TxfilltuningSpec>;
///Field `TXSCHOH` reader - TXSCHOH Scheduler Overhead. (Read/Write) \[Default = 0\] This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core .
pub type TxschohR = crate::FieldReader;
///Field `TXSCHOH` writer - TXSCHOH Scheduler Overhead. (Read/Write) \[Default = 0\] This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core .
pub type TxschohW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `TXSCHHEALTH` reader - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31.
pub type TxschhealthR = crate::FieldReader;
///Field `TXSCHHEALTH` writer - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31.
pub type TxschhealthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TXFIFOTHRES` reader - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set.
pub type TxfifothresR = crate::FieldReader;
///Field `TXFIFOTHRES` writer - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set.
pub type TxfifothresW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:6 - TXSCHOH Scheduler Overhead. (Read/Write) \[Default = 0\] This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core .
    #[inline(always)]
    pub fn txschoh(&self) -> TxschohR {
        TxschohR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:12 - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31.
    #[inline(always)]
    pub fn txschhealth(&self) -> TxschhealthR {
        TxschhealthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:21 - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set.
    #[inline(always)]
    pub fn txfifothres(&self) -> TxfifothresR {
        TxfifothresR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:6 - TXSCHOH Scheduler Overhead. (Read/Write) \[Default = 0\] This register adds an additional fixed offset to the schedule time estimator described above as Tff. As an approximation, the value chosen for this register should limit the number of back-off events captured in the TXSCHHEALTH to less than 10 per second in a highly utilized bus. Choosing a value that is too high for this register is not desired as it can needlessly reduce USB utilization. The time unit represented in this register is 1.267us when a device is connected in High-Speed Mode. The time unit represented in this register is 6.333us when a device is connected in Low/Full Speed Mode. Default value is '08h' for OTG controller core .
    #[inline(always)]
    pub fn txschoh(&mut self) -> TxschohW<'_, TxfilltuningSpec> {
        TxschohW::new(self, 0)
    }
    ///Bits 8:12 - TXSCHHEALTH Scheduler Health Counter. (Read/Write To Clear) Table continues on the next page This register increments when the host controller fails to fill the TX latency FIFO to the level programmed by TXFIFOTHRES before running out of time to send the packet before the next Start-Of-Frame. This health counter measures the number of times this occurs to provide feedback to selecting a proper TXSCHOH. Writing to this register will clear the counter and this counter will max. at 31.
    #[inline(always)]
    pub fn txschhealth(&mut self) -> TxschhealthW<'_, TxfilltuningSpec> {
        TxschhealthW::new(self, 8)
    }
    ///Bits 16:21 - TXFIFOTHRES FIFO Burst Threshold. (Read/Write) This register controls the number of data bursts that are posted to the TX latency FIFO in host mode before the packet begins on to the bus. The minimum value is 2 and this value should be a low as possible to maximize USB performance. A higher value can be used in systems with unpredictable latency and/or insufficient bandwidth where the FIFO may underrun because the data transferred from the latency FIFO to USB occurs before it can be replenished from system memory. This value is ignored if the Stream Disable bit in USB_n_USBMODE register is set.
    #[inline(always)]
    pub fn txfifothres(&mut self) -> TxfifothresW<'_, TxfilltuningSpec> {
        TxfifothresW::new(self, 16)
    }
}
/**TX FIFO Fill Tuning Register

You can [`read`](crate::Reg::read) this register and get [`txfilltuning::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfilltuning::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TxfilltuningSpec;
impl crate::RegisterSpec for TxfilltuningSpec {
    type Ux = u32;
}
///`read()` method returns [`txfilltuning::R`](R) reader structure
impl crate::Readable for TxfilltuningSpec {}
///`write(|w| ..)` method takes [`txfilltuning::W`](W) writer structure
impl crate::Writable for TxfilltuningSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXFILLTUNING to value 0
impl crate::Resettable for TxfilltuningSpec {}
