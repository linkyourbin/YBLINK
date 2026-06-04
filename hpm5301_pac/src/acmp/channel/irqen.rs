///Register `irqen` reader
pub type R = crate::R<IrqenSpec>;
///Register `irqen` writer
pub type W = crate::W<IrqenSpec>;
///Field `REDGEN` reader - Output rising edge flag interrupt enable bit.
pub type RedgenR = crate::BitReader;
///Field `REDGEN` writer - Output rising edge flag interrupt enable bit.
pub type RedgenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEDGEN` reader - Output falling edge flag interrupt enable bit.
pub type FedgenR = crate::BitReader;
///Field `FEDGEN` writer - Output falling edge flag interrupt enable bit.
pub type FedgenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Output rising edge flag interrupt enable bit.
    #[inline(always)]
    pub fn redgen(&self) -> RedgenR {
        RedgenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output falling edge flag interrupt enable bit.
    #[inline(always)]
    pub fn fedgen(&self) -> FedgenR {
        FedgenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Output rising edge flag interrupt enable bit.
    #[inline(always)]
    pub fn redgen(&mut self) -> RedgenW<'_, IrqenSpec> {
        RedgenW::new(self, 0)
    }
    ///Bit 1 - Output falling edge flag interrupt enable bit.
    #[inline(always)]
    pub fn fedgen(&mut self) -> FedgenW<'_, IrqenSpec> {
        FedgenW::new(self, 1)
    }
}
/**Interrupt request enable register

You can [`read`](crate::Reg::read) this register and get [`irqen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IrqenSpec;
impl crate::RegisterSpec for IrqenSpec {
    type Ux = u32;
}
///`read()` method returns [`irqen::R`](R) reader structure
impl crate::Readable for IrqenSpec {}
///`write(|w| ..)` method takes [`irqen::W`](W) writer structure
impl crate::Writable for IrqenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets irqen to value 0
impl crate::Resettable for IrqenSpec {}
