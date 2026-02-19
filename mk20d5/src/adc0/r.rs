#[doc = "Register `R%s` reader"]
pub type R = crate::R<RSpec>;
#[doc = "Field `D` reader - Data result"]
pub type DR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data result"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC data result register\n\nYou can [`read`](crate::Reg::read) this register and get [`r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSpec;
impl crate::RegisterSpec for RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r::R`](R) reader structure"]
impl crate::Readable for RSpec {}
#[doc = "`reset()` method sets R%s to value 0"]
impl crate::Resettable for RSpec {
    const RESET_VALUE: u32 = 0;
}
