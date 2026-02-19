#[doc = "Register `GPCLR` writer"]
pub type W = crate::W<GpclrSpec>;
#[doc = "Field `GPWD` writer - Global Pin Write Data"]
pub type GpwdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GPWE` writer - Global Pin Write Enable"]
pub type GpweW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn gpwd(&mut self) -> GpwdW<GpclrSpec> {
        GpwdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe(&mut self) -> GpweW<GpclrSpec> {
        GpweW::new(self, 16)
    }
}
#[doc = "Global Pin Control Low Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpclrSpec;
impl crate::RegisterSpec for GpclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpclr::W`](W) writer structure"]
impl crate::Writable for GpclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPCLR to value 0"]
impl crate::Resettable for GpclrSpec {
    const RESET_VALUE: u32 = 0;
}
