#[doc = "Register `CEEI` writer"]
pub type W = crate::W<CeeiSpec>;
#[doc = "Field `CEEI` writer - Clear Enable Error Interrupt"]
pub type CeeiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Clear All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caee {
    #[doc = "0: Clear only the EEI bit specified in the CEEI field"]
    _0 = 0,
    #[doc = "1: Clear all bits in EEI"]
    _1 = 1,
}
impl From<Caee> for bool {
    #[inline(always)]
    fn from(variant: Caee) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEE` writer - Clear All Enable Error Interrupts"]
pub type CaeeW<'a, REG> = crate::BitWriter<'a, REG, Caee>;
impl<'a, REG> CaeeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear only the EEI bit specified in the CEEI field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Caee::_0)
    }
    #[doc = "Clear all bits in EEI"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Caee::_1)
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
    #[doc = "Bits 0:3 - Clear Enable Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ceei(&mut self) -> CeeiW<CeeiSpec> {
        CeeiW::new(self, 0)
    }
    #[doc = "Bit 6 - Clear All Enable Error Interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn caee(&mut self) -> CaeeW<CeeiSpec> {
        CaeeW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn nop(&mut self) -> NopW<CeeiSpec> {
        NopW::new(self, 7)
    }
}
#[doc = "Clear Enable Error Interrupt Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceei::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CeeiSpec;
impl crate::RegisterSpec for CeeiSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ceei::W`](W) writer structure"]
impl crate::Writable for CeeiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEEI to value 0"]
impl crate::Resettable for CeeiSpec {
    const RESET_VALUE: u8 = 0;
}
