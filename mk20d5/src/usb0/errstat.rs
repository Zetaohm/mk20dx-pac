#[doc = "Register `ERRSTAT` reader"]
pub type R = crate::R<ErrstatSpec>;
#[doc = "Register `ERRSTAT` writer"]
pub type W = crate::W<ErrstatSpec>;
#[doc = "Field `PIDERR` reader - This bit is set when the PID check field fails."]
pub type PiderrR = crate::BitReader;
#[doc = "Field `PIDERR` writer - This bit is set when the PID check field fails."]
pub type PiderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC5EOF` reader - This error interrupt has two functions"]
pub type Crc5eofR = crate::BitReader;
#[doc = "Field `CRC5EOF` writer - This error interrupt has two functions"]
pub type Crc5eofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16` reader - This bit is set when a data packet is rejected due to a CRC16 error."]
pub type Crc16R = crate::BitReader;
#[doc = "Field `CRC16` writer - This bit is set when a data packet is rejected due to a CRC16 error."]
pub type Crc16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFN8` reader - This bit is set if the data field received was not 8 bits in length"]
pub type Dfn8R = crate::BitReader;
#[doc = "Field `DFN8` writer - This bit is set if the data field received was not 8 bits in length"]
pub type Dfn8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTOERR` reader - This bit is set when a bus turnaround timeout error occurs"]
pub type BtoerrR = crate::BitReader;
#[doc = "Field `BTOERR` writer - This bit is set when a bus turnaround timeout error occurs"]
pub type BtoerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAERR` reader - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
pub type DmaerrR = crate::BitReader;
#[doc = "Field `DMAERR` writer - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
pub type DmaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTSERR` reader - This bit is set when a bit stuff error is detected"]
pub type BtserrR = crate::BitReader;
#[doc = "Field `BTSERR` writer - This bit is set when a bit stuff error is detected"]
pub type BtserrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is set when the PID check field fails."]
    #[inline(always)]
    pub fn piderr(&self) -> PiderrR {
        PiderrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    pub fn crc5eof(&self) -> Crc5eofR {
        Crc5eofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error."]
    #[inline(always)]
    pub fn crc16(&self) -> Crc16R {
        Crc16R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length"]
    #[inline(always)]
    pub fn dfn8(&self) -> Dfn8R {
        Dfn8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs"]
    #[inline(always)]
    pub fn btoerr(&self) -> BtoerrR {
        BtoerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DmaerrR {
        DmaerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is set when a bit stuff error is detected"]
    #[inline(always)]
    pub fn btserr(&self) -> BtserrR {
        BtserrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when the PID check field fails."]
    #[inline(always)]
    #[must_use]
    pub fn piderr(&mut self) -> PiderrW<ErrstatSpec> {
        PiderrW::new(self, 0)
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    #[must_use]
    pub fn crc5eof(&mut self) -> Crc5eofW<ErrstatSpec> {
        Crc5eofW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error."]
    #[inline(always)]
    #[must_use]
    pub fn crc16(&mut self) -> Crc16W<ErrstatSpec> {
        Crc16W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length"]
    #[inline(always)]
    #[must_use]
    pub fn dfn8(&mut self) -> Dfn8W<ErrstatSpec> {
        Dfn8W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs"]
    #[inline(always)]
    #[must_use]
    pub fn btoerr(&mut self) -> BtoerrW<ErrstatSpec> {
        BtoerrW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
    #[inline(always)]
    #[must_use]
    pub fn dmaerr(&mut self) -> DmaerrW<ErrstatSpec> {
        DmaerrW::new(self, 5)
    }
    #[doc = "Bit 7 - This bit is set when a bit stuff error is detected"]
    #[inline(always)]
    #[must_use]
    pub fn btserr(&mut self) -> BtserrW<ErrstatSpec> {
        BtserrW::new(self, 7)
    }
}
#[doc = "Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrstatSpec;
impl crate::RegisterSpec for ErrstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`errstat::R`](R) reader structure"]
impl crate::Readable for ErrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`errstat::W`](W) writer structure"]
impl crate::Writable for ErrstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ERRSTAT to value 0"]
impl crate::Resettable for ErrstatSpec {
    const RESET_VALUE: u8 = 0;
}
