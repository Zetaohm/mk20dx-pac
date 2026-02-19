#[doc = "Register `C3` reader"]
pub type R = crate::R<C3Spec>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3Spec>;
#[doc = "Field `SCTRIM` reader - Slow Internal Reference Clock Trim Setting"]
pub type SctrimR = crate::FieldReader;
#[doc = "Field `SCTRIM` writer - Slow Internal Reference Clock Trim Setting"]
pub type SctrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Slow Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn sctrim(&self) -> SctrimR {
        SctrimR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slow Internal Reference Clock Trim Setting"]
    #[inline(always)]
    #[must_use]
    pub fn sctrim(&mut self) -> SctrimW<C3Spec> {
        SctrimW::new(self, 0)
    }
}
#[doc = "MCG Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3Spec;
impl crate::RegisterSpec for C3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c3::R`](R) reader structure"]
impl crate::Readable for C3Spec {}
#[doc = "`write(|w| ..)` method takes [`c3::W`](W) writer structure"]
impl crate::Writable for C3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C3 to value 0"]
impl crate::Resettable for C3Spec {
    const RESET_VALUE: u8 = 0;
}
