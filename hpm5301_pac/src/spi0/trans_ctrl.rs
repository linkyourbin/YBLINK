///Register `TransCtrl` reader
pub type R = crate::R<TransCtrlSpec>;
///Register `TransCtrl` writer
pub type W = crate::W<TransCtrlSpec>;
///Field `RDTRANCNT` reader - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
pub type RdtrancntR = crate::FieldReader<u16>;
///Field `RDTRANCNT` writer - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
pub type RdtrancntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DUMMYCNT` reader - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases.
pub type DummycntR = crate::FieldReader;
///Field `DUMMYCNT` writer - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases.
pub type DummycntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TOKENVALUE` reader - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69
pub type TokenvalueR = crate::BitReader;
///Field `TOKENVALUE` writer - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69
pub type TokenvalueW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRTRANCNT` reader - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
pub type WrtrancntR = crate::FieldReader<u16>;
///Field `WRTRANCNT` writer - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
pub type WrtrancntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `TOKENEN` reader - Token transfer enable (Master mode only) Append a one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token
pub type TokenenR = crate::BitReader;
///Field `TOKENEN` writer - Token transfer enable (Master mode only) Append a one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token
pub type TokenenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DUALQUAD` reader - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved
pub type DualquadR = crate::FieldReader;
///Field `DUALQUAD` writer - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved
pub type DualquadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRANSMODE` reader - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved
pub type TransmodeR = crate::FieldReader;
///Field `TRANSMODE` writer - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved
pub type TransmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRFMT` reader - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad).
pub type AddrfmtR = crate::BitReader;
///Field `ADDRFMT` writer - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad).
pub type AddrfmtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDREN` reader - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase
pub type AddrenR = crate::BitReader;
///Field `ADDREN` writer - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase
pub type AddrenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDEN` reader - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase
pub type CmdenR = crate::BitReader;
///Field `CMDEN` writer - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase
pub type CmdenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLVDATAONLY` reader - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0.
pub type SlvdataonlyR = crate::BitReader;
///Field `SLVDATAONLY` writer - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0.
pub type SlvdataonlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
    #[inline(always)]
    pub fn rdtrancnt(&self) -> RdtrancntR {
        RdtrancntR::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:10 - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases.
    #[inline(always)]
    pub fn dummycnt(&self) -> DummycntR {
        DummycntR::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69
    #[inline(always)]
    pub fn tokenvalue(&self) -> TokenvalueR {
        TokenvalueR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:20 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
    #[inline(always)]
    pub fn wrtrancnt(&self) -> WrtrancntR {
        WrtrancntR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    ///Bit 21 - Token transfer enable (Master mode only) Append a one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token
    #[inline(always)]
    pub fn tokenen(&self) -> TokenenR {
        TokenenR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved
    #[inline(always)]
    pub fn dualquad(&self) -> DualquadR {
        DualquadR::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:27 - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved
    #[inline(always)]
    pub fn transmode(&self) -> TransmodeR {
        TransmodeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad).
    #[inline(always)]
    pub fn addrfmt(&self) -> AddrfmtR {
        AddrfmtR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase
    #[inline(always)]
    pub fn addren(&self) -> AddrenR {
        AddrenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase
    #[inline(always)]
    pub fn cmden(&self) -> CmdenR {
        CmdenR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0.
    #[inline(always)]
    pub fn slvdataonly(&self) -> SlvdataonlyR {
        SlvdataonlyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt.
    #[inline(always)]
    pub fn rdtrancnt(&mut self) -> RdtrancntW<'_, TransCtrlSpec> {
        RdtrancntW::new(self, 0)
    }
    ///Bits 9:10 - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases.
    #[inline(always)]
    pub fn dummycnt(&mut self) -> DummycntW<'_, TransCtrlSpec> {
        DummycntW::new(self, 9)
    }
    ///Bit 11 - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69
    #[inline(always)]
    pub fn tokenvalue(&mut self) -> TokenvalueW<'_, TransCtrlSpec> {
        TokenvalueW::new(self, 11)
    }
    ///Bits 12:20 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt.
    #[inline(always)]
    pub fn wrtrancnt(&mut self) -> WrtrancntW<'_, TransCtrlSpec> {
        WrtrancntW::new(self, 12)
    }
    ///Bit 21 - Token transfer enable (Master mode only) Append a one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token
    #[inline(always)]
    pub fn tokenen(&mut self) -> TokenenW<'_, TransCtrlSpec> {
        TokenenW::new(self, 21)
    }
    ///Bits 22:23 - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved
    #[inline(always)]
    pub fn dualquad(&mut self) -> DualquadW<'_, TransCtrlSpec> {
        DualquadW::new(self, 22)
    }
    ///Bits 24:27 - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved
    #[inline(always)]
    pub fn transmode(&mut self) -> TransmodeW<'_, TransCtrlSpec> {
        TransmodeW::new(self, 24)
    }
    ///Bit 28 - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad).
    #[inline(always)]
    pub fn addrfmt(&mut self) -> AddrfmtW<'_, TransCtrlSpec> {
        AddrfmtW::new(self, 28)
    }
    ///Bit 29 - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase
    #[inline(always)]
    pub fn addren(&mut self) -> AddrenW<'_, TransCtrlSpec> {
        AddrenW::new(self, 29)
    }
    ///Bit 30 - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase
    #[inline(always)]
    pub fn cmden(&mut self) -> CmdenW<'_, TransCtrlSpec> {
        CmdenW::new(self, 30)
    }
    ///Bit 31 - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0.
    #[inline(always)]
    pub fn slvdataonly(&mut self) -> SlvdataonlyW<'_, TransCtrlSpec> {
        SlvdataonlyW::new(self, 31)
    }
}
/**Transfer Control Register

You can [`read`](crate::Reg::read) this register and get [`trans_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trans_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TransCtrlSpec;
impl crate::RegisterSpec for TransCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`trans_ctrl::R`](R) reader structure
impl crate::Readable for TransCtrlSpec {}
///`write(|w| ..)` method takes [`trans_ctrl::W`](W) writer structure
impl crate::Writable for TransCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TransCtrl to value 0
impl crate::Resettable for TransCtrlSpec {}
