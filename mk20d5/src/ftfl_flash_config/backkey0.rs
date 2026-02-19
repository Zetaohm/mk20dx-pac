#[doc = "Register `BACKKEY0` reader"]
pub type R = crate::R<Backkey0Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey0Spec;
impl crate::RegisterSpec for Backkey0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey0::R`](R) reader structure"]
impl crate::Readable for Backkey0Spec {}
#[doc = "`reset()` method sets BACKKEY0 to value 0xff"]
impl crate::Resettable for Backkey0Spec {
    const RESET_VALUE: u8 = 0xff;
}
