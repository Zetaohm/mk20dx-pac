#[doc = "Register `PTOR` writer"]
pub type W = crate::W<PtorSpec>;
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ptto {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<Ptto> for u32 {
    #[inline(always)]
    fn from(variant: Ptto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptto {
    type Ux = u32;
}
impl crate::IsEnum for Ptto {}
#[doc = "Field `PTTO` writer - Port Toggle Output"]
pub type PttoW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ptto>;
impl<'a, REG> PttoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptto::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptto::_1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto(&mut self) -> PttoW<PtorSpec> {
        PttoW::new(self, 0)
    }
}
#[doc = "Port Toggle Output Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptor::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtorSpec;
impl crate::RegisterSpec for PtorSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ptor::W`](W) writer structure"]
impl crate::Writable for PtorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTOR to value 0"]
impl crate::Resettable for PtorSpec {
    const RESET_VALUE: u32 = 0;
}
