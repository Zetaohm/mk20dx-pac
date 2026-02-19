#[doc = "Register `PRE` reader"]
pub type R = crate::R<PreSpec>;
#[doc = "Register `PRE` writer"]
pub type W = crate::W<PreSpec>;
#[doc = "Field `PREAMBLE` reader - CEA709.1-B Preamble Register"]
pub type PreambleR = crate::FieldReader;
#[doc = "Field `PREAMBLE` writer - CEA709.1-B Preamble Register"]
pub type PreambleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CEA709.1-B Preamble Register"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CEA709.1-B Preamble Register"]
    #[inline(always)]
    #[must_use]
    pub fn preamble(&mut self) -> PreambleW<PreSpec> {
        PreambleW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Preamble\n\nYou can [`read`](crate::Reg::read) this register and get [`pre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PreSpec;
impl crate::RegisterSpec for PreSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pre::R`](R) reader structure"]
impl crate::Readable for PreSpec {}
#[doc = "`write(|w| ..)` method takes [`pre::W`](W) writer structure"]
impl crate::Writable for PreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PRE to value 0"]
impl crate::Resettable for PreSpec {
    const RESET_VALUE: u8 = 0;
}
