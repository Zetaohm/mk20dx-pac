#[doc = "Register `S` reader"]
pub type R = crate::R<SSpec>;
#[doc = "Register `S` writer"]
pub type W = crate::W<SSpec>;
#[doc = "Internal Reference Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ircst {
    #[doc = "0: Source of internal reference clock is the slow clock (32 kHz IRC)."]
    _0 = 0,
    #[doc = "1: Source of internal reference clock is the fast clock (2 MHz IRC)."]
    _1 = 1,
}
impl From<Ircst> for bool {
    #[inline(always)]
    fn from(variant: Ircst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCST` reader - Internal Reference Clock Status"]
pub type IrcstR = crate::BitReader<Ircst>;
impl IrcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ircst {
        match self.bits {
            false => Ircst::_0,
            true => Ircst::_1,
        }
    }
    #[doc = "Source of internal reference clock is the slow clock (32 kHz IRC)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ircst::_0
    }
    #[doc = "Source of internal reference clock is the fast clock (2 MHz IRC)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ircst::_1
    }
}
#[doc = "Field `OSCINIT0` reader - OSC Initialization"]
pub type Oscinit0R = crate::BitReader;
#[doc = "Clock Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkst {
    #[doc = "0: Encoding 0 - Output of the FLL is selected (reset default)."]
    _00 = 0,
    #[doc = "1: Encoding 1 - Internal reference clock is selected."]
    _01 = 1,
    #[doc = "2: Encoding 2 - External reference clock is selected."]
    _10 = 2,
    #[doc = "3: Encoding 3 - Output of the PLL is selected."]
    _11 = 3,
}
impl From<Clkst> for u8 {
    #[inline(always)]
    fn from(variant: Clkst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkst {
    type Ux = u8;
}
impl crate::IsEnum for Clkst {}
#[doc = "Field `CLKST` reader - Clock Mode Status"]
pub type ClkstR = crate::FieldReader<Clkst>;
impl ClkstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkst {
        match self.bits {
            0 => Clkst::_00,
            1 => Clkst::_01,
            2 => Clkst::_10,
            3 => Clkst::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Encoding 0 - Output of the FLL is selected (reset default)."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Clkst::_00
    }
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Clkst::_01
    }
    #[doc = "Encoding 2 - External reference clock is selected."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Clkst::_10
    }
    #[doc = "Encoding 3 - Output of the PLL is selected."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Clkst::_11
    }
}
#[doc = "Internal Reference Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefst {
    #[doc = "0: Source of FLL reference clock is the external reference clock."]
    _0 = 0,
    #[doc = "1: Source of FLL reference clock is the internal reference clock."]
    _1 = 1,
}
impl From<Irefst> for bool {
    #[inline(always)]
    fn from(variant: Irefst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFST` reader - Internal Reference Status"]
pub type IrefstR = crate::BitReader<Irefst>;
impl IrefstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefst {
        match self.bits {
            false => Irefst::_0,
            true => Irefst::_1,
        }
    }
    #[doc = "Source of FLL reference clock is the external reference clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irefst::_0
    }
    #[doc = "Source of FLL reference clock is the internal reference clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irefst::_1
    }
}
#[doc = "PLL Select Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllst {
    #[doc = "0: Source of PLLS clock is FLL clock."]
    _0 = 0,
    #[doc = "1: Source of PLLS clock is PLL clock."]
    _1 = 1,
}
impl From<Pllst> for bool {
    #[inline(always)]
    fn from(variant: Pllst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLST` reader - PLL Select Status"]
pub type PllstR = crate::BitReader<Pllst>;
impl PllstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllst {
        match self.bits {
            false => Pllst::_0,
            true => Pllst::_1,
        }
    }
    #[doc = "Source of PLLS clock is FLL clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllst::_0
    }
    #[doc = "Source of PLLS clock is PLL clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllst::_1
    }
}
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock0 {
    #[doc = "0: PLL is currently unlocked."]
    _0 = 0,
    #[doc = "1: PLL is currently locked."]
    _1 = 1,
}
impl From<Lock0> for bool {
    #[inline(always)]
    fn from(variant: Lock0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK0` reader - Lock Status"]
pub type Lock0R = crate::BitReader<Lock0>;
impl Lock0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock0 {
        match self.bits {
            false => Lock0::_0,
            true => Lock0::_1,
        }
    }
    #[doc = "PLL is currently unlocked."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lock0::_0
    }
    #[doc = "PLL is currently locked."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lock0::_1
    }
}
#[doc = "Loss of Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lols0 {
    #[doc = "0: PLL has not lost lock since LOLS 0 was last cleared."]
    _0 = 0,
    #[doc = "1: PLL has lost lock since LOLS 0 was last cleared."]
    _1 = 1,
}
impl From<Lols0> for bool {
    #[inline(always)]
    fn from(variant: Lols0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOLS0` reader - Loss of Lock Status"]
pub type Lols0R = crate::BitReader<Lols0>;
impl Lols0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lols0 {
        match self.bits {
            false => Lols0::_0,
            true => Lols0::_1,
        }
    }
    #[doc = "PLL has not lost lock since LOLS 0 was last cleared."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lols0::_0
    }
    #[doc = "PLL has lost lock since LOLS 0 was last cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lols0::_1
    }
}
#[doc = "Field `LOLS0` writer - Loss of Lock Status"]
pub type Lols0W<'a, REG> = crate::BitWriter<'a, REG, Lols0>;
impl<'a, REG> Lols0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL has not lost lock since LOLS 0 was last cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lols0::_0)
    }
    #[doc = "PLL has lost lock since LOLS 0 was last cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lols0::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Clock Status"]
    #[inline(always)]
    pub fn ircst(&self) -> IrcstR {
        IrcstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OSC Initialization"]
    #[inline(always)]
    pub fn oscinit0(&self) -> Oscinit0R {
        Oscinit0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline(always)]
    pub fn clkst(&self) -> ClkstR {
        ClkstR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline(always)]
    pub fn irefst(&self) -> IrefstR {
        IrefstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL Select Status"]
    #[inline(always)]
    pub fn pllst(&self) -> PllstR {
        PllstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Status"]
    #[inline(always)]
    pub fn lock0(&self) -> Lock0R {
        Lock0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols0(&self) -> Lols0R {
        Lols0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    #[must_use]
    pub fn lols0(&mut self) -> Lols0W<SSpec> {
        Lols0W::new(self, 7)
    }
}
#[doc = "MCG Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSpec;
impl crate::RegisterSpec for SSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s::R`](R) reader structure"]
impl crate::Readable for SSpec {}
#[doc = "`write(|w| ..)` method takes [`s::W`](W) writer structure"]
impl crate::Writable for SSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S to value 0x10"]
impl crate::Resettable for SSpec {
    const RESET_VALUE: u8 = 0x10;
}
