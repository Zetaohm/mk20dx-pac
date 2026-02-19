#[doc = "Register `UIDH` reader"]
pub type R = crate::R<UidhSpec>;
#[doc = "Field `UID` reader - Unique Identification"]
pub type UidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid(&self) -> UidR {
        UidR::new(self.bits)
    }
}
#[doc = "Unique Identification Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`uidh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UidhSpec;
impl crate::RegisterSpec for UidhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uidh::R`](R) reader structure"]
impl crate::Readable for UidhSpec {}
#[doc = "`reset()` method sets UIDH to value 0"]
impl crate::Resettable for UidhSpec {
    const RESET_VALUE: u32 = 0;
}
