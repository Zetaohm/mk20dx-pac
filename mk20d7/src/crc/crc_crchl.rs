#[doc = "Register `CRCHL` reader"]
pub type R = crate::R<CrcCrchlSpec>;
#[doc = "Register `CRCHL` writer"]
pub type W = crate::W<CrcCrchlSpec>;
#[doc = "Field `CRCHL` reader - CRCHL stores the third 8 bits of the 32 bit CRC"]
pub type CrchlR = crate::FieldReader;
#[doc = "Field `CRCHL` writer - CRCHL stores the third 8 bits of the 32 bit CRC"]
pub type CrchlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CRCHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchl(&self) -> CrchlR {
        CrchlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crchl(&mut self) -> CrchlW<CrcCrchlSpec> {
        CrchlW::new(self, 0)
    }
}
#[doc = "CRC_CRCHL register.\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_crchl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_crchl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCrchlSpec;
impl crate::RegisterSpec for CrcCrchlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crc_crchl::R`](R) reader structure"]
impl crate::Readable for CrcCrchlSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_crchl::W`](W) writer structure"]
impl crate::Writable for CrcCrchlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CRCHL to value 0xff"]
impl crate::Resettable for CrcCrchlSpec {
    const RESET_VALUE: u8 = 0xff;
}
