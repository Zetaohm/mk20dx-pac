#[doc = "Register `WUCNTR` reader"]
pub type R = crate::R<WucntrSpec>;
#[doc = "Field `WUCNT` reader - TouchSensing wake-up Channel 16bit counter value"]
pub type WucntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TouchSensing wake-up Channel 16bit counter value"]
    #[inline(always)]
    pub fn wucnt(&self) -> WucntR {
        WucntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Wake-Up Channel Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wucntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WucntrSpec;
impl crate::RegisterSpec for WucntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wucntr::R`](R) reader structure"]
impl crate::Readable for WucntrSpec {}
#[doc = "`reset()` method sets WUCNTR to value 0"]
impl crate::Resettable for WucntrSpec {
    const RESET_VALUE: u32 = 0;
}
