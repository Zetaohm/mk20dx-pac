#[doc = "Register `C8` reader"]
pub type R = crate::R<C8Spec>;
#[doc = "Register `C8` writer"]
pub type W = crate::W<C8Spec>;
#[doc = "RTC Loss of Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locs1 {
    #[doc = "0: Loss of RTC has not occur."]
    _0 = 0,
    #[doc = "1: Loss of RTC has occur"]
    _1 = 1,
}
impl From<Locs1> for bool {
    #[inline(always)]
    fn from(variant: Locs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCS1` reader - RTC Loss of Clock Status"]
pub type Locs1R = crate::BitReader<Locs1>;
impl Locs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locs1 {
        match self.bits {
            false => Locs1::_0,
            true => Locs1::_1,
        }
    }
    #[doc = "Loss of RTC has not occur."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Locs1::_0
    }
    #[doc = "Loss of RTC has occur"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Locs1::_1
    }
}
#[doc = "Field `LOCS1` writer - RTC Loss of Clock Status"]
pub type Locs1W<'a, REG> = crate::BitWriter<'a, REG, Locs1>;
impl<'a, REG> Locs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loss of RTC has not occur."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Locs1::_0)
    }
    #[doc = "Loss of RTC has occur"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Locs1::_1)
    }
}
#[doc = "Clock Monitor Enable1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cme1 {
    #[doc = "0: External clock monitor is disabled for RTC clock."]
    _0 = 0,
    #[doc = "1: External clock monitor is enabled for RTC clock."]
    _1 = 1,
}
impl From<Cme1> for bool {
    #[inline(always)]
    fn from(variant: Cme1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CME1` reader - Clock Monitor Enable1"]
pub type Cme1R = crate::BitReader<Cme1>;
impl Cme1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cme1 {
        match self.bits {
            false => Cme1::_0,
            true => Cme1::_1,
        }
    }
    #[doc = "External clock monitor is disabled for RTC clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cme1::_0
    }
    #[doc = "External clock monitor is enabled for RTC clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cme1::_1
    }
}
#[doc = "Field `CME1` writer - Clock Monitor Enable1"]
pub type Cme1W<'a, REG> = crate::BitWriter<'a, REG, Cme1>;
impl<'a, REG> Cme1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock monitor is disabled for RTC clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cme1::_0)
    }
    #[doc = "External clock monitor is enabled for RTC clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cme1::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lolre {
    #[doc = "0: Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a PLL loss of lock indication."]
    _1 = 1,
}
impl From<Lolre> for bool {
    #[inline(always)]
    fn from(variant: Lolre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOLRE` reader - no description available"]
pub type LolreR = crate::BitReader<Lolre>;
impl LolreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lolre {
        match self.bits {
            false => Lolre::_0,
            true => Lolre::_1,
        }
    }
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lolre::_0
    }
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lolre::_1
    }
}
#[doc = "Field `LOLRE` writer - no description available"]
pub type LolreW<'a, REG> = crate::BitWriter<'a, REG, Lolre>;
impl<'a, REG> LolreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lolre::_0)
    }
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lolre::_1)
    }
}
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locre1 {
    #[doc = "0: Interrupt request is generated on a loss of RTC external reference clock."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a loss of RTC external reference clock"]
    _1 = 1,
}
impl From<Locre1> for bool {
    #[inline(always)]
    fn from(variant: Locre1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCRE1` reader - Loss of Clock Reset Enable"]
pub type Locre1R = crate::BitReader<Locre1>;
impl Locre1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locre1 {
        match self.bits {
            false => Locre1::_0,
            true => Locre1::_1,
        }
    }
    #[doc = "Interrupt request is generated on a loss of RTC external reference clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Locre1::_0
    }
    #[doc = "Generate a reset request on a loss of RTC external reference clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Locre1::_1
    }
}
#[doc = "Field `LOCRE1` writer - Loss of Clock Reset Enable"]
pub type Locre1W<'a, REG> = crate::BitWriter<'a, REG, Locre1>;
impl<'a, REG> Locre1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request is generated on a loss of RTC external reference clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Locre1::_0)
    }
    #[doc = "Generate a reset request on a loss of RTC external reference clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Locre1::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline(always)]
    pub fn locs1(&self) -> Locs1R {
        Locs1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline(always)]
    pub fn cme1(&self) -> Cme1R {
        Cme1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn lolre(&self) -> LolreR {
        LolreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre1(&self) -> Locre1R {
        Locre1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline(always)]
    #[must_use]
    pub fn locs1(&mut self) -> Locs1W<C8Spec> {
        Locs1W::new(self, 0)
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline(always)]
    #[must_use]
    pub fn cme1(&mut self) -> Cme1W<C8Spec> {
        Cme1W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn lolre(&mut self) -> LolreW<C8Spec> {
        LolreW::new(self, 6)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locre1(&mut self) -> Locre1W<C8Spec> {
        Locre1W::new(self, 7)
    }
}
#[doc = "MCG Control 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C8Spec;
impl crate::RegisterSpec for C8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c8::R`](R) reader structure"]
impl crate::Readable for C8Spec {}
#[doc = "`write(|w| ..)` method takes [`c8::W`](W) writer structure"]
impl crate::Writable for C8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C8 to value 0x80"]
impl crate::Resettable for C8Spec {
    const RESET_VALUE: u8 = 0x80;
}
