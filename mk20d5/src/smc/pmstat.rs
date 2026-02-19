#[doc = "Register `PMSTAT` reader"]
pub type R = crate::R<PmstatSpec>;
#[doc = "Field `PMSTAT` reader - When debug is enabled, the PMSTAT will not update to STOP or VLPS"]
pub type PmstatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - When debug is enabled, the PMSTAT will not update to STOP or VLPS"]
    #[inline(always)]
    pub fn pmstat(&self) -> PmstatR {
        PmstatR::new(self.bits & 0x7f)
    }
}
#[doc = "Power Mode Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmstatSpec;
impl crate::RegisterSpec for PmstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmstat::R`](R) reader structure"]
impl crate::Readable for PmstatSpec {}
#[doc = "`reset()` method sets PMSTAT to value 0x01"]
impl crate::Resettable for PmstatSpec {
    const RESET_VALUE: u8 = 0x01;
}
