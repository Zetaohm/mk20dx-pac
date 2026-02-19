#[doc = "Register `TOVALH` reader"]
pub type R = crate::R<TovalhSpec>;
#[doc = "Register `TOVALH` writer"]
pub type W = crate::W<TovalhSpec>;
#[doc = "Field `TOVALHIGH` reader - no description available"]
pub type TovalhighR = crate::FieldReader<u16>;
#[doc = "Field `TOVALHIGH` writer - no description available"]
pub type TovalhighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn tovalhigh(&self) -> TovalhighR {
        TovalhighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tovalhigh(&mut self) -> TovalhighW<TovalhSpec> {
        TovalhighW::new(self, 0)
    }
}
#[doc = "Watchdog Time-out Value Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`tovalh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tovalh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TovalhSpec;
impl crate::RegisterSpec for TovalhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tovalh::R`](R) reader structure"]
impl crate::Readable for TovalhSpec {}
#[doc = "`write(|w| ..)` method takes [`tovalh::W`](W) writer structure"]
impl crate::Writable for TovalhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TOVALH to value 0x4c"]
impl crate::Resettable for TovalhSpec {
    const RESET_VALUE: u16 = 0x4c;
}
