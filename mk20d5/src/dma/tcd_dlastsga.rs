#[doc = "Register `TCD%s_DLASTSGA` reader"]
pub type R = crate::R<TcdDlastsgaSpec>;
#[doc = "Register `TCD%s_DLASTSGA` writer"]
pub type W = crate::W<TcdDlastsgaSpec>;
#[doc = "Field `DLASTSGA` reader - no description available"]
pub type DlastsgaR = crate::FieldReader<u32>;
#[doc = "Field `DLASTSGA` writer - no description available"]
pub type DlastsgaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dlastsga(&self) -> DlastsgaR {
        DlastsgaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn dlastsga(&mut self) -> DlastsgaW<TcdDlastsgaSpec> {
        DlastsgaW::new(self, 0)
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_dlastsga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_dlastsga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdDlastsgaSpec;
impl crate::RegisterSpec for TcdDlastsgaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcd_dlastsga::R`](R) reader structure"]
impl crate::Readable for TcdDlastsgaSpec {}
#[doc = "`write(|w| ..)` method takes [`tcd_dlastsga::W`](W) writer structure"]
impl crate::Writable for TcdDlastsgaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCD%s_DLASTSGA to value 0"]
impl crate::Resettable for TcdDlastsgaSpec {
    const RESET_VALUE: u32 = 0;
}
