///Register `LCR` reader
pub type R = crate::R<LcrSpec>;
///Register `LCR` writer
pub type W = crate::W<LcrSpec>;
///Field `WLS` reader - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits
pub type WlsR = crate::FieldReader;
///Field `WLS` writer - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits
pub type WlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STB` reader - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits
pub type StbR = crate::BitReader;
///Field `STB` writer - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits
pub type StbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEN` reader - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data.
pub type PenR = crate::BitReader;
///Field `PEN` writer - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data.
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPS` reader - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity.
pub type EpsR = crate::BitReader;
///Field `EPS` writer - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity.
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPS` reader - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity.
pub type SpsR = crate::BitReader;
///Field `SPS` writer - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity.
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BC` reader - Break control
pub type BcR = crate::BitReader;
///Field `BC` writer - Break control
pub type BcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLAB` reader - Divisor latch access bit
pub type DlabR = crate::BitReader;
///Field `DLAB` writer - Divisor latch access bit
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits
    #[inline(always)]
    pub fn wls(&self) -> WlsR {
        WlsR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data.
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity.
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity.
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Break control
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Divisor latch access bit
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits
    #[inline(always)]
    pub fn wls(&mut self) -> WlsW<'_, LcrSpec> {
        WlsW::new(self, 0)
    }
    ///Bit 2 - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits
    #[inline(always)]
    pub fn stb(&mut self) -> StbW<'_, LcrSpec> {
        StbW::new(self, 2)
    }
    ///Bit 3 - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data.
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, LcrSpec> {
        PenW::new(self, 3)
    }
    ///Bit 4 - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity.
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<'_, LcrSpec> {
        EpsW::new(self, 4)
    }
    ///Bit 5 - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity.
    #[inline(always)]
    pub fn sps(&mut self) -> SpsW<'_, LcrSpec> {
        SpsW::new(self, 5)
    }
    ///Bit 6 - Break control
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, LcrSpec> {
        BcW::new(self, 6)
    }
    ///Bit 7 - Divisor latch access bit
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<'_, LcrSpec> {
        DlabW::new(self, 7)
    }
}
/**Line Control Register

You can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
///`read()` method returns [`lcr::R`](R) reader structure
impl crate::Readable for LcrSpec {}
///`write(|w| ..)` method takes [`lcr::W`](W) writer structure
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCR to value 0
impl crate::Resettable for LcrSpec {}
