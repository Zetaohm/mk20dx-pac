#[doc = "Register `CINT` writer"]
pub type W = crate::W<CintSpec>;
#[doc = "Field `CINT` writer - Clear Interrupt Request"]
pub type CintW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Clear All Interrupt Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cair {
    #[doc = "0: Clear only the INT bit specified in the CINT field"]
    _0 = 0,
    #[doc = "1: Clear all bits in INT"]
    _1 = 1,
}
impl From<Cair> for bool {
    #[inline(always)]
    fn from(variant: Cair) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAIR` writer - Clear All Interrupt Requests"]
pub type CairW<'a, REG> = crate::BitWriter<'a, REG, Cair>;
impl<'a, REG> CairW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear only the INT bit specified in the CINT field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cair::_0)
    }
    #[doc = "Clear all bits in INT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cair::_1)
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
    #[doc = "Bits 0:3 - Clear Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CintW<CintSpec> {
        CintW::new(self, 0)
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline(always)]
    #[must_use]
    pub fn cair(&mut self) -> CairW<CintSpec> {
        CairW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn nop(&mut self) -> NopW<CintSpec> {
        NopW::new(self, 7)
    }
}
#[doc = "Clear Interrupt Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cint::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CintSpec;
impl crate::RegisterSpec for CintSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`cint::W`](W) writer structure"]
impl crate::Writable for CintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CINT to value 0"]
impl crate::Resettable for CintSpec {
    const RESET_VALUE: u8 = 0;
}
