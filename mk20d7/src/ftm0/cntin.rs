#[doc = "Register `CNTIN` reader"]
pub type R = crate::R<CntinSpec>;
#[doc = "Register `CNTIN` writer"]
pub type W = crate::W<CntinSpec>;
#[doc = "Field `INIT` reader - no description available"]
pub type InitR = crate::FieldReader<u16>;
#[doc = "Field `INIT` writer - no description available"]
pub type InitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<CntinSpec> {
        InitW::new(self, 0)
    }
}
#[doc = "Counter Initial Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cntin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntinSpec;
impl crate::RegisterSpec for CntinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntin::R`](R) reader structure"]
impl crate::Readable for CntinSpec {}
#[doc = "`write(|w| ..)` method takes [`cntin::W`](W) writer structure"]
impl crate::Writable for CntinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTIN to value 0"]
impl crate::Resettable for CntinSpec {
    const RESET_VALUE: u32 = 0;
}
