#[doc = "Register `BACKKEY4` reader"]
pub type R = crate::R<Backkey4Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey4Spec;
impl crate::RegisterSpec for Backkey4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey4::R`](R) reader structure"]
impl crate::Readable for Backkey4Spec {}
#[doc = "`reset()` method sets BACKKEY4 to value 0xff"]
impl crate::Resettable for Backkey4Spec {
    const RESET_VALUE: u8 = 0xff;
}
