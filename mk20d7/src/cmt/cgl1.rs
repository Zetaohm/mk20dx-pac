#[doc = "Register `CGL1` reader"]
pub type R = crate::R<Cgl1Spec>;
#[doc = "Register `CGL1` writer"]
pub type W = crate::W<Cgl1Spec>;
#[doc = "Field `PL` reader - Primary Carrier Low Time Data Value"]
pub type PlR = crate::FieldReader;
#[doc = "Field `PL` writer - Primary Carrier Low Time Data Value"]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Primary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Primary Carrier Low Time Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PlW<Cgl1Spec> {
        PlW::new(self, 0)
    }
}
#[doc = "CMT Carrier Generator Low Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cgl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgl1Spec;
impl crate::RegisterSpec for Cgl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cgl1::R`](R) reader structure"]
impl crate::Readable for Cgl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cgl1::W`](W) writer structure"]
impl crate::Writable for Cgl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CGL1 to value 0"]
impl crate::Resettable for Cgl1Spec {
    const RESET_VALUE: u8 = 0;
}
