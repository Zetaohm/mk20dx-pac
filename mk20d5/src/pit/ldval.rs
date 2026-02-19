#[doc = "Register `LDVAL%s` reader"]
pub type R = crate::R<LdvalSpec>;
#[doc = "Register `LDVAL%s` writer"]
pub type W = crate::W<LdvalSpec>;
#[doc = "Field `TSV` reader - Timer Start Value Bits"]
pub type TsvR = crate::FieldReader<u32>;
#[doc = "Field `TSV` writer - Timer Start Value Bits"]
pub type TsvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Start Value Bits"]
    #[inline(always)]
    pub fn tsv(&self) -> TsvR {
        TsvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Start Value Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tsv(&mut self) -> TsvW<LdvalSpec> {
        TsvW::new(self, 0)
    }
}
#[doc = "Timer Load Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdvalSpec;
impl crate::RegisterSpec for LdvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldval::R`](R) reader structure"]
impl crate::Readable for LdvalSpec {}
#[doc = "`write(|w| ..)` method takes [`ldval::W`](W) writer structure"]
impl crate::Writable for LdvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDVAL%s to value 0"]
impl crate::Resettable for LdvalSpec {
    const RESET_VALUE: u32 = 0;
}
