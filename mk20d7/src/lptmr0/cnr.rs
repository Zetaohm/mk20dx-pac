#[doc = "Register `CNR` reader"]
pub type R = crate::R<CnrSpec>;
#[doc = "Register `CNR` writer"]
pub type W = crate::W<CnrSpec>;
#[doc = "Field `COUNTER` reader - Counter Value"]
pub type CounterR = crate::FieldReader<u16>;
#[doc = "Field `COUNTER` writer - Counter Value"]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<CnrSpec> {
        CounterW::new(self, 0)
    }
}
#[doc = "Low Power Timer Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnrSpec;
impl crate::RegisterSpec for CnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnr::R`](R) reader structure"]
impl crate::Readable for CnrSpec {}
#[doc = "`write(|w| ..)` method takes [`cnr::W`](W) writer structure"]
impl crate::Writable for CnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNR to value 0"]
impl crate::Resettable for CnrSpec {
    const RESET_VALUE: u32 = 0;
}
