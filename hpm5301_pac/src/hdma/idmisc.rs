///Register `IDMisc` reader
pub type R = crate::R<IdmiscSpec>;
///Register `IDMisc` writer
pub type W = crate::W<IdmiscSpec>;
///Field `CURCHAN` reader - current channel in used
pub type CurchanR = crate::FieldReader;
///Field `DMASTATE` reader - DMA state machine localparam ST_IDLE = 3'b000; localparam ST_READ = 3'b001; localparam ST_READ_ACK = 3'b010; localparam ST_WRITE = 3'b011; localparam ST_WRITE_ACK = 3'b100; localparam ST_LL = 3'b101; localparam ST_END = 3'b110; localparam ST_END_WAIT = 3'b111;
pub type DmastateR = crate::FieldReader;
impl R {
    ///Bits 8:12 - current channel in used
    #[inline(always)]
    pub fn curchan(&self) -> CurchanR {
        CurchanR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 13:15 - DMA state machine localparam ST_IDLE = 3'b000; localparam ST_READ = 3'b001; localparam ST_READ_ACK = 3'b010; localparam ST_WRITE = 3'b011; localparam ST_WRITE_ACK = 3'b100; localparam ST_LL = 3'b101; localparam ST_END = 3'b110; localparam ST_END_WAIT = 3'b111;
    #[inline(always)]
    pub fn dmastate(&self) -> DmastateR {
        DmastateR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {}
/**ID Misc

You can [`read`](crate::Reg::read) this register and get [`idmisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IdmiscSpec;
impl crate::RegisterSpec for IdmiscSpec {
    type Ux = u32;
}
///`read()` method returns [`idmisc::R`](R) reader structure
impl crate::Readable for IdmiscSpec {}
///`write(|w| ..)` method takes [`idmisc::W`](W) writer structure
impl crate::Writable for IdmiscSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMisc to value 0
impl crate::Resettable for IdmiscSpec {}
