#[doc = "Register `TL7816` reader"]
pub type R = crate::R<Tl7816Spec>;
#[doc = "Register `TL7816` writer"]
pub type W = crate::W<Tl7816Spec>;
#[doc = "Field `TLEN` reader - Transmit Length"]
pub type TlenR = crate::FieldReader;
#[doc = "Field `TLEN` writer - Transmit Length"]
pub type TlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Length"]
    #[inline(always)]
    pub fn tlen(&self) -> TlenR {
        TlenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TlenW<Tl7816Spec> {
        TlenW::new(self, 0)
    }
}
#[doc = "UART 7816 Transmit Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tl7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tl7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tl7816Spec;
impl crate::RegisterSpec for Tl7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tl7816::R`](R) reader structure"]
impl crate::Readable for Tl7816Spec {}
#[doc = "`write(|w| ..)` method takes [`tl7816::W`](W) writer structure"]
impl crate::Writable for Tl7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TL7816 to value 0"]
impl crate::Resettable for Tl7816Spec {
    const RESET_VALUE: u8 = 0;
}
