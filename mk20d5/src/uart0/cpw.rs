#[doc = "Register `CPW` reader"]
pub type R = crate::R<CpwSpec>;
#[doc = "Register `CPW` writer"]
pub type W = crate::W<CpwSpec>;
#[doc = "Field `CPW` reader - CEA709.1-B CPW register"]
pub type CpwR = crate::FieldReader;
#[doc = "Field `CPW` writer - CEA709.1-B CPW register"]
pub type CpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CEA709.1-B CPW register"]
    #[inline(always)]
    pub fn cpw(&self) -> CpwR {
        CpwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CEA709.1-B CPW register"]
    #[inline(always)]
    #[must_use]
    pub fn cpw(&mut self) -> CpwW<CpwSpec> {
        CpwW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Collision Pulse Width\n\nYou can [`read`](crate::Reg::read) this register and get [`cpw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpwSpec;
impl crate::RegisterSpec for CpwSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpw::R`](R) reader structure"]
impl crate::Readable for CpwSpec {}
#[doc = "`write(|w| ..)` method takes [`cpw::W`](W) writer structure"]
impl crate::Writable for CpwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CPW to value 0"]
impl crate::Resettable for CpwSpec {
    const RESET_VALUE: u8 = 0;
}
