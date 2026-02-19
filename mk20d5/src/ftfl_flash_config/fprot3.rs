#[doc = "Register `FPROT3` reader"]
pub type R = crate::R<Fprot3Spec>;
#[doc = "Field `PROT` reader - P-Flash Region Protect"]
pub type ProtR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - P-Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(self.bits)
    }
}
#[doc = "Non-volatile P-Flash Protection 1 - Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fprot3Spec;
impl crate::RegisterSpec for Fprot3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fprot3::R`](R) reader structure"]
impl crate::Readable for Fprot3Spec {}
#[doc = "`reset()` method sets FPROT3 to value 0xff"]
impl crate::Resettable for Fprot3Spec {
    const RESET_VALUE: u8 = 0xff;
}
