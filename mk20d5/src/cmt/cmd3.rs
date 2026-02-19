#[doc = "Register `CMD3` reader"]
pub type R = crate::R<Cmd3Spec>;
#[doc = "Register `CMD3` writer"]
pub type W = crate::W<Cmd3Spec>;
#[doc = "Field `SB` reader - These bits control the upper space periods of the modulator for all modes."]
pub type SbR = crate::FieldReader;
#[doc = "Field `SB` writer - These bits control the upper space periods of the modulator for all modes."]
pub type SbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits control the upper space periods of the modulator for all modes."]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits control the upper space periods of the modulator for all modes."]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SbW<Cmd3Spec> {
        SbW::new(self, 0)
    }
}
#[doc = "CMT Modulator Data Register Space High\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmd3Spec;
impl crate::RegisterSpec for Cmd3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cmd3::R`](R) reader structure"]
impl crate::Readable for Cmd3Spec {}
#[doc = "`write(|w| ..)` method takes [`cmd3::W`](W) writer structure"]
impl crate::Writable for Cmd3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CMD3 to value 0"]
impl crate::Resettable for Cmd3Spec {
    const RESET_VALUE: u8 = 0;
}
