#[doc = "Register `CLP0` reader"]
pub type R = crate::R<Clp0Spec>;
#[doc = "Register `CLP0` writer"]
pub type W = crate::W<Clp0Spec>;
#[doc = "Field `CLP0` reader - Calibration value"]
pub type Clp0R = crate::FieldReader;
#[doc = "Field `CLP0` writer - Calibration value"]
pub type Clp0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Calibration value"]
    #[inline(always)]
    pub fn clp0(&self) -> Clp0R {
        Clp0R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn clp0(&mut self) -> Clp0W<Clp0Spec> {
        Clp0W::new(self, 0)
    }
}
#[doc = "ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clp0Spec;
impl crate::RegisterSpec for Clp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clp0::R`](R) reader structure"]
impl crate::Readable for Clp0Spec {}
#[doc = "`write(|w| ..)` method takes [`clp0::W`](W) writer structure"]
impl crate::Writable for Clp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLP0 to value 0x20"]
impl crate::Resettable for Clp0Spec {
    const RESET_VALUE: u32 = 0x20;
}
