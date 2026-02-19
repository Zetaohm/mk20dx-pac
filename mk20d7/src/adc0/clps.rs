#[doc = "Register `CLPS` reader"]
pub type R = crate::R<ClpsSpec>;
#[doc = "Register `CLPS` writer"]
pub type W = crate::W<ClpsSpec>;
#[doc = "Field `CLPS` reader - no description available"]
pub type ClpsR = crate::FieldReader;
#[doc = "Field `CLPS` writer - no description available"]
pub type ClpsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - no description available"]
    #[inline(always)]
    pub fn clps(&self) -> ClpsR {
        ClpsR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn clps(&mut self) -> ClpsW<ClpsSpec> {
        ClpsW::new(self, 0)
    }
}
#[doc = "ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClpsSpec;
impl crate::RegisterSpec for ClpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clps::R`](R) reader structure"]
impl crate::Readable for ClpsSpec {}
#[doc = "`write(|w| ..)` method takes [`clps::W`](W) writer structure"]
impl crate::Writable for ClpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLPS to value 0x20"]
impl crate::Resettable for ClpsSpec {
    const RESET_VALUE: u32 = 0x20;
}
