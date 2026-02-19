#[doc = "Register `FPROT1` reader"]
pub type R = crate::R<Fprot1Spec>;
#[doc = "Field `PROT` reader - P-Flash Region Protect"]
pub type ProtR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - P-Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(self.bits)
    }
}
#[doc = "Non-volatile P-Flash Protection 0 - Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fprot1Spec;
impl crate::RegisterSpec for Fprot1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fprot1::R`](R) reader structure"]
impl crate::Readable for Fprot1Spec {}
#[doc = "`reset()` method sets FPROT1 to value 0xff"]
impl crate::Resettable for Fprot1Spec {
    const RESET_VALUE: u8 = 0xff;
}
