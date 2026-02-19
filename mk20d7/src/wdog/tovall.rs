#[doc = "Register `TOVALL` reader"]
pub type R = crate::R<TovallSpec>;
#[doc = "Register `TOVALL` writer"]
pub type W = crate::W<TovallSpec>;
#[doc = "Field `TOVALLOW` reader - no description available"]
pub type TovallowR = crate::FieldReader<u16>;
#[doc = "Field `TOVALLOW` writer - no description available"]
pub type TovallowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn tovallow(&self) -> TovallowR {
        TovallowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tovallow(&mut self) -> TovallowW<TovallSpec> {
        TovallowW::new(self, 0)
    }
}
#[doc = "Watchdog Time-out Value Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`tovall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tovall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TovallSpec;
impl crate::RegisterSpec for TovallSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tovall::R`](R) reader structure"]
impl crate::Readable for TovallSpec {}
#[doc = "`write(|w| ..)` method takes [`tovall::W`](W) writer structure"]
impl crate::Writable for TovallSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TOVALL to value 0x4b4c"]
impl crate::Resettable for TovallSpec {
    const RESET_VALUE: u16 = 0x4b4c;
}
