#[doc = "Register `PSOR` writer"]
pub type W = crate::W<PsorSpec>;
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ptso {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic one."]
    _1 = 1,
}
impl From<Ptso> for u32 {
    #[inline(always)]
    fn from(variant: Ptso) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptso {
    type Ux = u32;
}
impl crate::IsEnum for Ptso {}
#[doc = "Field `PTSO` writer - Port Set Output"]
pub type PtsoW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ptso>;
impl<'a, REG> PtsoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptso::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptso::_1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso(&mut self) -> PtsoW<PsorSpec> {
        PtsoW::new(self, 0)
    }
}
#[doc = "Port Set Output Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psor::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsorSpec;
impl crate::RegisterSpec for PsorSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`psor::W`](W) writer structure"]
impl crate::Writable for PsorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSOR to value 0"]
impl crate::Resettable for PsorSpec {
    const RESET_VALUE: u32 = 0;
}
