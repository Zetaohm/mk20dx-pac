#[doc = "Register `CLM2` reader"]
pub type R = crate::R<Clm2Spec>;
#[doc = "Register `CLM2` writer"]
pub type W = crate::W<Clm2Spec>;
#[doc = "Field `CLM2` reader - Calibration value"]
pub type Clm2R = crate::FieldReader;
#[doc = "Field `CLM2` writer - Calibration value"]
pub type Clm2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Calibration value"]
    #[inline(always)]
    pub fn clm2(&self) -> Clm2R {
        Clm2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn clm2(&mut self) -> Clm2W<Clm2Spec> {
        Clm2W::new(self, 0)
    }
}
#[doc = "ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clm2Spec;
impl crate::RegisterSpec for Clm2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clm2::R`](R) reader structure"]
impl crate::Readable for Clm2Spec {}
#[doc = "`write(|w| ..)` method takes [`clm2::W`](W) writer structure"]
impl crate::Writable for Clm2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLM2 to value 0x80"]
impl crate::Resettable for Clm2Spec {
    const RESET_VALUE: u32 = 0x80;
}
