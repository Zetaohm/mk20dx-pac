#[doc = "Register `A1` reader"]
pub type R = crate::R<A1Spec>;
#[doc = "Register `A1` writer"]
pub type W = crate::W<A1Spec>;
#[doc = "Field `AD` reader - Address"]
pub type AdR = crate::FieldReader;
#[doc = "Field `AD` writer - Address"]
pub type AdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    pub fn ad(&self) -> AdR {
        AdR::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn ad(&mut self) -> AdW<A1Spec> {
        AdW::new(self, 1)
    }
}
#[doc = "I2C Address Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`a1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A1Spec;
impl crate::RegisterSpec for A1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a1::R`](R) reader structure"]
impl crate::Readable for A1Spec {}
#[doc = "`write(|w| ..)` method takes [`a1::W`](W) writer structure"]
impl crate::Writable for A1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A1 to value 0"]
impl crate::Resettable for A1Spec {
    const RESET_VALUE: u8 = 0;
}
