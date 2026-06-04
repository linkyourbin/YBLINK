///Register `TRGM_OUT[%s]` reader
pub type R = crate::R<TrgmOutSpec>;
///Register `TRGM_OUT[%s]` writer
pub type W = crate::W<TrgmOutSpec>;
///Field `TRGM_OUT` reader - motor_to_opamp0\[7:0\] = trig_mux_out\[7:0\]; motor_to_opamp1\[7:0\] = trig_mux_out\[15:8\]; motor_to_gtmr0_capt\[1:0\] = trig_mux_out\[17:16\]; motor_to_gtmr0_sync = trig_mux_out\[18\]; motor_to_gtmr1_capt\[1:0\] = trig_mux_out\[20:19\]; motor_to_gtmr1_sync = trig_mux_out\[21\]; motor_to_gtmr2_capt\[1:0\] = trig_mux_out\[23:22\]; motor_to_gtmr2_sync = trig_mux_out\[24\]; motor_to_gtmr3_capt\[1:0\] = trig_mux_out\[26:25\]; motor_to_gtmr3_sync = trig_mux_out\[27\]; acmp_window\[1:0\] = trig_mux_out\[29:28\]; dac0_buf_trigger = trig_mux_out\[30\]; dac1_buf_trigger = trig_mux_out\[31\]; dac0_step_trigger\[3:0\] = {trig_mux_out\[24:22\],trig_mux_out\[30\]};//use same buf_trig, and gtmr2 dac1_step_trigger\[3:0\] = {trig_mux_out\[27:25\],trig_mux_out\[31\]}; //use same buf_trig, and gtmr3
pub type TrgmOutR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - motor_to_opamp0\[7:0\] = trig_mux_out\[7:0\]; motor_to_opamp1\[7:0\] = trig_mux_out\[15:8\]; motor_to_gtmr0_capt\[1:0\] = trig_mux_out\[17:16\]; motor_to_gtmr0_sync = trig_mux_out\[18\]; motor_to_gtmr1_capt\[1:0\] = trig_mux_out\[20:19\]; motor_to_gtmr1_sync = trig_mux_out\[21\]; motor_to_gtmr2_capt\[1:0\] = trig_mux_out\[23:22\]; motor_to_gtmr2_sync = trig_mux_out\[24\]; motor_to_gtmr3_capt\[1:0\] = trig_mux_out\[26:25\]; motor_to_gtmr3_sync = trig_mux_out\[27\]; acmp_window\[1:0\] = trig_mux_out\[29:28\]; dac0_buf_trigger = trig_mux_out\[30\]; dac1_buf_trigger = trig_mux_out\[31\]; dac0_step_trigger\[3:0\] = {trig_mux_out\[24:22\],trig_mux_out\[30\]};//use same buf_trig, and gtmr2 dac1_step_trigger\[3:0\] = {trig_mux_out\[27:25\],trig_mux_out\[31\]}; //use same buf_trig, and gtmr3
    #[inline(always)]
    pub fn trgm_out(&self) -> TrgmOutR {
        TrgmOutR::new(self.bits)
    }
}
impl W {}
/**no description available

You can [`read`](crate::Reg::read) this register and get [`trgm_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgm_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrgmOutSpec;
impl crate::RegisterSpec for TrgmOutSpec {
    type Ux = u32;
}
///`read()` method returns [`trgm_out::R`](R) reader structure
impl crate::Readable for TrgmOutSpec {}
///`write(|w| ..)` method takes [`trgm_out::W`](W) writer structure
impl crate::Writable for TrgmOutSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRGM_OUT[%s] to value 0
impl crate::Resettable for TrgmOutSpec {}
