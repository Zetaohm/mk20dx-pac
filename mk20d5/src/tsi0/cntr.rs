#[doc = "Register `CNTR%s` reader"]
pub type R = crate::R<CntrSpec>;
#[doc = "Field `CTN1` reader - TouchSensing Channel n-1 16-bit counter value"]
pub type Ctn1R = crate::FieldReader<u16>;
#[doc = "Field `CTN` reader - TouchSensing Channel n 16-bit counter value"]
pub type CtnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TouchSensing Channel n-1 16-bit counter value"]
    #[inline(always)]
    pub fn ctn1(&self) -> Ctn1R {
        Ctn1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TouchSensing Channel n 16-bit counter value"]
    #[inline(always)]
    pub fn ctn(&self) -> CtnR {
        CtnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrSpec;
impl crate::RegisterSpec for CntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CntrSpec {}
#[doc = "`reset()` method sets CNTR%s to value 0"]
impl crate::Resettable for CntrSpec {
    const RESET_VALUE: u32 = 0;
}
