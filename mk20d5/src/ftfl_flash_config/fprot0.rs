#[doc = "Register `FPROT0` reader"]
pub type R = crate::R<Fprot0Spec>;
#[doc = "Field `PROT` reader - P-Flash Region Protect"]
pub type ProtR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - P-Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(self.bits)
    }
}
#[doc = "Non-volatile P-Flash Protection 0 - High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fprot0Spec;
impl crate::RegisterSpec for Fprot0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fprot0::R`](R) reader structure"]
impl crate::Readable for Fprot0Spec {}
#[doc = "`reset()` method sets FPROT0 to value 0xff"]
impl crate::Resettable for Fprot0Spec {
    const RESET_VALUE: u8 = 0xff;
}
