#[doc = "Register `CRCL` reader"]
pub type R = crate::R<CrcCrclSpec>;
#[doc = "Register `CRCL` writer"]
pub type W = crate::W<CrcCrclSpec>;
#[doc = "Field `CRCL` reader - CRCL stores the lower 16 bits of the 16/32 bit CRC"]
pub type CrclR = crate::FieldReader<u16>;
#[doc = "Field `CRCL` writer - CRCL stores the lower 16 bits of the 16/32 bit CRC"]
pub type CrclW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRCL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crcl(&self) -> CrclR {
        CrclR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRCL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcl(&mut self) -> CrclW<CrcCrclSpec> {
        CrclW::new(self, 0)
    }
}
#[doc = "CRC_CRCL register.\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_crcl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_crcl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCrclSpec;
impl crate::RegisterSpec for CrcCrclSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc_crcl::R`](R) reader structure"]
impl crate::Readable for CrcCrclSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_crcl::W`](W) writer structure"]
impl crate::Writable for CrcCrclSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRCL to value 0xffff"]
impl crate::Resettable for CrcCrclSpec {
    const RESET_VALUE: u16 = 0xffff;
}
