#[doc = "Register `CLM0` reader"]
pub type R = crate::R<Clm0Spec>;
#[doc = "Register `CLM0` writer"]
pub type W = crate::W<Clm0Spec>;
#[doc = "Field `CLM0` reader - Calibration value"]
pub type Clm0R = crate::FieldReader;
#[doc = "Field `CLM0` writer - Calibration value"]
pub type Clm0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Calibration value"]
    #[inline(always)]
    pub fn clm0(&self) -> Clm0R {
        Clm0R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn clm0(&mut self) -> Clm0W<Clm0Spec> {
        Clm0W::new(self, 0)
    }
}
#[doc = "ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clm0Spec;
impl crate::RegisterSpec for Clm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clm0::R`](R) reader structure"]
impl crate::Readable for Clm0Spec {}
#[doc = "`write(|w| ..)` method takes [`clm0::W`](W) writer structure"]
impl crate::Writable for Clm0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLM0 to value 0x20"]
impl crate::Resettable for Clm0Spec {
    const RESET_VALUE: u32 = 0x20;
}
