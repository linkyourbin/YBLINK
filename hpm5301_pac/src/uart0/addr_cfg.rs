///Register `ADDR_CFG` reader
pub type R = crate::R<AddrCfgSpec>;
///Register `ADDR_CFG` writer
pub type W = crate::W<AddrCfgSpec>;
///Field `ADDR0` reader - address 0 field.
pub type Addr0R = crate::FieldReader;
///Field `ADDR0` writer - address 0 field.
pub type Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADDR1` reader - address 1 fileld. in 9bit mode, this is the full address byte. For other mode(8/7/6/5bit), MSB should be set for address flag. If want address==0 to be matched at 8bit mode, should set addr1=0x80
pub type Addr1R = crate::FieldReader;
///Field `ADDR1` writer - address 1 fileld. in 9bit mode, this is the full address byte. For other mode(8/7/6/5bit), MSB should be set for address flag. If want address==0 to be matched at 8bit mode, should set addr1=0x80
pub type Addr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `A0_EN` reader - enable addr0 compare for the first character
pub type A0EnR = crate::BitReader;
///Field `A0_EN` writer - enable addr0 compare for the first character
pub type A0EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `A1_EN` reader - enable addr1 compare for the first character. If a1_en OR a0_en, then do not receive data if address not match. If ~a1_en AND ~a0_en, the receive all data like before. NOTE: should set idle_tmout_en if enable address match feature
pub type A1EnR = crate::BitReader;
///Field `A1_EN` writer - enable addr1 compare for the first character. If a1_en OR a0_en, then do not receive data if address not match. If ~a1_en AND ~a0_en, the receive all data like before. NOTE: should set idle_tmout_en if enable address match feature
pub type A1EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXEN_9BIT` reader - set to use 9bit mode for receiver, only valid if rxen_addr_msb is set
pub type Rxen9bitR = crate::BitReader;
///Field `RXEN_9BIT` writer - set to use 9bit mode for receiver, only valid if rxen_addr_msb is set
pub type Rxen9bitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXEN_ADDR_MSB` reader - set to use MSB as address flag at receiver(actually this is done by software set correct MSB in addr0/addr1). Clr to use first character as address. Only needed if enable address match feature
pub type RxenAddrMsbR = crate::BitReader;
///Field `RXEN_ADDR_MSB` writer - set to use MSB as address flag at receiver(actually this is done by software set correct MSB in addr0/addr1). Clr to use first character as address. Only needed if enable address match feature
pub type RxenAddrMsbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEN_9BIT` reader - set to use 9bit mode for transmitter, will set the MSB for the first character as address flag, keep 0 for others.
pub type Txen9bitR = crate::BitReader;
///Field `TXEN_9BIT` writer - set to use 9bit mode for transmitter, will set the MSB for the first character as address flag, keep 0 for others.
pub type Txen9bitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - address 0 field.
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - address 1 fileld. in 9bit mode, this is the full address byte. For other mode(8/7/6/5bit), MSB should be set for address flag. If want address==0 to be matched at 8bit mode, should set addr1=0x80
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - enable addr0 compare for the first character
    #[inline(always)]
    pub fn a0_en(&self) -> A0EnR {
        A0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - enable addr1 compare for the first character. If a1_en OR a0_en, then do not receive data if address not match. If ~a1_en AND ~a0_en, the receive all data like before. NOTE: should set idle_tmout_en if enable address match feature
    #[inline(always)]
    pub fn a1_en(&self) -> A1EnR {
        A1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - set to use 9bit mode for receiver, only valid if rxen_addr_msb is set
    #[inline(always)]
    pub fn rxen_9bit(&self) -> Rxen9bitR {
        Rxen9bitR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - set to use MSB as address flag at receiver(actually this is done by software set correct MSB in addr0/addr1). Clr to use first character as address. Only needed if enable address match feature
    #[inline(always)]
    pub fn rxen_addr_msb(&self) -> RxenAddrMsbR {
        RxenAddrMsbR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - set to use 9bit mode for transmitter, will set the MSB for the first character as address flag, keep 0 for others.
    #[inline(always)]
    pub fn txen_9bit(&self) -> Txen9bitR {
        Txen9bitR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - address 0 field.
    #[inline(always)]
    pub fn addr0(&mut self) -> Addr0W<'_, AddrCfgSpec> {
        Addr0W::new(self, 0)
    }
    ///Bits 8:15 - address 1 fileld. in 9bit mode, this is the full address byte. For other mode(8/7/6/5bit), MSB should be set for address flag. If want address==0 to be matched at 8bit mode, should set addr1=0x80
    #[inline(always)]
    pub fn addr1(&mut self) -> Addr1W<'_, AddrCfgSpec> {
        Addr1W::new(self, 8)
    }
    ///Bit 16 - enable addr0 compare for the first character
    #[inline(always)]
    pub fn a0_en(&mut self) -> A0EnW<'_, AddrCfgSpec> {
        A0EnW::new(self, 16)
    }
    ///Bit 17 - enable addr1 compare for the first character. If a1_en OR a0_en, then do not receive data if address not match. If ~a1_en AND ~a0_en, the receive all data like before. NOTE: should set idle_tmout_en if enable address match feature
    #[inline(always)]
    pub fn a1_en(&mut self) -> A1EnW<'_, AddrCfgSpec> {
        A1EnW::new(self, 17)
    }
    ///Bit 18 - set to use 9bit mode for receiver, only valid if rxen_addr_msb is set
    #[inline(always)]
    pub fn rxen_9bit(&mut self) -> Rxen9bitW<'_, AddrCfgSpec> {
        Rxen9bitW::new(self, 18)
    }
    ///Bit 19 - set to use MSB as address flag at receiver(actually this is done by software set correct MSB in addr0/addr1). Clr to use first character as address. Only needed if enable address match feature
    #[inline(always)]
    pub fn rxen_addr_msb(&mut self) -> RxenAddrMsbW<'_, AddrCfgSpec> {
        RxenAddrMsbW::new(self, 19)
    }
    ///Bit 20 - set to use 9bit mode for transmitter, will set the MSB for the first character as address flag, keep 0 for others.
    #[inline(always)]
    pub fn txen_9bit(&mut self) -> Txen9bitW<'_, AddrCfgSpec> {
        Txen9bitW::new(self, 20)
    }
}
/**address match config register

You can [`read`](crate::Reg::read) this register and get [`addr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AddrCfgSpec;
impl crate::RegisterSpec for AddrCfgSpec {
    type Ux = u32;
}
///`read()` method returns [`addr_cfg::R`](R) reader structure
impl crate::Readable for AddrCfgSpec {}
///`write(|w| ..)` method takes [`addr_cfg::W`](W) writer structure
impl crate::Writable for AddrCfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR_CFG to value 0
impl crate::Resettable for AddrCfgSpec {}
