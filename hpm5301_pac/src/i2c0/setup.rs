///Register `Setup` reader
pub type R = crate::R<SetupSpec>;
///Register `Setup` writer
pub type W = crate::W<SetupSpec>;
///Field `IICEN` reader - Enable the I2C controller. 1: Enable 0: Disable
pub type IicenR = crate::BitReader;
///Field `IICEN` writer - Enable the I2C controller. 1: Enable 0: Disable
pub type IicenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRESSING` reader - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode
pub type AddressingR = crate::BitReader;
///Field `ADDRESSING` writer - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode
pub type AddressingW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASTER` reader - Configure this device as a master or a slave. 1: Master mode 0: Slave mode
pub type MasterR = crate::BitReader;
///Field `MASTER` writer - Configure this device as a master or a slave. 1: Master mode 0: Slave mode
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - Enable the direct memory access mode data transfer. 1: Enable 0: Disable
pub type DmaenR = crate::BitReader;
///Field `DMAEN` writer - Enable the direct memory access mode data transfer. 1: Enable 0: Disable
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `T_SCLHI` reader - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode.
pub type TSclhiR = crate::FieldReader<u16>;
///Field `T_SCLHI` writer - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode.
pub type TSclhiW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `T_SCLRADIO` reader - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode.
pub type TSclradioR = crate::BitReader;
///Field `T_SCLRADIO` writer - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode.
pub type TSclradioW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `T_HDDAT` reader - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)
pub type THddatR = crate::FieldReader;
///Field `T_HDDAT` writer - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)
pub type THddatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `T_SP` reader - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)
pub type TSpR = crate::FieldReader;
///Field `T_SP` writer - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)
pub type TSpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `T_SUDAT` reader - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register
pub type TSudatR = crate::FieldReader;
///Field `T_SUDAT` writer - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register
pub type TSudatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - Enable the I2C controller. 1: Enable 0: Disable
    #[inline(always)]
    pub fn iicen(&self) -> IicenR {
        IicenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode
    #[inline(always)]
    pub fn addressing(&self) -> AddressingR {
        AddressingR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configure this device as a master or a slave. 1: Master mode 0: Slave mode
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable the direct memory access mode data transfer. 1: Enable 0: Disable
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:12 - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode.
    #[inline(always)]
    pub fn t_sclhi(&self) -> TSclhiR {
        TSclhiR::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    ///Bit 13 - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode.
    #[inline(always)]
    pub fn t_sclradio(&self) -> TSclradioR {
        TSclradioR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:20 - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)
    #[inline(always)]
    pub fn t_hddat(&self) -> THddatR {
        THddatR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:23 - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)
    #[inline(always)]
    pub fn t_sp(&self) -> TSpR {
        TSpR::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:28 - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register
    #[inline(always)]
    pub fn t_sudat(&self) -> TSudatR {
        TSudatR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - Enable the I2C controller. 1: Enable 0: Disable
    #[inline(always)]
    pub fn iicen(&mut self) -> IicenW<'_, SetupSpec> {
        IicenW::new(self, 0)
    }
    ///Bit 1 - I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode
    #[inline(always)]
    pub fn addressing(&mut self) -> AddressingW<'_, SetupSpec> {
        AddressingW::new(self, 1)
    }
    ///Bit 2 - Configure this device as a master or a slave. 1: Master mode 0: Slave mode
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<'_, SetupSpec> {
        MasterW::new(self, 2)
    }
    ///Bit 3 - Enable the direct memory access mode data transfer. 1: Enable 0: Disable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, SetupSpec> {
        DmaenW::new(self, 3)
    }
    ///Bits 4:12 - The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode.
    #[inline(always)]
    pub fn t_sclhi(&mut self) -> TSclhiW<'_, SetupSpec> {
        TSclhiW::new(self, 4)
    }
    ///Bit 13 - The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode.
    #[inline(always)]
    pub fn t_sclradio(&mut self) -> TSclradioW<'_, SetupSpec> {
        TSclradioW::new(self, 13)
    }
    ///Bits 16:20 - T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)
    #[inline(always)]
    pub fn t_hddat(&mut self) -> THddatW<'_, SetupSpec> {
        THddatW::new(self, 16)
    }
    ///Bits 21:23 - T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)
    #[inline(always)]
    pub fn t_sp(&mut self) -> TSpW<'_, SetupSpec> {
        TSpW::new(self, 21)
    }
    ///Bits 24:28 - T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register
    #[inline(always)]
    pub fn t_sudat(&mut self) -> TSudatW<'_, SetupSpec> {
        TSudatW::new(self, 24)
    }
}
/**Setup Register

You can [`read`](crate::Reg::read) this register and get [`setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SetupSpec;
impl crate::RegisterSpec for SetupSpec {
    type Ux = u32;
}
///`read()` method returns [`setup::R`](R) reader structure
impl crate::Readable for SetupSpec {}
///`write(|w| ..)` method takes [`setup::W`](W) writer structure
impl crate::Writable for SetupSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets Setup to value 0x0525_2100
impl crate::Resettable for SetupSpec {
    const RESET_VALUE: u32 = 0x0525_2100;
}
