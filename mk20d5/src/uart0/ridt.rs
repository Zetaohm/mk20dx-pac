#[doc = "Register `RIDT` reader"]
pub type R = crate::R<RidtSpec>;
#[doc = "Register `RIDT` writer"]
pub type W = crate::W<RidtSpec>;
#[doc = "Field `RIDT` reader - CEA709.1-B Receive IDT Register"]
pub type RidtR = crate::FieldReader;
#[doc = "Field `RIDT` writer - CEA709.1-B Receive IDT Register"]
pub type RidtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CEA709.1-B Receive IDT Register"]
    #[inline(always)]
    pub fn ridt(&self) -> RidtR {
        RidtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CEA709.1-B Receive IDT Register"]
    #[inline(always)]
    #[must_use]
    pub fn ridt(&mut self) -> RidtW<RidtSpec> {
        RidtW::new(self, 0)
    }
}
#[doc = "UART CEA709.1-B Receive Indeterminate Time\n\nYou can [`read`](crate::Reg::read) this register and get [`ridt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ridt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RidtSpec;
impl crate::RegisterSpec for RidtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ridt::R`](R) reader structure"]
impl crate::Readable for RidtSpec {}
#[doc = "`write(|w| ..)` method takes [`ridt::W`](W) writer structure"]
impl crate::Writable for RidtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RIDT to value 0"]
impl crate::Resettable for RidtSpec {
    const RESET_VALUE: u8 = 0;
}
