#[doc = "Register `GPOLYLU` reader"]
pub type R = crate::R<GpolyluSpec>;
#[doc = "Register `GPOLYLU` writer"]
pub type W = crate::W<GpolyluSpec>;
#[doc = "Field `GPOLYLU` reader - POLYLL stores the second 8 bits of the 32 bit CRC"]
pub type GpolyluR = crate::FieldReader;
#[doc = "Field `GPOLYLU` writer - POLYLL stores the second 8 bits of the 32 bit CRC"]
pub type GpolyluW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolylu(&self) -> GpolyluR {
        GpolyluR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn gpolylu(&mut self) -> GpolyluW<GpolyluSpec> {
        GpolyluW::new(self, 0)
    }
}
#[doc = "CRC_GPOLYLU register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpolylu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpolylu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpolyluSpec;
impl crate::RegisterSpec for GpolyluSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gpolylu::R`](R) reader structure"]
impl crate::Readable for GpolyluSpec {}
#[doc = "`write(|w| ..)` method takes [`gpolylu::W`](W) writer structure"]
impl crate::Writable for GpolyluSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GPOLYLU to value 0xff"]
impl crate::Resettable for GpolyluSpec {
    const RESET_VALUE: u8 = 0xff;
}
