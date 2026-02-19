#[doc = "Register `CLP1` reader"]
pub type R = crate::R<Clp1Spec>;
#[doc = "Register `CLP1` writer"]
pub type W = crate::W<Clp1Spec>;
#[doc = "Field `CLP1` reader - Calibration value"]
pub type Clp1R = crate::FieldReader;
#[doc = "Field `CLP1` writer - Calibration value"]
pub type Clp1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn clp1(&self) -> Clp1R {
        Clp1R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn clp1(&mut self) -> Clp1W<Clp1Spec> {
        Clp1W::new(self, 0)
    }
}
#[doc = "ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clp1Spec;
impl crate::RegisterSpec for Clp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clp1::R`](R) reader structure"]
impl crate::Readable for Clp1Spec {}
#[doc = "`write(|w| ..)` method takes [`clp1::W`](W) writer structure"]
impl crate::Writable for Clp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLP1 to value 0x40"]
impl crate::Resettable for Clp1Spec {
    const RESET_VALUE: u32 = 0x40;
}
