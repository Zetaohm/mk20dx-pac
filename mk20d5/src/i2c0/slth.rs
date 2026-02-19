#[doc = "Register `SLTH` reader"]
pub type R = crate::R<SlthSpec>;
#[doc = "Register `SLTH` writer"]
pub type W = crate::W<SlthSpec>;
#[doc = "Field `SSLT` reader - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
pub type SsltR = crate::FieldReader;
#[doc = "Field `SSLT` writer - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
pub type SsltW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
    #[inline(always)]
    pub fn sslt(&self) -> SsltR {
        SsltR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
    #[inline(always)]
    #[must_use]
    pub fn sslt(&mut self) -> SsltW<SlthSpec> {
        SsltW::new(self, 0)
    }
}
#[doc = "I2C SCL Low Timeout Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`slth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlthSpec;
impl crate::RegisterSpec for SlthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`slth::R`](R) reader structure"]
impl crate::Readable for SlthSpec {}
#[doc = "`write(|w| ..)` method takes [`slth::W`](W) writer structure"]
impl crate::Writable for SlthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SLTH to value 0"]
impl crate::Resettable for SlthSpec {
    const RESET_VALUE: u8 = 0;
}
