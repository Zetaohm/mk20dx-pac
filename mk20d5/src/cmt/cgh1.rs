#[doc = "Register `CGH1` reader"]
pub type R = crate::R<Cgh1Spec>;
#[doc = "Register `CGH1` writer"]
pub type W = crate::W<Cgh1Spec>;
#[doc = "Field `PH` reader - Primary Carrier High Time Data Value"]
pub type PhR = crate::FieldReader;
#[doc = "Field `PH` writer - Primary Carrier High Time Data Value"]
pub type PhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Primary Carrier High Time Data Value"]
    #[inline(always)]
    pub fn ph(&self) -> PhR {
        PhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Primary Carrier High Time Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn ph(&mut self) -> PhW<Cgh1Spec> {
        PhW::new(self, 0)
    }
}
#[doc = "CMT Carrier Generator High Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cgh1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgh1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgh1Spec;
impl crate::RegisterSpec for Cgh1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cgh1::R`](R) reader structure"]
impl crate::Readable for Cgh1Spec {}
#[doc = "`write(|w| ..)` method takes [`cgh1::W`](W) writer structure"]
impl crate::Writable for Cgh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CGH1 to value 0"]
impl crate::Resettable for Cgh1Spec {
    const RESET_VALUE: u8 = 0;
}
