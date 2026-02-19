#[doc = "Register `CLM4` reader"]
pub type R = crate::R<Clm4Spec>;
#[doc = "Register `CLM4` writer"]
pub type W = crate::W<Clm4Spec>;
#[doc = "Field `CLM4` reader - Calibration value"]
pub type Clm4R = crate::FieldReader<u16>;
#[doc = "Field `CLM4` writer - Calibration value"]
pub type Clm4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Calibration value"]
    #[inline(always)]
    pub fn clm4(&self) -> Clm4R {
        Clm4R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn clm4(&mut self) -> Clm4W<Clm4Spec> {
        Clm4W::new(self, 0)
    }
}
#[doc = "ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clm4Spec;
impl crate::RegisterSpec for Clm4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clm4::R`](R) reader structure"]
impl crate::Readable for Clm4Spec {}
#[doc = "`write(|w| ..)` method takes [`clm4::W`](W) writer structure"]
impl crate::Writable for Clm4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLM4 to value 0x0200"]
impl crate::Resettable for Clm4Spec {
    const RESET_VALUE: u32 = 0x0200;
}
