#[doc = "Register `CGH2` reader"]
pub type R = crate::R<Cgh2Spec>;
#[doc = "Register `CGH2` writer"]
pub type W = crate::W<Cgh2Spec>;
#[doc = "Field `SH` reader - Secondary Carrier High Time Data Value"]
pub type ShR = crate::FieldReader;
#[doc = "Field `SH` writer - Secondary Carrier High Time Data Value"]
pub type ShW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Secondary Carrier High Time Data Value"]
    #[inline(always)]
    pub fn sh(&self) -> ShR {
        ShR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secondary Carrier High Time Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn sh(&mut self) -> ShW<Cgh2Spec> {
        ShW::new(self, 0)
    }
}
#[doc = "CMT Carrier Generator High Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cgh2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgh2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgh2Spec;
impl crate::RegisterSpec for Cgh2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cgh2::R`](R) reader structure"]
impl crate::Readable for Cgh2Spec {}
#[doc = "`write(|w| ..)` method takes [`cgh2::W`](W) writer structure"]
impl crate::Writable for Cgh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CGH2 to value 0"]
impl crate::Resettable for Cgh2Spec {
    const RESET_VALUE: u8 = 0;
}
