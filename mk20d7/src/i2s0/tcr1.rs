#[doc = "Register `TCR1` reader"]
pub type R = crate::R<Tcr1Spec>;
#[doc = "Register `TCR1` writer"]
pub type W = crate::W<Tcr1Spec>;
#[doc = "Field `TFW` reader - Transmit FIFO watermark"]
pub type TfwR = crate::FieldReader;
#[doc = "Field `TFW` writer - Transmit FIFO watermark"]
pub type TfwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO watermark"]
    #[inline(always)]
    pub fn tfw(&self) -> TfwR {
        TfwR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit FIFO watermark"]
    #[inline(always)]
    #[must_use]
    pub fn tfw(&mut self) -> TfwW<Tcr1Spec> {
        TfwW::new(self, 0)
    }
}
#[doc = "SAI Transmit Configuration 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr1Spec;
impl crate::RegisterSpec for Tcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr1::R`](R) reader structure"]
impl crate::Readable for Tcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tcr1::W`](W) writer structure"]
impl crate::Writable for Tcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR1 to value 0"]
impl crate::Resettable for Tcr1Spec {
    const RESET_VALUE: u32 = 0;
}
