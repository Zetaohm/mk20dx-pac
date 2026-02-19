#[doc = "Register `DFWR` reader"]
pub type R = crate::R<DfwrSpec>;
#[doc = "Register `DFWR` writer"]
pub type W = crate::W<DfwrSpec>;
#[doc = "Field `FILT` reader - Filter Length"]
pub type FiltR = crate::FieldReader;
#[doc = "Field `FILT` writer - Filter Length"]
pub type FiltW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Filter Length"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Filter Length"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FiltW<DfwrSpec> {
        FiltW::new(self, 0)
    }
}
#[doc = "Digital Filter Width Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfwrSpec;
impl crate::RegisterSpec for DfwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfwr::R`](R) reader structure"]
impl crate::Readable for DfwrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfwr::W`](W) writer structure"]
impl crate::Writable for DfwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFWR to value 0"]
impl crate::Resettable for DfwrSpec {
    const RESET_VALUE: u32 = 0;
}
