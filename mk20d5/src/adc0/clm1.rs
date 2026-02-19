#[doc = "Register `CLM1` reader"]
pub type R = crate::R<Clm1Spec>;
#[doc = "Register `CLM1` writer"]
pub type W = crate::W<Clm1Spec>;
#[doc = "Field `CLM1` reader - Calibration value"]
pub type Clm1R = crate::FieldReader;
#[doc = "Field `CLM1` writer - Calibration value"]
pub type Clm1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn clm1(&self) -> Clm1R {
        Clm1R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn clm1(&mut self) -> Clm1W<Clm1Spec> {
        Clm1W::new(self, 0)
    }
}
#[doc = "ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clm1Spec;
impl crate::RegisterSpec for Clm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clm1::R`](R) reader structure"]
impl crate::Readable for Clm1Spec {}
#[doc = "`write(|w| ..)` method takes [`clm1::W`](W) writer structure"]
impl crate::Writable for Clm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLM1 to value 0x40"]
impl crate::Resettable for Clm1Spec {
    const RESET_VALUE: u32 = 0x40;
}
