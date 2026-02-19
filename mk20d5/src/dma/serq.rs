#[doc = "Register `SERQ` writer"]
pub type W = crate::W<SerqSpec>;
#[doc = "Field `SERQ` writer - Set enable request"]
pub type SerqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Set All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saer {
    #[doc = "0: Set only the ERQ bit specified in the SERQ field"]
    _0 = 0,
    #[doc = "1: Set all bits in ERQ"]
    _1 = 1,
}
impl From<Saer> for bool {
    #[inline(always)]
    fn from(variant: Saer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAER` writer - Set All Enable Requests"]
pub type SaerW<'a, REG> = crate::BitWriter<'a, REG, Saer>;
impl<'a, REG> SaerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set only the ERQ bit specified in the SERQ field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saer::_0)
    }
    #[doc = "Set all bits in ERQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saer::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nop {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    _1 = 1,
}
impl From<Nop> for bool {
    #[inline(always)]
    fn from(variant: Nop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOP` writer - no description available"]
pub type NopW<'a, REG> = crate::BitWriter<'a, REG, Nop>;
impl<'a, REG> NopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nop::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nop::_1)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set enable request"]
    #[inline(always)]
    #[must_use]
    pub fn serq(&mut self) -> SerqW<SerqSpec> {
        SerqW::new(self, 0)
    }
    #[doc = "Bit 6 - Set All Enable Requests"]
    #[inline(always)]
    #[must_use]
    pub fn saer(&mut self) -> SaerW<SerqSpec> {
        SaerW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn nop(&mut self) -> NopW<SerqSpec> {
        NopW::new(self, 7)
    }
}
#[doc = "Set Enable Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerqSpec;
impl crate::RegisterSpec for SerqSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`serq::W`](W) writer structure"]
impl crate::Writable for SerqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SERQ to value 0"]
impl crate::Resettable for SerqSpec {
    const RESET_VALUE: u8 = 0;
}
