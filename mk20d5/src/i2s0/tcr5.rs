#[doc = "Register `TCR5` reader"]
pub type R = crate::R<Tcr5Spec>;
#[doc = "Register `TCR5` writer"]
pub type W = crate::W<Tcr5Spec>;
#[doc = "Field `FBT` reader - First bit shifted"]
pub type FbtR = crate::FieldReader;
#[doc = "Field `FBT` writer - First bit shifted"]
pub type FbtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W0W` reader - Word 0 width"]
pub type W0wR = crate::FieldReader;
#[doc = "Field `W0W` writer - Word 0 width"]
pub type W0wW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WNW` reader - Word N width"]
pub type WnwR = crate::FieldReader;
#[doc = "Field `WNW` writer - Word N width"]
pub type WnwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 8:12 - First bit shifted"]
    #[inline(always)]
    pub fn fbt(&self) -> FbtR {
        FbtR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Word 0 width"]
    #[inline(always)]
    pub fn w0w(&self) -> W0wR {
        W0wR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Word N width"]
    #[inline(always)]
    pub fn wnw(&self) -> WnwR {
        WnwR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - First bit shifted"]
    #[inline(always)]
    #[must_use]
    pub fn fbt(&mut self) -> FbtW<Tcr5Spec> {
        FbtW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Word 0 width"]
    #[inline(always)]
    #[must_use]
    pub fn w0w(&mut self) -> W0wW<Tcr5Spec> {
        W0wW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Word N width"]
    #[inline(always)]
    #[must_use]
    pub fn wnw(&mut self) -> WnwW<Tcr5Spec> {
        WnwW::new(self, 24)
    }
}
#[doc = "SAI Transmit Configuration 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr5Spec;
impl crate::RegisterSpec for Tcr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr5::R`](R) reader structure"]
impl crate::Readable for Tcr5Spec {}
#[doc = "`write(|w| ..)` method takes [`tcr5::W`](W) writer structure"]
impl crate::Writable for Tcr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR5 to value 0"]
impl crate::Resettable for Tcr5Spec {
    const RESET_VALUE: u32 = 0;
}
