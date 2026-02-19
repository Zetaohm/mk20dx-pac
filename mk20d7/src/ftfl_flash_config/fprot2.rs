#[doc = "Register `FPROT2` reader"]
pub type R = crate::R<Fprot2Spec>;
#[doc = "Field `PROT` reader - P-Flash Region Protect"]
pub type ProtR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - P-Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(self.bits)
    }
}
#[doc = "Non-volatile P-Flash Protection 1 - High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fprot2Spec;
impl crate::RegisterSpec for Fprot2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fprot2::R`](R) reader structure"]
impl crate::Readable for Fprot2Spec {}
#[doc = "`reset()` method sets FPROT2 to value 0xff"]
impl crate::Resettable for Fprot2Spec {
    const RESET_VALUE: u8 = 0xff;
}
