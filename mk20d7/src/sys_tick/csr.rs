#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: counter disabled"]
    _0 = 0,
    #[doc = "1: counter enabled"]
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - no description available"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::_0,
            true => Enable::_1,
        }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
#[doc = "Field `ENABLE` writer - no description available"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tickint {
    #[doc = "0: counting down to 0 does not assert the SysTick exception request"]
    _0 = 0,
    #[doc = "1: counting down to 0 asserts the SysTick exception request"]
    _1 = 1,
}
impl From<Tickint> for bool {
    #[inline(always)]
    fn from(variant: Tickint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - no description available"]
pub type TickintR = crate::BitReader<Tickint>;
impl TickintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tickint {
        match self.bits {
            false => Tickint::_0,
            true => Tickint::_1,
        }
    }
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tickint::_0
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tickint::_1
    }
}
#[doc = "Field `TICKINT` writer - no description available"]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG, Tickint>;
impl<'a, REG> TickintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tickint::_0)
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tickint::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksource {
    #[doc = "0: external clock"]
    _0 = 0,
    #[doc = "1: processor clock"]
    _1 = 1,
}
impl From<Clksource> for bool {
    #[inline(always)]
    fn from(variant: Clksource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSOURCE` reader - no description available"]
pub type ClksourceR = crate::BitReader<Clksource>;
impl ClksourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksource {
        match self.bits {
            false => Clksource::_0,
            true => Clksource::_1,
        }
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clksource::_0
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clksource::_1
    }
}
#[doc = "Field `CLKSOURCE` writer - no description available"]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG, Clksource>;
impl<'a, REG> ClksourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksource::_0)
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksource::_1)
    }
}
#[doc = "Field `COUNTFLAG` reader - no description available"]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - no description available"]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TickintW<CsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> ClksourceW<CsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> CountflagW<CsrSpec> {
        CountflagW::new(self, 16)
    }
}
#[doc = "SysTick Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x04"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
