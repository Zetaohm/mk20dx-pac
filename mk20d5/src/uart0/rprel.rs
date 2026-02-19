#[doc = "Register `RPREL` reader"]
pub type R = crate::R<RprelSpec>;
#[doc = "Field `RPREL` reader - Received preamble length"]
pub type RprelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received preamble length"]
    #[inline(always)]
    pub fn rprel(&self) -> RprelR {
        RprelR::new(self.bits)
    }
}
#[doc = "UART CEA709.1-B Received Preamble Length\n\nYou can [`read`](crate::Reg::read) this register and get [`rprel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RprelSpec;
impl crate::RegisterSpec for RprelSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rprel::R`](R) reader structure"]
impl crate::Readable for RprelSpec {}
#[doc = "`reset()` method sets RPREL to value 0"]
impl crate::Resettable for RprelSpec {
    const RESET_VALUE: u8 = 0;
}
