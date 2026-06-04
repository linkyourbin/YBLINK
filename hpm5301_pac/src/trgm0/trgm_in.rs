///Register `TRGM_IN[%s]` reader
pub type R = crate::R<TrgmInSpec>;
///Register `TRGM_IN[%s]` writer
pub type W = crate::W<TrgmInSpec>;
///Field `TRGM_IN` reader - mmc1_trig_out\[1:0\], mmc0_trig_out\[1:0\],sync_pulse\[3:0\],moto_gpio_in_sync\[7:0\],//31:16 gtmr3_to_motor_sync\[1:0\],gtmr2_to_motor_sync\[1:0\],gtmr1_to_motor_sync\[1:0\],gtmr0_to_motor_sync\[1:0\], //15:8 acmp_out_sync\[1:0\],can2mot_event_sync\[1:0\],usb0_sof_tog_sync,pwm_debug,1'b1,1'b0 //7:0
pub type TrgmInR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - mmc1_trig_out\[1:0\], mmc0_trig_out\[1:0\],sync_pulse\[3:0\],moto_gpio_in_sync\[7:0\],//31:16 gtmr3_to_motor_sync\[1:0\],gtmr2_to_motor_sync\[1:0\],gtmr1_to_motor_sync\[1:0\],gtmr0_to_motor_sync\[1:0\], //15:8 acmp_out_sync\[1:0\],can2mot_event_sync\[1:0\],usb0_sof_tog_sync,pwm_debug,1'b1,1'b0 //7:0
    #[inline(always)]
    pub fn trgm_in(&self) -> TrgmInR {
        TrgmInR::new(self.bits)
    }
}
impl W {}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`trgm_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgm_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrgmInSpec;
impl crate::RegisterSpec for TrgmInSpec {
    type Ux = u32;
}
///`read()` method returns [`trgm_in::R`](R) reader structure
impl crate::Readable for TrgmInSpec {}
///`write(|w| ..)` method takes [`trgm_in::W`](W) writer structure
impl crate::Writable for TrgmInSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRGM_IN[%s] to value 0
impl crate::Resettable for TrgmInSpec {}
