///Register `POS_MATRIX_SEL1` reader
pub type R = crate::R<PosMatrixSel1Spec>;
///Register `POS_MATRIX_SEL1` writer
pub type W = crate::W<PosMatrixSel1Spec>;
///Field `QEO0_POS_SEL` reader - No description available
pub type Qeo0PosSelR = crate::FieldReader;
///Field `QEO0_POS_SEL` writer - No description available
pub type Qeo0PosSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QEO1_POS_SEL` reader - No description available
pub type Qeo1PosSelR = crate::FieldReader;
///Field `QEO1_POS_SEL` writer - No description available
pub type Qeo1PosSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn qeo0_pos_sel(&self) -> Qeo0PosSelR {
        Qeo0PosSelR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn qeo1_pos_sel(&self) -> Qeo1PosSelR {
        Qeo1PosSelR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - No description available
    #[inline(always)]
    pub fn qeo0_pos_sel(&mut self) -> Qeo0PosSelW<'_, PosMatrixSel1Spec> {
        Qeo0PosSelW::new(self, 0)
    }
    ///Bits 8:15 - No description available
    #[inline(always)]
    pub fn qeo1_pos_sel(&mut self) -> Qeo1PosSelW<'_, PosMatrixSel1Spec> {
        Qeo1PosSelW::new(self, 8)
    }
}
/**position matrix select register1

You can [`read`](crate::Reg::read) this register and get [`pos_matrix_sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pos_matrix_sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PosMatrixSel1Spec;
impl crate::RegisterSpec for PosMatrixSel1Spec {
    type Ux = u32;
}
///`read()` method returns [`pos_matrix_sel1::R`](R) reader structure
impl crate::Readable for PosMatrixSel1Spec {}
///`write(|w| ..)` method takes [`pos_matrix_sel1::W`](W) writer structure
impl crate::Writable for PosMatrixSel1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POS_MATRIX_SEL1 to value 0
impl crate::Resettable for PosMatrixSel1Spec {}
