#[doc = "Register `CGL2` reader"]
pub type R = crate::R<Cgl2Spec>;
#[doc = "Register `CGL2` writer"]
pub type W = crate::W<Cgl2Spec>;
#[doc = "Field `SL` reader - Secondary Carrier Low Time Data Value"]
pub type SlR = crate::FieldReader;
#[doc = "Field `SL` writer - Secondary Carrier Low Time Data Value"]
pub type SlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Secondary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn sl(&self) -> SlR {
        SlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secondary Carrier Low Time Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn sl(&mut self) -> SlW<Cgl2Spec> {
        SlW::new(self, 0)
    }
}
#[doc = "CMT Carrier Generator Low Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cgl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgl2Spec;
impl crate::RegisterSpec for Cgl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cgl2::R`](R) reader structure"]
impl crate::Readable for Cgl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cgl2::W`](W) writer structure"]
impl crate::Writable for Cgl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CGL2 to value 0"]
impl crate::Resettable for Cgl2Spec {
    const RESET_VALUE: u8 = 0;
}
