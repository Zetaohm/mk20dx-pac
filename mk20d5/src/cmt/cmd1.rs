#[doc = "Register `CMD1` reader"]
pub type R = crate::R<Cmd1Spec>;
#[doc = "Register `CMD1` writer"]
pub type W = crate::W<Cmd1Spec>;
#[doc = "Field `MB` reader - These bits control the upper mark periods of the modulator for all modes."]
pub type MbR = crate::FieldReader;
#[doc = "Field `MB` writer - These bits control the upper mark periods of the modulator for all modes."]
pub type MbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits control the upper mark periods of the modulator for all modes."]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits control the upper mark periods of the modulator for all modes."]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MbW<Cmd1Spec> {
        MbW::new(self, 0)
    }
}
#[doc = "CMT Modulator Data Register Mark High\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmd1Spec;
impl crate::RegisterSpec for Cmd1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cmd1::R`](R) reader structure"]
impl crate::Readable for Cmd1Spec {}
#[doc = "`write(|w| ..)` method takes [`cmd1::W`](W) writer structure"]
impl crate::Writable for Cmd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CMD1 to value 0"]
impl crate::Resettable for Cmd1Spec {
    const RESET_VALUE: u8 = 0;
}
