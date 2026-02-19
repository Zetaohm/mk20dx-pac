#[doc = "Register `CERQ` writer"]
pub type W = crate::W<CerqSpec>;
#[doc = "Field `CERQ` writer - Clear Enable Request"]
pub type CerqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Clear All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caer {
    #[doc = "0: Clear only the ERQ bit specified in the CERQ field"]
    _0 = 0,
    #[doc = "1: Clear all bits in ERQ"]
    _1 = 1,
}
impl From<Caer> for bool {
    #[inline(always)]
    fn from(variant: Caer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAER` writer - Clear All Enable Requests"]
pub type CaerW<'a, REG> = crate::BitWriter<'a, REG, Caer>;
impl<'a, REG> CaerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear only the ERQ bit specified in the CERQ field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Caer::_0)
    }
    #[doc = "Clear all bits in ERQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Caer::_1)
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
    #[doc = "Bits 0:3 - Clear Enable Request"]
    #[inline(always)]
    #[must_use]
    pub fn cerq(&mut self) -> CerqW<CerqSpec> {
        CerqW::new(self, 0)
    }
    #[doc = "Bit 6 - Clear All Enable Requests"]
    #[inline(always)]
    #[must_use]
    pub fn caer(&mut self) -> CaerW<CerqSpec> {
        CaerW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn nop(&mut self) -> NopW<CerqSpec> {
        NopW::new(self, 7)
    }
}
#[doc = "Clear Enable Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cerq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CerqSpec;
impl crate::RegisterSpec for CerqSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`cerq::W`](W) writer structure"]
impl crate::Writable for CerqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CERQ to value 0"]
impl crate::Resettable for CerqSpec {
    const RESET_VALUE: u8 = 0;
}
