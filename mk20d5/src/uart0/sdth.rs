#[doc = "Register `SDTH` reader"]
pub type R = crate::R<SdthSpec>;
#[doc = "Register `SDTH` writer"]
pub type W = crate::W<SdthSpec>;
#[doc = "Field `SDTH` reader - Secondary Delay Timer High"]
pub type SdthR = crate::FieldReader;
#[doc = "Field `SDTH` writer - Secondary Delay Timer High"]
pub type SdthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Secondary Delay Timer High"]
    #[inline(always)]
    pub fn sdth(&self) -> SdthR {
        SdthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secondary Delay Timer High"]
    #[inline(always)]
    #[must_use]
    pub fn sdth(&mut self) -> SdthW<SdthSpec> {
        SdthW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Secondary Delay Timer High\n\nYou can [`read`](crate::Reg::read) this register and get [`sdth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdthSpec;
impl crate::RegisterSpec for SdthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdth::R`](R) reader structure"]
impl crate::Readable for SdthSpec {}
#[doc = "`write(|w| ..)` method takes [`sdth::W`](W) writer structure"]
impl crate::Writable for SdthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDTH to value 0"]
impl crate::Resettable for SdthSpec {
    const RESET_VALUE: u8 = 0;
}
