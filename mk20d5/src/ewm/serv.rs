#[doc = "Register `SERV` writer"]
pub type W = crate::W<ServSpec>;
#[doc = "Field `SERVICE` writer - The EWM service mechanism requires the CPU to write two values to the SERV register: a first data byte of 0xB4, followed by a second data byte of 0x2C"]
pub type ServiceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - The EWM service mechanism requires the CPU to write two values to the SERV register: a first data byte of 0xB4, followed by a second data byte of 0x2C"]
    #[inline(always)]
    #[must_use]
    pub fn service(&mut self) -> ServiceW<ServSpec> {
        ServiceW::new(self, 0)
    }
}
#[doc = "Service Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ServSpec;
impl crate::RegisterSpec for ServSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`serv::W`](W) writer structure"]
impl crate::Writable for ServSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SERV to value 0"]
impl crate::Resettable for ServSpec {
    const RESET_VALUE: u8 = 0;
}
