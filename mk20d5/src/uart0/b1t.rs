#[doc = "Register `B1T` reader"]
pub type R = crate::R<B1tSpec>;
#[doc = "Register `B1T` writer"]
pub type W = crate::W<B1tSpec>;
#[doc = "Field `B1T` reader - Beta1 Timer"]
pub type B1tR = crate::FieldReader;
#[doc = "Field `B1T` writer - Beta1 Timer"]
pub type B1tW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Beta1 Timer"]
    #[inline(always)]
    pub fn b1t(&self) -> B1tR {
        B1tR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Beta1 Timer"]
    #[inline(always)]
    #[must_use]
    pub fn b1t(&mut self) -> B1tW<B1tSpec> {
        B1tW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Beta1 Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`b1t::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b1t::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B1tSpec;
impl crate::RegisterSpec for B1tSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`b1t::R`](R) reader structure"]
impl crate::Readable for B1tSpec {}
#[doc = "`write(|w| ..)` method takes [`b1t::W`](W) writer structure"]
impl crate::Writable for B1tSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets B1T to value 0"]
impl crate::Resettable for B1tSpec {
    const RESET_VALUE: u8 = 0;
}
