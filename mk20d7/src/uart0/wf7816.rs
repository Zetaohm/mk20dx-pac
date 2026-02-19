#[doc = "Register `WF7816` reader"]
pub type R = crate::R<Wf7816Spec>;
#[doc = "Register `WF7816` writer"]
pub type W = crate::W<Wf7816Spec>;
#[doc = "Field `GTFD` reader - FD Multiplier"]
pub type GtfdR = crate::FieldReader;
#[doc = "Field `GTFD` writer - FD Multiplier"]
pub type GtfdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FD Multiplier"]
    #[inline(always)]
    pub fn gtfd(&self) -> GtfdR {
        GtfdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - FD Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn gtfd(&mut self) -> GtfdW<Wf7816Spec> {
        GtfdW::new(self, 0)
    }
}
#[doc = "UART 7816 Wait FD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wf7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf7816Spec;
impl crate::RegisterSpec for Wf7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf7816::R`](R) reader structure"]
impl crate::Readable for Wf7816Spec {}
#[doc = "`write(|w| ..)` method takes [`wf7816::W`](W) writer structure"]
impl crate::Writable for Wf7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF7816 to value 0x01"]
impl crate::Resettable for Wf7816Spec {
    const RESET_VALUE: u8 = 0x01;
}
