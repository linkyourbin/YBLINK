///Register `POS_MATRIX_SEL0` reader
pub type R = crate::R<PosMatrixSel0Spec>;
///Register `POS_MATRIX_SEL0` writer
pub type W = crate::W<PosMatrixSel0Spec>;
///Field `SEI_POSIN0_SEL` reader - No description available
pub type SeiPosin0SelR = crate::FieldReader;
///Field `SEI_POSIN0_SEL` writer - No description available
pub type SeiPosin0SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SEI_POSIN1_SEL` reader - No description available
pub type SeiPosin1SelR = crate::FieldReader;
///Field `SEI_POSIN1_SEL` writer - No description available
pub type SeiPosin1SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MMC0_POSIN_SEL` reader - No description available
pub type Mmc0PosinSelR = crate::FieldReader;
///Field `MMC0_POSIN_SEL` writer - No description available
pub type Mmc0PosinSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MMC1_POSIN_SEL` reader - 0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved
pub type Mmc1PosinSelR = crate::FieldReader;
///Field `MMC1_POSIN_SEL` writer - 0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved
pub type Mmc1PosinSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn sei_posin0_sel(&self) -> SeiPosin0SelR {
        SeiPosin0SelR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn sei_posin1_sel(&self) -> SeiPosin1SelR {
        SeiPosin1SelR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - No description available
    #[inline(always)]
    pub fn mmc0_posin_sel(&self) -> Mmc0PosinSelR {
        Mmc0PosinSelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - 0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved
    #[inline(always)]
    pub fn mmc1_posin_sel(&self) -> Mmc1PosinSelR {
        Mmc1PosinSelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn sei_posin0_sel(&mut self) -> SeiPosin0SelW<'_, PosMatrixSel0Spec> {
        SeiPosin0SelW::new(self, 0)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn sei_posin1_sel(&mut self) -> SeiPosin1SelW<'_, PosMatrixSel0Spec> {
        SeiPosin1SelW::new(self, 8)
    }
    ///Bits 16:23 - No description available
    #[inline(always)]
    pub fn mmc0_posin_sel(&mut self) -> Mmc0PosinSelW<'_, PosMatrixSel0Spec> {
        Mmc0PosinSelW::new(self, 16)
    }
    ///Bits 24:31 - 0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved
    #[inline(always)]
    pub fn mmc1_posin_sel(&mut self) -> Mmc1PosinSelW<'_, PosMatrixSel0Spec> {
        Mmc1PosinSelW::new(self, 24)
    }
}
/**position matrix select register0

You can [`read`](crate::Reg::read) this register and get [`pos_matrix_sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pos_matrix_sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PosMatrixSel0Spec;
impl crate::RegisterSpec for PosMatrixSel0Spec {
    type Ux = u32;
}
///`read()` method returns [`pos_matrix_sel0::R`](R) reader structure
impl crate::Readable for PosMatrixSel0Spec {}
///`write(|w| ..)` method takes [`pos_matrix_sel0::W`](W) writer structure
impl crate::Writable for PosMatrixSel0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POS_MATRIX_SEL0 to value 0
impl crate::Resettable for PosMatrixSel0Spec {}
