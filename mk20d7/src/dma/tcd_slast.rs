#[doc = "Register `TCD%s_SLAST` reader"]
pub type R = crate::R<TcdSlastSpec>;
#[doc = "Register `TCD%s_SLAST` writer"]
pub type W = crate::W<TcdSlastSpec>;
#[doc = "Field `SLAST` reader - Last source Address Adjustment"]
pub type SlastR = crate::FieldReader<u32>;
#[doc = "Field `SLAST` writer - Last source Address Adjustment"]
pub type SlastW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last source Address Adjustment"]
    #[inline(always)]
    pub fn slast(&self) -> SlastR {
        SlastR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last source Address Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn slast(&mut self) -> SlastW<TcdSlastSpec> {
        SlastW::new(self, 0)
    }
}
#[doc = "TCD Last Source Address Adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_slast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_slast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdSlastSpec;
impl crate::RegisterSpec for TcdSlastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcd_slast::R`](R) reader structure"]
impl crate::Readable for TcdSlastSpec {}
#[doc = "`write(|w| ..)` method takes [`tcd_slast::W`](W) writer structure"]
impl crate::Writable for TcdSlastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCD%s_SLAST to value 0"]
impl crate::Resettable for TcdSlastSpec {
    const RESET_VALUE: u32 = 0;
}
