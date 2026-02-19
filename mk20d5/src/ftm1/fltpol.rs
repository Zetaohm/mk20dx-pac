#[doc = "Register `FLTPOL` reader"]
pub type R = crate::R<FltpolSpec>;
#[doc = "Register `FLTPOL` writer"]
pub type W = crate::W<FltpolSpec>;
#[doc = "Fault Input 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flt0pol {
    #[doc = "0: The fault input polarity is active high. A one at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A zero at the fault input indicates a fault."]
    _1 = 1,
}
impl From<Flt0pol> for bool {
    #[inline(always)]
    fn from(variant: Flt0pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT0POL` reader - Fault Input 0 Polarity"]
pub type Flt0polR = crate::BitReader<Flt0pol>;
impl Flt0polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flt0pol {
        match self.bits {
            false => Flt0pol::_0,
            true => Flt0pol::_1,
        }
    }
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flt0pol::_0
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flt0pol::_1
    }
}
#[doc = "Field `FLT0POL` writer - Fault Input 0 Polarity"]
pub type Flt0polW<'a, REG> = crate::BitWriter<'a, REG, Flt0pol>;
impl<'a, REG> Flt0polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Flt0pol::_0)
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flt0pol::_1)
    }
}
#[doc = "Fault Input 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flt1pol {
    #[doc = "0: The fault input polarity is active high. A one at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A zero at the fault input indicates a fault."]
    _1 = 1,
}
impl From<Flt1pol> for bool {
    #[inline(always)]
    fn from(variant: Flt1pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1POL` reader - Fault Input 1 Polarity"]
pub type Flt1polR = crate::BitReader<Flt1pol>;
impl Flt1polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flt1pol {
        match self.bits {
            false => Flt1pol::_0,
            true => Flt1pol::_1,
        }
    }
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flt1pol::_0
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flt1pol::_1
    }
}
#[doc = "Field `FLT1POL` writer - Fault Input 1 Polarity"]
pub type Flt1polW<'a, REG> = crate::BitWriter<'a, REG, Flt1pol>;
impl<'a, REG> Flt1polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Flt1pol::_0)
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flt1pol::_1)
    }
}
#[doc = "Fault Input 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flt2pol {
    #[doc = "0: The fault input polarity is active high. A one at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A zero at the fault input indicates a fault."]
    _1 = 1,
}
impl From<Flt2pol> for bool {
    #[inline(always)]
    fn from(variant: Flt2pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT2POL` reader - Fault Input 2 Polarity"]
pub type Flt2polR = crate::BitReader<Flt2pol>;
impl Flt2polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flt2pol {
        match self.bits {
            false => Flt2pol::_0,
            true => Flt2pol::_1,
        }
    }
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flt2pol::_0
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flt2pol::_1
    }
}
#[doc = "Field `FLT2POL` writer - Fault Input 2 Polarity"]
pub type Flt2polW<'a, REG> = crate::BitWriter<'a, REG, Flt2pol>;
impl<'a, REG> Flt2polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Flt2pol::_0)
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flt2pol::_1)
    }
}
#[doc = "Fault Input 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flt3pol {
    #[doc = "0: The fault input polarity is active high. A one at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A zero at the fault input indicates a fault."]
    _1 = 1,
}
impl From<Flt3pol> for bool {
    #[inline(always)]
    fn from(variant: Flt3pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT3POL` reader - Fault Input 3 Polarity"]
pub type Flt3polR = crate::BitReader<Flt3pol>;
impl Flt3polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flt3pol {
        match self.bits {
            false => Flt3pol::_0,
            true => Flt3pol::_1,
        }
    }
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flt3pol::_0
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flt3pol::_1
    }
}
#[doc = "Field `FLT3POL` writer - Fault Input 3 Polarity"]
pub type Flt3polW<'a, REG> = crate::BitWriter<'a, REG, Flt3pol>;
impl<'a, REG> Flt3polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The fault input polarity is active high. A one at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Flt3pol::_0)
    }
    #[doc = "The fault input polarity is active low. A zero at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flt3pol::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    pub fn flt0pol(&self) -> Flt0polR {
        Flt0polR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    pub fn flt1pol(&self) -> Flt1polR {
        Flt1polR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    pub fn flt2pol(&self) -> Flt2polR {
        Flt2polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    pub fn flt3pol(&self) -> Flt3polR {
        Flt3polR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt0pol(&mut self) -> Flt0polW<FltpolSpec> {
        Flt0polW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt1pol(&mut self) -> Flt1polW<FltpolSpec> {
        Flt1polW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt2pol(&mut self) -> Flt2polW<FltpolSpec> {
        Flt2polW::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt3pol(&mut self) -> Flt3polW<FltpolSpec> {
        Flt3polW::new(self, 3)
    }
}
#[doc = "FTM Fault Input Polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`fltpol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltpol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltpolSpec;
impl crate::RegisterSpec for FltpolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltpol::R`](R) reader structure"]
impl crate::Readable for FltpolSpec {}
#[doc = "`write(|w| ..)` method takes [`fltpol::W`](W) writer structure"]
impl crate::Writable for FltpolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTPOL to value 0"]
impl crate::Resettable for FltpolSpec {
    const RESET_VALUE: u32 = 0;
}
