#[doc = "Register `TIDT` reader"]
pub type R = crate::R<TidtSpec>;
#[doc = "Register `TIDT` writer"]
pub type W = crate::W<TidtSpec>;
#[doc = "Field `TIDT` reader - CEA709.1-B Transmit IDT Register"]
pub type TidtR = crate::FieldReader;
#[doc = "Field `TIDT` writer - CEA709.1-B Transmit IDT Register"]
pub type TidtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CEA709.1-B Transmit IDT Register"]
    #[inline(always)]
    pub fn tidt(&self) -> TidtR {
        TidtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CEA709.1-B Transmit IDT Register"]
    #[inline(always)]
    #[must_use]
    pub fn tidt(&mut self) -> TidtW<TidtSpec> {
        TidtW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Transmit Indeterminate Time\n\nYou can [`read`](crate::Reg::read) this register and get [`tidt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TidtSpec;
impl crate::RegisterSpec for TidtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tidt::R`](R) reader structure"]
impl crate::Readable for TidtSpec {}
#[doc = "`write(|w| ..)` method takes [`tidt::W`](W) writer structure"]
impl crate::Writable for TidtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TIDT to value 0"]
impl crate::Resettable for TidtSpec {
    const RESET_VALUE: u8 = 0;
}
