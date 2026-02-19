#[doc = "Register `GPOLYLL` reader"]
pub type R = crate::R<CrcGpolyllSpec>;
#[doc = "Register `GPOLYLL` writer"]
pub type W = crate::W<CrcGpolyllSpec>;
#[doc = "Field `GPOLYLL` reader - POLYLL stores the first 8 bits of the 32 bit CRC"]
pub type GpolyllR = crate::FieldReader;
#[doc = "Field `GPOLYLL` writer - POLYLL stores the first 8 bits of the 32 bit CRC"]
pub type GpolyllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - POLYLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyll(&self) -> GpolyllR {
        GpolyllR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn gpolyll(&mut self) -> GpolyllW<CrcGpolyllSpec> {
        GpolyllW::new(self, 0)
    }
}
#[doc = "CRC_GPOLYLL register.\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_gpolyll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_gpolyll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcGpolyllSpec;
impl crate::RegisterSpec for CrcGpolyllSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crc_gpolyll::R`](R) reader structure"]
impl crate::Readable for CrcGpolyllSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_gpolyll::W`](W) writer structure"]
impl crate::Writable for CrcGpolyllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GPOLYLL to value 0xff"]
impl crate::Resettable for CrcGpolyllSpec {
    const RESET_VALUE: u8 = 0xff;
}
