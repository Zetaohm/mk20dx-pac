#[doc = "Register `CRCH` reader"]
pub type R = crate::R<CrcCrchSpec>;
#[doc = "Register `CRCH` writer"]
pub type W = crate::W<CrcCrchSpec>;
#[doc = "Field `CRCH` reader - CRCH stores the high 16 bits of the 16/32 bit CRC"]
pub type CrchR = crate::FieldReader<u16>;
#[doc = "Field `CRCH` writer - CRCH stores the high 16 bits of the 16/32 bit CRC"]
pub type CrchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRCH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crch(&self) -> CrchR {
        CrchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRCH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crch(&mut self) -> CrchW<CrcCrchSpec> {
        CrchW::new(self, 0)
    }
}
#[doc = "CRC_CRCH register.\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_crch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_crch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCrchSpec;
impl crate::RegisterSpec for CrcCrchSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc_crch::R`](R) reader structure"]
impl crate::Readable for CrcCrchSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_crch::W`](W) writer structure"]
impl crate::Writable for CrcCrchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRCH to value 0xffff"]
impl crate::Resettable for CrcCrchSpec {
    const RESET_VALUE: u16 = 0xffff;
}
